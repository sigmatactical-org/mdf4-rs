//! [`EventCause`].

#[allow(unused_imports)]
use super::*;

/// Event cause enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum EventCause {
    /// Other/unknown cause.
    Other = 0,
    /// Error condition.
    Error = 1,
    /// Tool-generated event.
    Tool = 2,
    /// Script-generated event.
    Script = 3,
    /// User-generated event.
    User = 4,
}
impl EventCause {
    /// Create from raw u8 value.
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Other),
            1 => Some(Self::Error),
            2 => Some(Self::Tool),
            3 => Some(Self::Script),
            4 => Some(Self::User),
            _ => None,
        }
    }
}
