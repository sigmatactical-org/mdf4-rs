//! [`DecodedFrame`].

#[allow(unused_imports)]
use super::*;
use alloc::string::String;
use alloc::vec::Vec;

/// A decoded CAN frame with all signal values.
#[derive(Debug, Clone)]
pub struct DecodedFrame {
    /// Timestamp in microseconds
    pub timestamp_us: u64,
    /// CAN ID (without extended bit)
    pub can_id: u32,
    /// Whether this is an extended 29-bit ID
    pub is_extended: bool,
    /// Decoded signal values: (signal_name, physical_value)
    pub signals: Vec<(String, f64)>,
}
