use crate::chain::chain_data::ChainData;
use crate::chain::chain_event::ChainEvent;
use crate::hex::{hex_to_int, int_to_hex};
use crate::jsonrpc::get_logs::GetLogsResp;

pub fn parse_logs<U: ChainData, T: ChainEvent<U>>(logs: Vec<GetLogsResp>) -> Vec<T> {
    logs.into_iter()
        .filter(|deal| !deal.removed)
        .filter_map(|deal| parse_log::<U, T>(deal))
        .collect()
}

pub fn parse_log<U: ChainData, T: ChainEvent<U>>(deal: GetLogsResp) -> Option<T> {
    log::debug!("Parse block {:?}", deal.block_number);
    match U::parse(&deal.data) {
        Err(err) => {
            // Here we ignore blocks we cannot parse.
            // Is it okay? We can't send warning
            log::warn!(target: "connector",
                "Cannot parse data of chain from block {}: {:?}",
                deal.block_number,
                err.to_string()
            );
            None
        }
        Ok(data) => {
            let block_number = hex_to_int(&deal.block_number)?;
            let next_block_number = int_to_hex(block_number + 1);
            Some(T::new(next_block_number, deal.block_number, data))
        }
    }
}
