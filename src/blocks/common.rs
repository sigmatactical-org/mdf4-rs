//! Common types, traits, and helper functions for MDF block parsing.
//!
//! This module provides:
//! - [`BlockHeader`]: The 24-byte header present in all MDF blocks
//! - [`BlockParse`]: Trait for parsing blocks from bytes
//! - [`DataType`]: Enum representing MDF data types
//! - Byte parsing helper functions to reduce code duplication

mod block_header;
mod block_parse;
mod data_type;
pub use block_header::BlockHeader;
pub use block_parse::BlockParse;
pub use data_type::DataType;

use crate::{
    Error, Result,
    blocks::{metadata_block::MetadataBlock, text_block::TextBlock},
};
use alloc::format;
use alloc::string::String;

// ============================================================================
// Byte Parsing Helpers
// ============================================================================

/// Read a u64 from a byte slice at the given offset (little-endian).
///
/// # Panics
/// Panics if `offset + 8 > bytes.len()`. Use `read_u64_checked` for fallible version.
#[inline]
pub fn read_u64(bytes: &[u8], offset: usize) -> u64 {
    u64::from_le_bytes([
        bytes[offset],
        bytes[offset + 1],
        bytes[offset + 2],
        bytes[offset + 3],
        bytes[offset + 4],
        bytes[offset + 5],
        bytes[offset + 6],
        bytes[offset + 7],
    ])
}

/// Read a u32 from a byte slice at the given offset (little-endian).
#[inline]
pub fn read_u32(bytes: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes([
        bytes[offset],
        bytes[offset + 1],
        bytes[offset + 2],
        bytes[offset + 3],
    ])
}

/// Read a u16 from a byte slice at the given offset (little-endian).
#[inline]
pub fn read_u16(bytes: &[u8], offset: usize) -> u16 {
    u16::from_le_bytes([bytes[offset], bytes[offset + 1]])
}

/// Read an f64 from a byte slice at the given offset (little-endian).
#[inline]
pub fn read_f64(bytes: &[u8], offset: usize) -> f64 {
    f64::from_le_bytes([
        bytes[offset],
        bytes[offset + 1],
        bytes[offset + 2],
        bytes[offset + 3],
        bytes[offset + 4],
        bytes[offset + 5],
        bytes[offset + 6],
        bytes[offset + 7],
    ])
}

/// Read a u8 from a byte slice at the given offset.
#[inline]
pub fn read_u8(bytes: &[u8], offset: usize) -> u8 {
    bytes[offset]
}

// ============================================================================
// Validation Helpers
// ============================================================================

/// Validate that a buffer has at least `expected` bytes.
///
/// Returns `Err(TooShortBuffer)` if the buffer is too small.
#[inline]
pub fn validate_buffer_size(bytes: &[u8], expected: usize) -> Result<()> {
    if bytes.len() < expected {
        return Err(Error::TooShortBuffer {
            actual: bytes.len(),
            expected,
            file: file!(),
            line: line!(),
        });
    }
    Ok(())
}

/// Validate that a block header has the expected ID.
#[inline]
pub fn validate_block_id(header: &BlockHeader, expected_id: &str) -> Result<()> {
    if header.id != expected_id {
        return Err(Error::BlockSerializationError(format!(
            "Block must have ID '{}', found '{}'",
            expected_id, header.id
        )));
    }
    Ok(())
}

/// Validate that a block header has the expected length.
#[inline]
pub fn validate_block_length(header: &BlockHeader, expected: u64) -> Result<()> {
    if header.length != expected {
        return Err(Error::BlockSerializationError(format!(
            "Block must have length={}, found {}",
            expected, header.length
        )));
    }
    Ok(())
}

/// Assert that a buffer size is 8-byte aligned (debug builds only).
#[inline]
pub fn debug_assert_aligned(size: usize) {
    debug_assert_eq!(size % 8, 0, "Block size {} is not 8-byte aligned", size);
}

/// Calculate padding needed to reach 8-byte alignment.
#[inline]
pub const fn padding_to_align_8(size: usize) -> usize {
    (8 - (size % 8)) % 8
}

/// Safely convert a u64 offset/address to usize for indexing.
///
/// On 64-bit systems, this is always safe. On 32-bit systems, returns an error
/// if the value exceeds `usize::MAX`, preventing potential overflow issues.
///
/// # Arguments
/// * `value` - The u64 value to convert (typically a file offset or address).
/// * `context` - Description of what the value represents (for error messages).
///
/// # Returns
/// The value as `usize`, or an error if conversion would overflow.
#[inline]
pub fn u64_to_usize(value: u64, context: &str) -> Result<usize> {
    usize::try_from(value).map_err(|_| {
        Error::BlockSerializationError(format!(
            "{} value {} exceeds maximum addressable size on this platform",
            context, value
        ))
    })
}

/// Read a text or metadata block at `address` and return its contents.
///
/// # Arguments
/// * `mmap` - The full memory mapped MDF file.
/// * `address` - Offset of the target block; use `0` for no block.
///
/// # Returns
/// The block's string contents if present or `Ok(None)` if `address` is zero or
/// the block type is not text or metadata.
pub fn read_string_block(mmap: &[u8], address: u64) -> Result<Option<String>> {
    if address == 0 {
        return Ok(None);
    }

    let offset = u64_to_usize(address, "block address")?;
    validate_buffer_size(mmap, offset + 24)?;
    let header = BlockHeader::from_bytes(&mmap[offset..offset + 24])?;

    match header.id.as_str() {
        "##TX" => Ok(Some(TextBlock::from_bytes(&mmap[offset..])?.text)),
        "##MD" => Ok(Some(MetadataBlock::from_bytes(&mmap[offset..])?.xml)),
        _ => Ok(None),
    }
}
