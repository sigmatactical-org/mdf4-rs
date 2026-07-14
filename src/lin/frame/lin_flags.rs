//! [`LinFlags`].

#[allow(unused_imports)]
use super::*;

/// LIN frame flags for ASAM MDF4 Bus Logging.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct LinFlags(u8);
impl LinFlags {
    /// Bit 0: Frame direction (0 = Rx, 1 = Tx).
    pub const TX: u8 = 0x01;
    /// Bit 1: Wake-up signal.
    pub const WAKEUP: u8 = 0x02;
    /// Bit 2: Checksum error.
    pub const CHECKSUM_ERROR: u8 = 0x04;
    /// Bit 3: No response (slave didn't respond).
    pub const NO_RESPONSE: u8 = 0x08;
    /// Bit 4: Sync error.
    pub const SYNC_ERROR: u8 = 0x10;
    /// Bit 5: Framing error.
    pub const FRAMING_ERROR: u8 = 0x20;
    /// Bit 6: Short response (incomplete data).
    pub const SHORT_RESPONSE: u8 = 0x40;
    /// Bit 7: Enhanced checksum used (LIN 2.x).
    pub const ENHANCED_CHECKSUM: u8 = 0x80;

    /// Create flags from raw byte.
    pub fn from_byte(value: u8) -> Self {
        Self(value)
    }

    /// Get raw byte value.
    pub fn to_byte(self) -> u8 {
        self.0
    }

    /// Create flags for received frame.
    pub fn rx() -> Self {
        Self(0)
    }

    /// Create flags for transmitted frame.
    pub fn tx() -> Self {
        Self(Self::TX)
    }

    /// Check if this is a transmitted frame.
    pub fn is_tx(self) -> bool {
        self.0 & Self::TX != 0
    }

    /// Check if this is a received frame.
    pub fn is_rx(self) -> bool {
        !self.is_tx()
    }

    /// Check if this is a wake-up frame.
    pub fn is_wakeup(self) -> bool {
        self.0 & Self::WAKEUP != 0
    }

    /// Check if checksum error occurred.
    pub fn has_checksum_error(self) -> bool {
        self.0 & Self::CHECKSUM_ERROR != 0
    }

    /// Check if slave didn't respond.
    pub fn has_no_response(self) -> bool {
        self.0 & Self::NO_RESPONSE != 0
    }

    /// Check if sync error occurred.
    pub fn has_sync_error(self) -> bool {
        self.0 & Self::SYNC_ERROR != 0
    }

    /// Check if framing error occurred.
    pub fn has_framing_error(self) -> bool {
        self.0 & Self::FRAMING_ERROR != 0
    }

    /// Check if response was incomplete.
    pub fn has_short_response(self) -> bool {
        self.0 & Self::SHORT_RESPONSE != 0
    }

    /// Check if enhanced (LIN 2.x) checksum is used.
    pub fn uses_enhanced_checksum(self) -> bool {
        self.0 & Self::ENHANCED_CHECKSUM != 0
    }

    /// Check if any error occurred.
    pub fn has_error(self) -> bool {
        self.0
            & (Self::CHECKSUM_ERROR
                | Self::NO_RESPONSE
                | Self::SYNC_ERROR
                | Self::FRAMING_ERROR
                | Self::SHORT_RESPONSE)
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

    /// Set the enhanced checksum flag.
    pub fn with_enhanced_checksum(self, enhanced: bool) -> Self {
        if enhanced {
            Self(self.0 | Self::ENHANCED_CHECKSUM)
        } else {
            Self(self.0 & !Self::ENHANCED_CHECKSUM)
        }
    }
}
