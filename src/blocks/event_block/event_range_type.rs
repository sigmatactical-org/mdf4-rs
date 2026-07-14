//! [`EventRangeType`].

#[allow(unused_imports)]
use super::*;

/// Event range type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum EventRangeType {
    /// Point event (single instant).
    Point = 0,
    /// Begin of range.
    RangeBegin = 1,
    /// End of range.
    RangeEnd = 2,
}
impl EventRangeType {
    /// Create from raw u8 value.
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Point),
            1 => Some(Self::RangeBegin),
            2 => Some(Self::RangeEnd),
            _ => None,
        }
    }
}
