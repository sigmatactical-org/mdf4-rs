//! [`RawEthFrame`].

use super::super::frame::EthernetFlags;
#[allow(unused_imports)]
use super::*;
use crate::bus_logging::timestamp_to_seconds;
use alloc::vec::Vec;

/// A buffered raw Ethernet frame with timestamp.
#[derive(Clone)]
pub(crate) struct RawEthFrame {
    /// Timestamp in seconds (ASAM uses float64 seconds)
    pub(crate) timestamp_s: f64,
    /// Frame data (complete Ethernet frame including header)
    pub(crate) data: Vec<u8>,
    /// Frame flags
    pub(crate) flags: EthernetFlags,
}
impl RawEthFrame {
    /// Frame record from raw bytes (truncated to the jumbo cap).
    pub(crate) fn new(timestamp_us: u64, data: Vec<u8>, flags: EthernetFlags) -> Self {
        Self {
            timestamp_s: timestamp_to_seconds(timestamp_us),
            data,
            flags,
        }
    }

    /// The size class this frame is logged under.
    pub(crate) fn frame_size(&self) -> FrameSize {
        FrameSize::from_length(self.data.len())
    }

    /// Build the ETH_Frame ByteArray in ASAM format.
    ///
    /// Format:
    /// - Byte 0: Flags (direction, FCS valid, etc.)
    /// - Bytes 1-2: Frame length (little-endian)
    /// - Bytes 3+: Frame data (Dst MAC + Src MAC + EtherType + Payload)
    pub(crate) fn to_frame_bytes(&self, max_size: usize) -> Vec<u8> {
        // ASAM format: flags(1) + length(2) + frame data
        let header_size = 3;
        let total_size = header_size + max_size;

        let mut bytes = Vec::with_capacity(total_size);

        // Flags byte
        bytes.push(self.flags.to_byte());

        // Frame length (little-endian u16)
        let frame_len = self.data.len() as u16;
        bytes.extend_from_slice(&frame_len.to_le_bytes());

        // Frame data (padded to max_size)
        let copy_len = self.data.len().min(max_size);
        bytes.extend_from_slice(&self.data[..copy_len]);
        bytes.resize(total_size, 0);

        bytes
    }
}
