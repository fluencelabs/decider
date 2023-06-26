/// Convert hex string to u64
pub fn hex_to_int(block: &str) -> Option<u64> {
    let block = block.trim_start_matches("0x");
    u64::from_str_radix(block, 16).ok()
}

/// Convert u64 to hex string
pub fn int_to_hex(num: u64) -> String {
    format!("{:#x}", num)
}
