//! [`DataBlockInfo`].

#[allow(unused_imports)]
use super::*;
#[cfg(feature = "compression")]
use crate::blocks::DzBlock;

/// Location and metadata for a data block within the MDF file.
///
/// Each channel group can have multiple data blocks, especially in files
/// created with streaming writes. This struct stores the information needed
/// to locate and read a specific data block.
///
/// # Data Block Types
///
/// - **DT blocks**: Uncompressed raw data (most common)
/// - **DZ blocks**: Zlib-compressed data (requires decompression)
/// - **DL blocks**: Data lists pointing to multiple blocks
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DataBlockInfo {
    /// Absolute file offset where the block header starts.
    /// The actual data begins 24 bytes after this offset (after the block header).
    pub file_offset: u64,
    /// Total size of the block including the 24-byte header.
    pub size: u64,
    /// Whether this block contains compressed data (DZ block).
    /// Compressed blocks require decompression before reading values.
    pub is_compressed: bool,
}
