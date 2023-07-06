#![allow(dead_code)]
#![feature(assert_matches)]
#![feature(try_blocks)]

use marine_rs_sdk::module_manifest;
use marine_rs_sdk::WasmLoggerBuilder;
use thiserror::Error;

use crate::jsonrpc::request::RequestError;
use crate::jsonrpc::JsonRpcError;

mod chain;
mod config;
mod curl;
mod hex;
mod jsonrpc;
mod latest_block;

module_manifest!();

#[derive(Debug, Error)]
enum Error {
    #[error(transparent)]
    RequestError(#[from] RequestError),
    #[error(transparent)]
    JsonRpcError(#[from] JsonRpcError),
}

pub fn main() {
    WasmLoggerBuilder::new()
        .with_log_level(log::LevelFilter::Trace)
        .build()
        .unwrap();
}
