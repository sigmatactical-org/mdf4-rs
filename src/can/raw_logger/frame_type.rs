//! [`FrameType`].

#[allow(unused_imports)]
use super::*;
use alloc::string::String;

/// Frame type classification for ASAM channel grouping.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) enum FrameType {
    /// Classic CAN with 11-bit standard ID
    Classic,
    /// Classic CAN with 29-bit extended ID
    ClassicExtended,
    /// CAN FD with 11-bit standard ID, DLC <= 8
    FdSmall,
    /// CAN FD with 29-bit extended ID, DLC <= 8
    FdSmallExtended,
    /// CAN FD with 11-bit standard ID, DLC > 8
    FdLarge,
    /// CAN FD with 29-bit extended ID, DLC > 8
    FdLargeExtended,
}
impl FrameType {
    /// All frame type variants for zero-allocation iteration.
    pub(crate) const ALL: [Self; 6] = [
        Self::Classic,
        Self::ClassicExtended,
        Self::FdSmall,
        Self::FdSmallExtended,
        Self::FdLarge,
        Self::FdLargeExtended,
    ];

    /// ASAM channel-group name for this frame kind on `bus_name`.
    pub(crate) fn group_name(&self, bus_name: &str) -> String {
        match self {
            FrameType::Classic => alloc::format!("{}_DataFrame", bus_name),
            FrameType::ClassicExtended => alloc::format!("{}_DataFrame_IDE", bus_name),
            FrameType::FdSmall => alloc::format!("{}_DataFrame_FD", bus_name),
            FrameType::FdSmallExtended => alloc::format!("{}_DataFrame_FD_IDE", bus_name),
            FrameType::FdLarge => alloc::format!("{}_DataFrame_FD_DLC_over_8", bus_name),
            FrameType::FdLargeExtended => {
                alloc::format!("{}_DataFrame_FD_IDE_DLC_over_8", bus_name)
            }
        }
    }

    /// ASAM frame channel name.
    pub(crate) fn channel_name(&self) -> &'static str {
        "CAN_DataFrame"
    }

    /// Maximum payload bytes for this frame kind.
    pub(crate) fn max_data_len(&self) -> usize {
        match self {
            FrameType::Classic | FrameType::ClassicExtended => 8,
            FrameType::FdSmall | FrameType::FdSmallExtended => 8,
            FrameType::FdLarge | FrameType::FdLargeExtended => 64,
        }
    }

    /// Classify a frame by its flags and payload length.
    pub(crate) fn from_frame(is_extended: bool, is_fd: bool, data_len: usize) -> Self {
        match (is_extended, is_fd, data_len > 8) {
            (false, false, _) => FrameType::Classic,
            (true, false, _) => FrameType::ClassicExtended,
            (false, true, false) => FrameType::FdSmall,
            (true, true, false) => FrameType::FdSmallExtended,
            (false, true, true) => FrameType::FdLarge,
            (true, true, true) => FrameType::FdLargeExtended,
        }
    }
}
