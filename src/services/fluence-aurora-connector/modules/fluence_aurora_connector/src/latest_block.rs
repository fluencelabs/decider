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

    let result = match send_jsonrpc::<_, String>(api_endpoint, BlockNumberReq::new()) {
        Err(err) => {
            log::debug!(target: "connector", "request error: {:?}", err);
            return BlockNumberResult::error(err.to_string());
        }
        Ok(result) => result,
    };
    log::debug!(target: "connector", "request result: {:?}", result);
    let result = match result.get_result() {
        Err(err) => {
            return BlockNumberResult::error(err.to_string());
        }
        Ok(result) => result,
    };

    let hex_num = result.trim_start_matches("0x");
    if u64::from_str_radix(&hex_num, 16).is_err() {
        log::debug!(target: "connector", "{:?} isn't a hex number", result);
        return BlockNumberResult::error(format!(
            "can't parse a block: {:?} isn't a hex number",
            result
        ));
    }
    BlockNumberResult::ok(result)
}
