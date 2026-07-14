//! CAN FD (Flexible Data-rate) support.
//!
//! CAN FD extends classic CAN with:
//! - Data payloads up to 64 bytes (vs 8 for classic CAN)
//! - Bit Rate Switch (BRS) for faster data transmission
//! - Error State Indicator (ESI)
//!
//! # DLC to Data Length Mapping
//!
//! CAN FD uses a non-linear DLC to data length mapping for values > 8:
//! - DLC 0-8: data length = DLC
//! - DLC 9: 12 bytes
//! - DLC 10: 16 bytes
//! - DLC 11: 20 bytes
//! - DLC 12: 24 bytes
//! - DLC 13: 32 bytes
//! - DLC 14: 48 bytes
//! - DLC 15: 64 bytes
//!
//! # Example
//!
//! ```ignore
//! use mdf4_rs::can::{FdFrame, FdFlags};
//!
//! // Check if a frame is CAN FD
//! if frame.is_fd() {
//!     let flags = frame.fd_flags();
//!     if flags.brs() {
//!         println!("Bit rate switch enabled");
//!     }
//! }
//! ```

mod fd_flags;
#[cfg(feature = "can")]
mod simple_fd_frame;
pub use fd_flags::FdFlags;
#[cfg(feature = "can")]
pub use simple_fd_frame::SimpleFdFrame;

/// Maximum CAN FD data length in bytes.
pub const MAX_FD_DATA_LEN: usize = 64;

/// CAN FD DLC to data length mapping.
///
/// Returns the actual data length for a given DLC value.
#[inline]
pub const fn dlc_to_len(dlc: u8) -> usize {
    match dlc {
        0..=8 => dlc as usize,
        9 => 12,
        10 => 16,
        11 => 20,
        12 => 24,
        13 => 32,
        14 => 48,
        15 => 64,
        _ => 64, // Invalid DLC, assume max
    }
}

/// Data length to CAN FD DLC mapping.
///
/// Returns the minimum DLC that can hold the given data length.
#[inline]
pub const fn len_to_dlc(len: usize) -> u8 {
    match len {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => 4,
        5 => 5,
        6 => 6,
        7 => 7,
        8 => 8,
        9..=12 => 9,
        13..=16 => 10,
        17..=20 => 11,
        21..=24 => 12,
        25..=32 => 13,
        33..=48 => 14,
        _ => 15, // 49-64 bytes
    }
}

// ============================================================================
// embedded_can integration (requires `can` feature)
// ============================================================================

/// Trait for CAN FD frames.
///
/// This extends the concept of `embedded_can::Frame` for CAN FD,
/// supporting up to 64 bytes of data and FD-specific flags.
#[cfg(feature = "can")]
pub trait FdFrame: Sized {
    /// Creates a new CAN FD frame.
    ///
    /// Returns `None` if the data slice is too long (> 64 bytes).
    fn new_fd(id: impl Into<embedded_can::Id>, data: &[u8], flags: FdFlags) -> Option<Self>;

    /// Returns true if this is a CAN FD frame (vs classic CAN).
    fn is_fd(&self) -> bool;

    /// Returns the CAN FD flags (BRS, ESI).
    ///
    /// For classic CAN frames, returns default flags (both false).
    fn fd_flags(&self) -> FdFlags;

    /// Returns the frame identifier.
    fn id(&self) -> embedded_can::Id;

    /// Returns the data length code (DLC).
    ///
    /// For CAN FD, DLC can be 0-15, mapping to 0-64 bytes.
    fn dlc(&self) -> usize;

    /// Returns the actual data length in bytes.
    ///
    /// This handles the CAN FD DLC-to-length mapping.
    fn data_len(&self) -> usize {
        if self.is_fd() {
            dlc_to_len(self.dlc() as u8)
        } else {
            self.dlc().min(8)
        }
    }

    /// Returns the frame data.
    fn data(&self) -> &[u8];

    /// Returns true if this is an extended frame (29-bit ID).
    fn is_extended(&self) -> bool {
        matches!(self.id(), embedded_can::Id::Extended(_))
    }

