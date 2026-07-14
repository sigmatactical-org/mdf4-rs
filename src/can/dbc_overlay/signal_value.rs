//! [`SignalValue`].

#[allow(unused_imports)]
use super::*;

/// A timestamped signal value.
#[derive(Debug, Clone)]
pub struct SignalValue {
    /// Timestamp in microseconds
    pub timestamp_us: u64,
    /// Physical value after DBC conversion
    pub value: f64,
    /// Raw integer value before conversion
    pub raw_value: i64,
}
