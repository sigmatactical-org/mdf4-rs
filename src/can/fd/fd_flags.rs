//! [`FdFlags`].

#[allow(unused_imports)]
use super::*;

/// CAN FD frame flags.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct FdFlags {
    /// Bit Rate Switch - indicates the frame was transmitted with a higher
    /// bitrate in the data phase.
    brs: bool,
    /// Error State Indicator - indicates the transmitting node is in
    /// error passive state.
    esi: bool,
}
impl FdFlags {
    /// Create new FD flags.
    #[inline]
    pub const fn new(brs: bool, esi: bool) -> Self {
        Self { brs, esi }
    }

    /// Create flags from a raw byte.
    ///
    /// Bit 0: BRS
    /// Bit 1: ESI
    #[inline]
    pub const fn from_byte(byte: u8) -> Self {
        Self {
            brs: byte & 0x01 != 0,
            esi: byte & 0x02 != 0,
        }
    }

    /// Convert flags to a raw byte.
    #[inline]
    pub const fn to_byte(self) -> u8 {
        (self.brs as u8) | ((self.esi as u8) << 1)
    }

    /// Returns true if Bit Rate Switch is enabled.
    #[inline]
    pub const fn brs(&self) -> bool {
        self.brs
    }

    /// Returns true if Error State Indicator is set.
    #[inline]
    pub const fn esi(&self) -> bool {
        self.esi
    }
}
