#![feature(async_closure)]

use eyre::WrapErr;
use hyper::body::Buf;
use hyper::service::{make_service_fn, service_fn};
use hyper::{body, Body, Request, Response};
use serde::Deserialize;
use serde_json::{json, Value};
use std::convert::Infallible;
use std::future::Future;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::mpsc::{channel, unbounded_channel, Sender, UnboundedReceiver, UnboundedSender};
use tokio::sync::Mutex;
use tokio::task;

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct JrpcReq {
    jsonrpc: String,
    id: u32,
    method: String,
    params: Vec<Value>,
}

pub struct ServerHandle {
    pub url: String,
    shutdown: Sender<()>,
    shutdown_confirm: Option<task::JoinHandle<()>>,
}

impl ServerHandle {
    pub async fn shutdown(mut self) {
        self.shutdown.try_send(()).unwrap();
        self.shutdown_confirm.take().unwrap().await.unwrap();
    }
}

pub fn run_test_server() -> (
    ServerHandle,
    UnboundedReceiver<(String, Vec<Value>)>,
    UnboundedSender<Value>,
) {
    let (send_req, recv_req) = unbounded_channel();
    let (send_resp, recv_resp) = unbounded_channel();
    let recv_resp = Arc::new(Mutex::new(recv_resp));

    let process_http_request =
        async move |mut req: Request<Body>| -> Result<Response<Body>, Infallible> {
            let raw_body = req.body_mut();
            let mut buf = body::aggregate(raw_body).await.unwrap();
            let body = buf.copy_to_bytes(buf.remaining()).to_vec();
            let raw_request = serde_json::from_slice::<Value>(&body).unwrap();
            /*
            if raw_request.is_array() {
                // if it's a batch request
                let batch_req = serde_json::from_value::<Vec<JrpcReq>>(raw_request)?;
                let mut expect_id = 0;
                let mut result = Vec::new();
                for req in batch_req {
                    let result = Self::check_request(&req, expect_id);
                    if result.is_ok() {
                        result.push(handler(req.method, req.params));
                    }
                    expect_id += 1;
                }

                // do smth with result
            } else {

             */
            let req = serde_json::from_value::<JrpcReq>(raw_request).unwrap();
            assert_eq!(req.jsonrpc, "2.0", "wrong jsonrpc version: {}", req.jsonrpc);
            assert_eq!(req.id, 0, "wrong jsonrpc id: {}", req.id);

            //let handler = handler.clone();
            send_req
                .send((req.method, req.params))
                .wrap_err("send request")
                .unwrap();
            let result = recv_resp.lock().await.recv().await.unwrap();

            //let result = handler(req.method, req.params).await;
            let response_body: Vec<u8> = serde_json::to_string(&json!({
                    "jsonrpc": "2.0",
                    "id": 0,
                    "result": result,
            }))
            .unwrap()
            .into();
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
    };

    (hdl, recv_req, send_resp)
}

pub fn run_test_server_predefined<T, S>(handle: T) -> ServerHandle
where
    T: Fn(String, Vec<Value>) -> S + Send + Sync + 'static,
    S: Future<Output = Value> + Send + Sync,
{
    let (server, mut recv_req, send_resp) = run_test_server();
    tokio::task::spawn((async move || loop {
        if let Some((method, params)) = recv_req.recv().await {
            let response = handle(method, params).await;
            send_resp.send(response).unwrap();
        } else {
            break;
        }
    })());
    server
}
