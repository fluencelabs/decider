use crate::hex::{hex_to_int, int_to_hex};

// TODO: make it configurable
pub const DEFAULT_BLOCK_RANGE: u64 = 2000;

/// Default value for `right_boundary` in chain polling
///
/// Calculated based on `left_boundary` by adding `DEFAULT_BLOCK_RANGE`
/// If `left_boundary` is not a hex string, return `"latest"`
pub fn default_right_boundary(left_boundary: &str) -> String {
    let right_boundary = try {
        let left_boundary = hex_to_int(left_boundary)?;
        left_boundary.checked_add(DEFAULT_BLOCK_RANGE)?
    };
    match right_boundary {
        Some(right_boundary) => int_to_hex(right_boundary),
        None => "latest".to_string(),
    }
}
