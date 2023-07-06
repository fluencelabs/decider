use ethcontract::futures::future::BoxFuture;
use ethcontract::jsonrpc::{Call, Output};
use ethcontract::web3::{RequestId, Transport};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use eyre::Result;
use serde_json::json;
use serde_json::Value;
use url::Url;

use crate::curl::curl_request;
use crate::curl_request;

pub type FutResult = BoxFuture<'static, ethcontract::web3::error::Result<Value>>;

#[derive(Debug, Clone)]
pub struct CurlTransport {
    pub uri: Url,
    id: Arc<AtomicUsize>,
}

impl CurlTransport {
    pub fn new(uri: &str) -> Result<Self> {
        try {
            Self {
                uri: Url::parse(uri)?,
                id: Arc::new(AtomicUsize::new(0)),
            }
        }
    }

    pub fn next_id(&self) -> RequestId {
        self.id.fetch_add(1, Ordering::AcqRel)
    }
}

impl Transport for CurlTransport {
    type Out = FutResult;

    fn prepare(&self, method: &str, params: Vec<Value>) -> (RequestId, Call) {
        let id = self.next_id();
        let request = web3::helpers::build_request(id, method, params.clone());
        (id, request)
    }

    fn send(&self, _: RequestId, call: Call) -> Self::Out {
        if let Call::MethodCall(call) = call {
            /*
            curl --request POST \
                 --url $uri \
                 --header 'accept: application/json' \
                 --header 'content-type: application/json' \
                 --data '
            {
                 "id": 1,
                 "jsonrpc": "2.0",
                 "method": "eth_accounts"
            }
            '
            */
            let uri = self.uri.clone();
            Box::pin(async move {
                let json = json!(call).to_string();
                let args = vec![
                    "--request",
                    "POST",
                    "--url",
                    &uri,
                    "--header",
                    "accept: application/json",
                    "--header",
                    "content-type: application/json",
                    "--data",
                    json.as_str(),
                ];
                let args = args.into_iter().map(|s| s.to_string()).collect();
                let response = curl_request(args);

                // FIX: if there's a bad uri, the panic kicks in here.

                if response.ret_code != 0 {
                    let stdout = String::from_utf8_lossy(&response.stdout);
                    let error = if response.error.is_empty() {
                        stdout.to_string()
                    } else {
                        format!("error: {}\nstdout: {stdout}", response.error)
                    };
                    return Err(web3::error::Error::Transport(
                        web3::error::TransportError::Message(format!("error: {}", error)),
                    ));
                }

                let response = serde_json::from_slice(response.stdout.as_slice())?;
                let response: Output = serde_json::from_value(response)?;
                let result = match response {
                    Output::Success(jsonrpc_core::types::Success { result, .. }) => result,

                    // no sure if that's enough vs the complete jsonrpc error msg
                    Output::Failure(jsonrpc_core::types::Failure { error, .. }) => {
                        serde_json::to_value(error.message).unwrap()
                    }
                };

                Ok(result)
            })
        } else {
            std::future::ready(web3::Error::InvalidResponse(
                "Only method calls are supported".into(),
            ))
        }
    }
}
