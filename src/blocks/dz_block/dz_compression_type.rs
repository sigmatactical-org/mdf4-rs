//! [`DzCompressionType`].

#[allow(unused_imports)]
use super::*;
use crate::{
    Error, Result,
    blocks::common::{BlockHeader, BlockParse, read_u8, read_u32, read_u64, validate_buffer_size},
};
use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;

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
