use marine_rs_sdk::marine;
use std::cmp::Ordering;

/// Convert hex string to u64
pub fn hex_to_int(hex: &str) -> Option<u64> {
    let hex = hex.trim_start_matches("0x");
    u64::from_str_radix(hex, 16).ok()
}

/// Convert u64 to hex string
pub fn int_to_hex(num: u64) -> String {
    format!("{:#x}", num)
}

#[marine]
/// Calculates a difference between two hex strings as u64 number
/// Returns 0 on overflow
pub fn hex_diff(from: String, to: String) -> u64 {
    let diff: Option<u64> = try {
        let from = hex_to_int(&from)?;
        let to = hex_to_int(&to)?;

        to.checked_sub(from)?
    };
    diff.unwrap_or(0)
}

#[marine]
pub struct HexSub {
    pub diff: Vec<String>,
    pub success: bool,
}

impl HexSub {
    pub fn error() -> Self {
        HexSub {
            diff: vec![],
            success: false,
        }
    }

    pub fn success(diff: String) -> Self {
        HexSub {
            diff: vec![diff],
            success: true,
        }
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

#[marine]
pub struct HexCmp {
    /// Less = -1
    /// Equal = 0
    /// Greater = 1
    pub ordering: i8,
    pub success: bool,
    pub error: String,
}

fn hex_cmp_error(hex_a: &str, a_ok: bool, hex_b: &str, b_ok: bool) -> String {
    let a = if a_ok {
        format!("first argument is not a valid hex: {}\n", hex_a)
    } else {
        String::new()
    };

    let b = if b_ok {
        format!("second argument is not a valid hex: {}\n", hex_b)
    } else {
        String::new()
    };

    format!("{}{}", a, b)
}

#[marine]
pub fn hex_cmp(hex_a: &str, hex_b: &str) -> HexCmp {
    let int_a = hex_to_int(hex_a);
    let int_b = hex_to_int(hex_b);

    let ordering: Ordering = int_a.cmp(&int_b);

    HexCmp {
        ordering: ordering as i8,
        success: int_a.is_some() && int_b.is_some(),
        error: hex_cmp_error(hex_a, int_a.is_some(), hex_b, int_b.is_some()),
    }
}
