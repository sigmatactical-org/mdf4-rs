//! [`EthernetFlags`].

#[allow(unused_imports)]
use super::*;

/// Ethernet frame flags for ASAM MDF4 Bus Logging.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct EthernetFlags(u8);
impl EthernetFlags {
    /// Bit 0: Frame direction (0 = Rx, 1 = Tx).
    pub const TX: u8 = 0x01;
    /// Bit 1: FCS (Frame Check Sequence) is valid.
    pub const FCS_VALID: u8 = 0x02;
    /// Bit 2: Frame was truncated.
    pub const TRUNCATED: u8 = 0x04;
    /// Bit 3: CRC error detected.
    pub const CRC_ERROR: u8 = 0x08;
    /// Bit 4: Frame has VLAN tag.
    pub const VLAN_TAGGED: u8 = 0x10;

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

    /// Check if FCS is valid.
    pub fn fcs_valid(self) -> bool {
        self.0 & Self::FCS_VALID != 0
    }

    /// Check if frame was truncated.
    pub fn is_truncated(self) -> bool {
        self.0 & Self::TRUNCATED != 0
    }

    /// Check if CRC error was detected.
    pub fn has_crc_error(self) -> bool {
        self.0 & Self::CRC_ERROR != 0
    }

    /// Check if frame has VLAN tag.
    pub fn has_vlan_tag(self) -> bool {
        self.0 & Self::VLAN_TAGGED != 0
    }

    /// Set the transmit flag.
    pub fn with_tx(self, tx: bool) -> Self {
        if tx {
            Self(self.0 | Self::TX)
        } else {
            Self(self.0 & !Self::TX)
        }
    }

    /// Set the FCS valid flag.
    pub fn with_fcs_valid(self, valid: bool) -> Self {
        if valid {
            Self(self.0 | Self::FCS_VALID)
        } else {
            Self(self.0 & !Self::FCS_VALID)
        }
    }

    /// Set the VLAN tagged flag.
    pub fn with_vlan_tagged(self, tagged: bool) -> Self {
        if tagged {
            Self(self.0 | Self::VLAN_TAGGED)
        } else {
            Self(self.0 & !Self::VLAN_TAGGED)
        }
    }
}