    /// Returns true if this is a standard frame (11-bit ID).
    fn is_standard(&self) -> bool {
        !self.is_extended()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dlc_to_len() {
        assert_eq!(dlc_to_len(0), 0);
        assert_eq!(dlc_to_len(8), 8);
        assert_eq!(dlc_to_len(9), 12);
        assert_eq!(dlc_to_len(10), 16);
        assert_eq!(dlc_to_len(11), 20);
        assert_eq!(dlc_to_len(12), 24);
        assert_eq!(dlc_to_len(13), 32);
        assert_eq!(dlc_to_len(14), 48);
        assert_eq!(dlc_to_len(15), 64);
    }

    #[test]
    fn test_len_to_dlc() {
        assert_eq!(len_to_dlc(0), 0);
        assert_eq!(len_to_dlc(8), 8);
        assert_eq!(len_to_dlc(12), 9);
        assert_eq!(len_to_dlc(16), 10);
        assert_eq!(len_to_dlc(20), 11);
        assert_eq!(len_to_dlc(24), 12);
        assert_eq!(len_to_dlc(32), 13);
        assert_eq!(len_to_dlc(48), 14);
        assert_eq!(len_to_dlc(64), 15);
        // In-between values
        assert_eq!(len_to_dlc(10), 9); // 10 bytes needs DLC 9 (12 bytes)
        assert_eq!(len_to_dlc(50), 15); // 50 bytes needs DLC 15 (64 bytes)
    }

    #[test]
    fn test_fd_flags() {
        let flags = FdFlags::new(true, false);
        assert!(flags.brs());
        assert!(!flags.esi());
        assert_eq!(flags.to_byte(), 0x01);

        let flags = FdFlags::new(false, true);
        assert!(!flags.brs());
        assert!(flags.esi());
        assert_eq!(flags.to_byte(), 0x02);

        let flags = FdFlags::new(true, true);
        assert_eq!(flags.to_byte(), 0x03);

        let flags = FdFlags::from_byte(0x03);
        assert!(flags.brs());
        assert!(flags.esi());
    }

    #[cfg(feature = "can")]
    #[test]
    fn test_simple_fd_frame_classic() {
        use embedded_can::StandardId;

        let id = StandardId::new(0x100).unwrap();
        let frame = SimpleFdFrame::new_classic(id, &[1, 2, 3, 4, 5, 6, 7, 8]).unwrap();

        assert!(!frame.is_fd());
        assert_eq!(frame.data(), &[1, 2, 3, 4, 5, 6, 7, 8]);
        assert!(!frame.fd_flags().brs());
        assert!(!frame.fd_flags().esi());
    }

    #[cfg(feature = "can")]
    #[test]
    fn test_simple_fd_frame_fd() {
        use embedded_can::StandardId;

        let id = StandardId::new(0x100).unwrap();
        let data = [0xAAu8; 32];
        let flags = FdFlags::new(true, false);
        let frame = SimpleFdFrame::new_fd_frame(id, &data, flags).unwrap();

        assert!(frame.is_fd());
        assert_eq!(frame.data().len(), 32);
        assert!(frame.fd_flags().brs());
        assert!(!frame.fd_flags().esi());
    }

    #[cfg(feature = "can")]
    #[test]
    fn test_simple_fd_frame_max_size() {
        use embedded_can::StandardId;

        let id = StandardId::new(0x100).unwrap();
        let data = [0xBBu8; 64];
        let frame = SimpleFdFrame::new_fd_frame(id, &data, FdFlags::default()).unwrap();

        assert!(frame.is_fd());
        assert_eq!(frame.data().len(), 64);
    }

    #[cfg(feature = "can")]
    #[test]
    fn test_simple_fd_frame_too_large() {
        use embedded_can::StandardId;

        let id = StandardId::new(0x100).unwrap();
        let data = [0u8; 65]; // Too large
        assert!(SimpleFdFrame::new_fd_frame(id, &data, FdFlags::default()).is_none());
    }
}
