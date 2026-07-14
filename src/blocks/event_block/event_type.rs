//! [`EventType`].

#[allow(unused_imports)]
use super::*;

/// Event type enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum EventType {
    /// Recording event (start/stop/pause/resume of recording).
    Recording = 0,
    /// Trigger event (hardware or software trigger).
    Trigger = 1,
    /// Marker event (user-defined marker).
    Marker = 2,
}
impl EventType {
    /// Create from raw u8 value.
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Recording),
            1 => Some(Self::Trigger),
            2 => Some(Self::Marker),
            _ => None,
        }
    }
}
