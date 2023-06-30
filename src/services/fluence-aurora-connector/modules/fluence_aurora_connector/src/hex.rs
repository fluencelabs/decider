use marine_rs_sdk::marine;

/// Convert hex string to u64
pub fn hex_to_int(block: &str) -> Option<u64> {
    let block = block.trim_start_matches("0x");
    u64::from_str_radix(block, 16).ok()
}

/// Convert u64 to hex string
pub fn int_to_hex(num: u64) -> String {
    format!("{:#x}", num)
}

#[marine]
pub struct HexSub {
    pub diff: Vec<String>,
    pub success: bool,
}

impl HexSub {
    pub fn error() -> Self {
        HexSub { diff: vec![], success: false }
    }

    pub fn success(diff: String) -> Self {
        HexSub { diff: vec![diff], success: true }
    }
}

#[marine]
pub fn hex_sub(hex: &str, sub: u32) -> HexSub {
    if let Some(int) = hex_to_int(hex) {
        let diff = int - sub as u64;
        let diff = int_to_hex(diff);
        HexSub::success(diff)
    } else {
        HexSub::error()
    }
}