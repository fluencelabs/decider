use eyre::WrapErr;
use hyper::body::Buf;
use hyper::service::{make_service_fn, service_fn};
use hyper::{body, Body, Request, Response};
use serde::Deserialize;
use serde_json::{json, Value};
use std::convert::Infallible;
use std::future::Future;
use std::net::SocketAddr;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc::{channel, unbounded_channel, Sender, UnboundedReceiver, UnboundedSender};
use tokio::sync::Mutex;
use tokio::task;
use tokio::time::timeout;

pub type RpcMethod = String;
pub type RpcParams = Vec<Value>;

type RpcResult = Value;
type RpcError = Value;

#[derive(Clone)]
pub struct ServerChannels {
    recv: Arc<Mutex<UnboundedReceiver<(RpcMethod, RpcParams)>>>,
    send: Arc<UnboundedSender<Result<RpcResult, RpcError>>>,
}

impl ServerChannels {
    pub async fn receive_request(&mut self) -> Option<(RpcMethod, RpcParams)> {
        // need to move out to the top level?
        timeout(Duration::from_secs(60), self.recv.lock().await.recv())
            .await
            .ok()
            .flatten()
    }

    pub fn send_response(&mut self, response: Result<RpcResult, RpcError>) {
        self.send.send(response).unwrap();
    }

    pub fn view(&self) -> ServerChannels {
        self.clone()
    }
}

pub struct ServerHandle {
    pub url: String,
    shutdown: Sender<()>,
    shutdown_confirm: Option<task::JoinHandle<()>>,
    channels: ServerChannels,
}

impl ServerHandle {
    pub async fn shutdown(mut self) {
        self.shutdown.try_send(()).unwrap();
        self.shutdown_confirm.take().unwrap().await.unwrap();
    }
}

impl Deref for ServerHandle {
    type Target = ServerChannels;

    fn deref(&self) -> &Self::Target {
        &self.channels
    }
}

impl DerefMut for ServerHandle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.channels
    }
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct JrpcReq {
    jsonrpc: String,
    id: u32,
    method: RpcMethod,
    #[serde(default)]
    params: RpcParams,
}

async fn process_request(
    send_req: &UnboundedSender<(RpcMethod, RpcParams)>,
    recv_resp: &Arc<Mutex<UnboundedReceiver<Result<RpcResult, RpcError>>>>,
    req: JrpcReq,
) -> Value {
    send_req
        .send((req.method, req.params))
        .wrap_err("send request")
        .unwrap();
    let result = recv_resp.lock().await.recv().await.unwrap();
    match result {
        Ok(value) => json!({
                "jsonrpc": "2.0",
                "id": req.id,
                "result": value,
        }),
        Err(error_value) => json!({
            "jsonrpc": "2.0",
            "id": req.id,
            "error": error_value,
        }),
    }
}

pub fn run_test_server() -> ServerHandle {
    let (send_req, recv_req) = unbounded_channel();
    let (send_resp, recv_resp) = unbounded_channel::<Result<Value, Value>>();
    let recv_resp = Arc::new(Mutex::new(recv_resp));

    let process_http_request =
        async move |mut req: Request<Body>| -> Result<Response<Body>, Infallible> {
            let raw_body = req.body_mut();
            let mut buf = body::aggregate(raw_body).await.unwrap();
            let body = buf.copy_to_bytes(buf.remaining()).to_vec();
            let raw_request = serde_json::from_slice::<Value>(&body).unwrap();

            let response = if raw_request.is_array() {
                let reqs = serde_json::from_value::<Vec<JrpcReq>>(raw_request).unwrap();
                let mut results = Vec::new();
                for (idx, req) in reqs.into_iter().enumerate() {
                    assert_eq!(req.jsonrpc, "2.0", "wrong jsonrpc version: {}", req.jsonrpc);
                    assert_eq!(
                        req.id, idx as u32,
                        "wrong jsonrpc id: {}, req: {:?}",
                        req.id, req
                    );
                    let result = process_request(&send_req, &recv_resp, req).await;
                    results.push(result);
                }
                json!(results)
            } else {
                let req = serde_json::from_value::<JrpcReq>(raw_request).unwrap();
                assert_eq!(req.jsonrpc, "2.0", "wrong jsonrpc version: {}", req.jsonrpc);

                process_request(&send_req, &recv_resp, req).await
            };
            let response_body: Vec<u8> = serde_json::to_string(&response).unwrap().into();
            Ok::<Response<Body>, Infallible>(Response::new(Body::from(response_body)))
        };
    let address = SocketAddr::from(([127, 0, 0, 1], 0));
    let make_service = make_service_fn(move |_conn| {
        let process_http_request = process_http_request.clone();
        let service = service_fn(move |req| {
            let process_http_request = process_http_request.clone();
            process_http_request(req)
        });
        async move { Ok::<_, Infallible>(service) }
    });
    let server = hyper::server::Server::bind(&address).serve(make_service);
    let address = server.local_addr();
    let (shutdown_send, mut shutdown_receive) = channel(1);
    let graceful = server.with_graceful_shutdown((async move || {
        shutdown_receive.recv().await.unwrap();
    })());
    let handle = tokio::spawn((async move || {
        graceful.await.unwrap();
    })());
    let hdl = ServerHandle {
        url: format!("http://{}/", address),
        shutdown: shutdown_send,
        shutdown_confirm: Some(handle),
        channels: ServerChannels {
            recv: Arc::new(Mutex::new(recv_req)),
            send: Arc::new(send_resp),
        },
    };

    hdl
}

pub fn run_test_server_predefined<T, S>(handle: T) -> ServerHandle
where
    T: Fn(String, Vec<Value>) -> S + Send + Sync + 'static,
    S: Future<Output = Value> + Send + Sync,
{
    let server = run_test_server();
    let mut view = server.view();
    tokio::task::spawn((async move || loop {
        if let Some((method, params)) = view.receive_request().await {
            let response = handle(method, params).await;
            view.send_response(Ok(response));
        } else {
            break;
        }
    })());
    server
}
