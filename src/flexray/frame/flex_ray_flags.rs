//! [`FlexRayFlags`].

#[allow(unused_imports)]
use super::*;

/// FlexRay frame flags for ASAM MDF4 Bus Logging.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct FlexRayFlags(u16);
impl FlexRayFlags {
    /// Bit 0: Frame direction (0 = Rx, 1 = Tx).
    pub const TX: u16 = 0x0001;
    /// Bit 1: Startup frame indicator.
    pub const STARTUP: u16 = 0x0002;
    /// Bit 2: Sync frame indicator.
    pub const SYNC: u16 = 0x0004;
    /// Bit 3: Null frame indicator (no payload).
    pub const NULL_FRAME: u16 = 0x0008;
    /// Bit 4: Payload preamble indicator.
    pub const PAYLOAD_PREAMBLE: u16 = 0x0010;
    /// Bit 5: Header CRC error.
    pub const HEADER_CRC_ERROR: u16 = 0x0020;
    /// Bit 6: Frame CRC error.
    pub const FRAME_CRC_ERROR: u16 = 0x0040;
    /// Bit 7: Coding error.
    pub const CODING_ERROR: u16 = 0x0080;
    /// Bit 8: TSS violation.
    pub const TSS_VIOLATION: u16 = 0x0100;
    /// Bit 9: Valid frame received.
    pub const VALID: u16 = 0x0200;
    /// Bit 10: Network Management Vector included.
    pub const NM_VECTOR: u16 = 0x0400;
    /// Bit 11: Dynamic segment frame.
    pub const DYNAMIC: u16 = 0x0800;

    /// Create flags from raw u16.
    pub fn from_u16(value: u16) -> Self {
        Self(value)
    }

    /// Get raw u16 value.
    pub fn to_u16(self) -> u16 {
        self.0
    }

    /// Create flags for received frame.
    pub fn rx() -> Self {
        Self(Self::VALID)
    }

    /// Create flags for transmitted frame.
    pub fn tx() -> Self {
        Self(Self::TX | Self::VALID)
    }

    /// Check if this is a transmitted frame.
    pub fn is_tx(self) -> bool {
        self.0 & Self::TX != 0
    }

    /// Check if this is a received frame.
    pub fn is_rx(self) -> bool {
        !self.is_tx()
    }

    /// Check if this is a startup frame.
    pub fn is_startup(self) -> bool {
        self.0 & Self::STARTUP != 0
    }

    /// Check if this is a sync frame.
    pub fn is_sync(self) -> bool {
        self.0 & Self::SYNC != 0
    }

    /// Check if this is a null frame.
    pub fn is_null_frame(self) -> bool {
        self.0 & Self::NULL_FRAME != 0
    }

    /// Check if payload preamble is present.
    pub fn has_payload_preamble(self) -> bool {
        self.0 & Self::PAYLOAD_PREAMBLE != 0
    }

    /// Check if header CRC error occurred.
    pub fn has_header_crc_error(self) -> bool {
        self.0 & Self::HEADER_CRC_ERROR != 0
    }

    /// Check if frame CRC error occurred.
    pub fn has_frame_crc_error(self) -> bool {
        self.0 & Self::FRAME_CRC_ERROR != 0
    }

    /// Check if coding error occurred.
    pub fn has_coding_error(self) -> bool {
        self.0 & Self::CODING_ERROR != 0
    }

    /// Check if frame is valid.
    pub fn is_valid(self) -> bool {
        self.0 & Self::VALID != 0
    }

    /// Check if NM vector is included.
    pub fn has_nm_vector(self) -> bool {
        self.0 & Self::NM_VECTOR != 0
    }

    /// Check if this is a dynamic segment frame.
    pub fn is_dynamic(self) -> bool {
        self.0 & Self::DYNAMIC != 0
    }

    /// Check if any error occurred.
    pub fn has_error(self) -> bool {
        self.0
            & (Self::HEADER_CRC_ERROR
                | Self::FRAME_CRC_ERROR
                | Self::CODING_ERROR
                | Self::TSS_VIOLATION)
            != 0
    }

    /// Set the transmit flag.
    pub fn with_tx(self, tx: bool) -> Self {
        if tx {
            Self(self.0 | Self::TX)
        } else {
            Self(self.0 & !Self::TX)
        }
    }

    /// Set the valid flag.
    pub fn with_valid(self, valid: bool) -> Self {
        if valid {
            Self(self.0 | Self::VALID)
        } else {
            Self(self.0 & !Self::VALID)
        }
    }

    /// Set the startup flag.
    pub fn with_startup(self, startup: bool) -> Self {
        if startup {
            Self(self.0 | Self::STARTUP)
        } else {
            Self(self.0 & !Self::STARTUP)
        }
    }

    /// Set the sync flag.
    pub fn with_sync(self, sync: bool) -> Self {
        if sync {
            Self(self.0 | Self::SYNC)
        } else {
            Self(self.0 & !Self::SYNC)
        }
    }

    /// Set the null frame flag.
    pub fn with_null_frame(self, null_frame: bool) -> Self {
        if null_frame {
            Self(self.0 | Self::NULL_FRAME)
        } else {
            Self(self.0 & !Self::NULL_FRAME)
        }
    }

    /// Set the dynamic segment flag.
    pub fn with_dynamic(self, dynamic: bool) -> Self {
        if dynamic {
            Self(self.0 | Self::DYNAMIC)
        } else {
            Self(self.0 & !Self::DYNAMIC)
        }
    }
}
