use marine_rs_sdk::marine;

use crate::curl::send_jsonrpc;
use crate::hex::hex_to_int;
use crate::jsonrpc::block_number::{BlockNumberReq, BlockNumberResult};
use crate::jsonrpc::request::check_url;

#[marine]
pub fn latest_block_number(api_endpoint: String) -> BlockNumberResult {
    if let Err(err) = check_url(&api_endpoint) {
        return BlockNumberResult::error(err.to_string());
    }

    let response = match send_jsonrpc::<_, String>(api_endpoint, BlockNumberReq::new()) {
        Err(err) => {
            log::debug!(target: "connector", "latest_block_number request error: {:?}", err);
            return BlockNumberResult::error(err.to_string());
        }
        Ok(result) => result,
    };
    log::debug!(target: "connector", "latest_block_number response: {:?}", response);
    let block_number = match response.get_result() {
        Err(err) => {
            return BlockNumberResult::error(err.to_string());
        }
        Ok(result) => result,
    };

    if hex_to_int(&block_number).is_some() {
        BlockNumberResult::ok(block_number)
    } else {
        BlockNumberResult::error(format!(
            "can't parse a block number {}: {:?} isn't a hex number",
            block_number, block_number
        ))
    }
}
