//! [`AttachmentFlags`].

#[allow(unused_imports)]
use super::*;

/// Attachment flags bit definitions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct AttachmentFlags(u16);
impl AttachmentFlags {
    /// Bit 0: Data is embedded in this block (vs external file reference).
    pub const EMBEDDED: u16 = 0x0001;
    /// Bit 1: Embedded data is compressed (zlib).
    pub const COMPRESSED: u16 = 0x0002;
    /// Bit 2: MD5 checksum is valid.
    pub const MD5_VALID: u16 = 0x0004;

    /// Create flags from raw u16 value.
    pub fn from_u16(value: u16) -> Self {
        Self(value)
    }

    /// Get raw u16 value.
    pub fn as_u16(self) -> u16 {
        self.0
    }

    /// Check if data is embedded in this block.
    pub fn is_embedded(self) -> bool {
        self.0 & Self::EMBEDDED != 0
    }

    /// Check if embedded data is compressed.
    pub fn is_compressed(self) -> bool {
        self.0 & Self::COMPRESSED != 0
    }

    /// Check if MD5 checksum is valid.
    pub fn is_md5_valid(self) -> bool {
        self.0 & Self::MD5_VALID != 0
    }

    /// Create flags for embedded data.
    pub fn embedded() -> Self {
        Self(Self::EMBEDDED)
    }

    /// Create flags for embedded compressed data.
    pub fn embedded_compressed() -> Self {
        Self(Self::EMBEDDED | Self::COMPRESSED)
    }

    /// Create flags for external file reference.
    pub fn external() -> Self {
        Self(0)
    }
}
