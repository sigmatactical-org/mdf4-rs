//! [`FlexRayChannel`].

#[allow(unused_imports)]
use super::*;

/// FlexRay channel identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum FlexRayChannel {
    /// Channel A.
    #[default]
    A = 0,
    /// Channel B.
    B = 1,
    /// Both channels (A and B).
    AB = 2,
}
impl FlexRayChannel {
    /// Create from raw byte value.
    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => Self::A,
            1 => Self::B,
            2 => Self::AB,
            _ => Self::A,
        }
    }
}
