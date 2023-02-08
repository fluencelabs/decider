// #![allow(improper_ctypes)]
use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;

use marine_rs_sdk::MountedBinaryResult;

module_manifest!();

pub fn main() {}

#[marine]
pub fn curl_request(cmd: Vec<String>) -> MountedBinaryResult {
    curl(cmd)
}

#[marine]
#[link(wasm_import_module = "host")]
extern "C" {
    fn curl(cmd: Vec<String>) -> MountedBinaryResult;
}
