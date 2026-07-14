//! [`ChecksumType`].

#[allow(unused_imports)]
use super::*;

/// LIN checksum type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum ChecksumType {
    /// Classic checksum (LIN 1.x) - sum of data bytes only.
    #[default]
    Classic = 0,
    /// Enhanced checksum (LIN 2.x) - sum of ID and data bytes.
    Enhanced = 1,
}
impl ChecksumType {
    /// Create from raw byte value.
    pub fn from_u8(value: u8) -> Self {
        match value {
            1 => Self::Enhanced,
            _ => Self::Classic,
        }
    }
}
