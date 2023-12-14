use crate::hex_u32_deserialize;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogsReq {
    pub address: String,
    #[serde(deserialize_with = "hex_u32_deserialize")]
    pub from_block: u32,
    #[serde(deserialize_with = "hex_u32_deserialize")]
    pub to_block: u32,
    pub topics: Vec<String>,
}

pub fn filter_logs<'a, T>(blocks: &'a [(u32, T)], req: &LogsReq) -> Vec<&'a (u32, T)> {
    blocks
        .iter()
        .filter(|(block_number, _)| {
            *block_number >= req.from_block && *block_number <= req.to_block
        })
        .collect::<_>()
