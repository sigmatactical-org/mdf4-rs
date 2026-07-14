//! [`DzCompressionType`].

#[allow(unused_imports)]
use super::*;

/// Compression algorithm used in DZ block.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DzCompressionType {
    /// Deflate only (zlib).
    Deflate = 0,
    /// Transposition followed by deflate.
    TranspositionDeflate = 1,
}
impl DzCompressionType {
    /// Convert from raw u8 value.
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Deflate),
            1 => Some(Self::TranspositionDeflate),
            _ => None,
        }
    }
}
