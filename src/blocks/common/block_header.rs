//! [`BlockHeader`].

#[allow(unused_imports)]
use super::*;
use crate::Result;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BlockHeader {
    /// 4-byte block type identifier (e.g., "##HD", "##DG").
    pub id: String,
    /// Reserved field, always 0.
    pub reserved: u32,
    /// Total length of the block in bytes, including this header.
    pub length: u64,
    /// Number of link fields in this block.
    pub link_count: u64,
}
impl Default for BlockHeader {
    /// Returns a BlockHeader with id 'UNSET' and length 0 as a placeholder.
    /// This is not a valid MDF block header and should be replaced before writing.
    fn default() -> Self {
        BlockHeader {
            id: String::from("UNSET"),
            reserved: 0,
            length: 0,
            link_count: 0,
        }
    }
}
impl BlockHeader {
    /// Serializes the BlockHeader to bytes according to MDF 4.1 specification.
    ///
    /// The BlockHeader is always 24 bytes and consists of:
    /// - id: 4 bytes (ASCII characters, must be exactly 4 bytes)
    /// - reserved: 4 bytes (always 0)
    /// - length: 8 bytes (total length of the block including this header)
    /// - link_count: 8 bytes (number of links in this block)
    ///
    /// # Returns
    /// - `Ok(Vec<u8>)` containing the serialized block header
    /// - `Err(Error)` if serialization fails
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut buffer = Vec::with_capacity(24);

        // 1. Write the ID field (4 bytes)
        let id_bytes = self.id.as_bytes();
        let mut id_field = [0u8; 4];
        let id_len = core::cmp::min(id_bytes.len(), 4);
        id_field[..id_len].copy_from_slice(&id_bytes[..id_len]);
        buffer.extend_from_slice(&id_field);

        // 2. Write reserved field (4 bytes)
        buffer.extend_from_slice(&self.reserved.to_le_bytes());

        // 3. Write length field (8 bytes)
        buffer.extend_from_slice(&self.length.to_le_bytes());

        // 4. Write link_count field (8 bytes)
        buffer.extend_from_slice(&self.link_count.to_le_bytes());

        debug_assert_eq!(buffer.len(), 24);
        Ok(buffer)
    }

    /// Parse a block header from the first 24 bytes of `bytes`.
    ///
    /// # Arguments
    /// * `bytes` - Slice containing at least 24 bytes from the MDF file.
    ///
    /// # Returns
    /// A [`BlockHeader`] on success or [`Error::TooShortBuffer`] when the
    /// slice is smaller than 24 bytes.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        validate_buffer_size(bytes, 24)?;

        let id = match core::str::from_utf8(&bytes[0..4]) {
            Ok(s) => String::from(s),
            Err(_) => String::from_utf8_lossy(&bytes[0..4]).into_owned(),
        };

        Ok(Self {
            id,
            reserved: read_u32(bytes, 4),
            length: read_u64(bytes, 8),
            link_count: read_u64(bytes, 16),
        })
    }
}
