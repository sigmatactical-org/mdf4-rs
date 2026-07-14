//! [`MacAddress`].

#[allow(unused_imports)]
use super::*;

/// MAC address representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct MacAddress(pub [u8; 6]);
impl MacAddress {
    /// Create a MAC address from bytes.
    pub const fn new(bytes: [u8; 6]) -> Self {
        Self(bytes)
    }

    /// Create a broadcast MAC address (FF:FF:FF:FF:FF:FF).
    pub const fn broadcast() -> Self {
        Self([0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF])
    }

    /// Create a zero/unspecified MAC address.
    pub const fn zero() -> Self {
        Self([0x00, 0x00, 0x00, 0x00, 0x00, 0x00])
    }

    /// Check if this is a broadcast address.
    pub fn is_broadcast(&self) -> bool {
        self.0 == [0xFF; 6]
    }

    /// Check if this is a multicast address (bit 0 of first byte set).
    pub fn is_multicast(&self) -> bool {
        self.0[0] & 0x01 != 0
    }

    /// Check if this is a unicast address.
    pub fn is_unicast(&self) -> bool {
        !self.is_multicast()
    }

    /// Check if this is a locally administered address (bit 1 of first byte set).
    pub fn is_local(&self) -> bool {
        self.0[0] & 0x02 != 0
    }

    /// Get the raw bytes.
    pub fn as_bytes(&self) -> &[u8; 6] {
        &self.0
    }
}
impl From<[u8; 6]> for MacAddress {
    fn from(bytes: [u8; 6]) -> Self {
        Self(bytes)
    }
}
