//! [`RawFrame`].

use crate::can::fd::{FdFlags, MAX_FD_DATA_LEN};
#[allow(unused_imports)]
use super::*;
use crate::bus_logging::timestamp_to_seconds;
use alloc::vec::Vec;

/// A buffered raw CAN frame in ASAM format.
#[derive(Clone)]
pub(crate) struct RawFrame {
    /// Timestamp in seconds (ASAM uses float64 seconds)
    pub(crate) timestamp_s: f64,
    /// CAN ID (11 or 29 bits)
    pub(crate) can_id: u32,
    /// Data Length Code
    pub(crate) dlc: u8,
    /// Frame data
    pub(crate) data: [u8; MAX_FD_DATA_LEN],
    /// Actual data length
    pub(crate) data_len: usize,
    /// CAN FD flags (BRS, ESI)
    pub(crate) fd_flags: FdFlags,
    /// True if this frame uses a 29-bit extended ID
    pub(crate) is_extended: bool,
    /// True if this is a CAN FD frame
    pub(crate) is_fd: bool,
}
impl RawFrame {
    /// Classic CAN frame record.
    pub(crate) fn new_classic(
        timestamp_us: u64,
        can_id: u32,
        dlc: u8,
        data: &[u8],
        is_extended: bool,
    ) -> Self {
        let mut frame_data = [0u8; MAX_FD_DATA_LEN];
        let len = data.len().min(8);
        frame_data[..len].copy_from_slice(&data[..len]);
        Self {
            timestamp_s: timestamp_to_seconds(timestamp_us),
            can_id,
            dlc,
            data: frame_data,
            data_len: len,
            fd_flags: FdFlags::default(),
            is_extended,
            is_fd: false,
        }
    }

    /// CAN FD frame record.
    pub(crate) fn new_fd(
        timestamp_us: u64,
        can_id: u32,
        dlc: u8,
        data: &[u8],
        flags: FdFlags,
        is_extended: bool,
    ) -> Self {
        let mut frame_data = [0u8; MAX_FD_DATA_LEN];
        let len = data.len().min(MAX_FD_DATA_LEN);
        frame_data[..len].copy_from_slice(&data[..len]);
        Self {
            timestamp_s: timestamp_to_seconds(timestamp_us),
            can_id,
            dlc,
            data: frame_data,
            data_len: len,
            fd_flags: flags,
            is_extended,
            is_fd: true,
        }
    }

    /// The group this frame is logged under.
    pub(crate) fn frame_type(&self) -> FrameType {
        FrameType::from_frame(self.is_extended, self.is_fd, self.data_len)
    }

    /// Build the CAN_DataFrame ByteArray in ASAM format.
    ///
    /// Format for classic CAN (13 bytes):
    /// - Bytes 0-3: CAN ID (little-endian, bit 31 set for extended ID)
    /// - Byte 4: DLC
    /// - Bytes 5-12: Data (8 bytes, zero-padded)
    ///
    /// Format for CAN FD with DLC > 8:
    /// - Bytes 0-3: CAN ID (little-endian, bit 31 set for extended ID)
    /// - Byte 4: DLC
    /// - Byte 5: FD flags (bit 0 = BRS, bit 1 = ESI)
    /// - Bytes 6+: Data (up to 64 bytes)
    pub(crate) fn to_dataframe_bytes(&self) -> Vec<u8> {
        let max_data = self.frame_type().max_data_len();
        let has_fd_flags = self.is_fd && self.data_len > 8;

        // Calculate total size: ID(4) + DLC(1) + [FD_flags(1)] + Data(max_data)
        let total_size = if has_fd_flags {
            4 + 1 + 1 + max_data
        } else {
            4 + 1 + max_data
        };

        let mut bytes = Vec::with_capacity(total_size);

        // CAN ID with extended flag in bit 31
        let id_with_flags = if self.is_extended {
            self.can_id | 0x8000_0000
        } else {
            self.can_id
        };
        bytes.extend_from_slice(&id_with_flags.to_le_bytes());

        // DLC
        bytes.push(self.dlc);

        // FD flags (only for FD frames with DLC > 8)
        if has_fd_flags {
            bytes.push(self.fd_flags.to_byte());
        }

        // Data (zero-padded to max_data)
        bytes.extend_from_slice(&self.data[..self.data_len.min(max_data)]);
        bytes.resize(total_size, 0);

        bytes
    }
}
