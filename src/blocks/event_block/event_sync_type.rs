//! [`EventSyncType`].

#[allow(unused_imports)]
use super::*;

/// Event synchronization type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum EventSyncType {
    /// Time in seconds.
    Time = 1,
    /// Angle in radians.
    Angle = 2,
    /// Distance in meters.
    Distance = 3,
    /// Index (sample number).
    Index = 4,
}
impl EventSyncType {
    /// Create from raw u8 value.
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            1 => Some(Self::Time),
            2 => Some(Self::Angle),
            3 => Some(Self::Distance),
            4 => Some(Self::Index),
            _ => None,
        }
    }
}
