//! [`FrameSize`].

use super::super::frame::{ETH_HEADER_SIZE, MAX_ETHERNET_FRAME, MAX_JUMBO_PAYLOAD};
#[allow(unused_imports)]
use super::*;
use alloc::string::String;

/// Frame size classification for ASAM channel grouping.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) enum FrameSize {
    /// Standard Ethernet frame (up to 1514 bytes)
    Standard,
    /// Jumbo frame (> 1514 bytes, up to ~9000 bytes)
    Jumbo,
}
impl FrameSize {
    /// All frame size variants for zero-allocation iteration.
    pub(crate) const ALL: [Self; 2] = [Self::Standard, Self::Jumbo];

    /// ASAM channel-group name for this size class on `source_name`.
    pub(crate) fn group_name(&self, source_name: &str) -> String {
        match self {
            FrameSize::Standard => alloc::format!("{}_ETH_Frame", source_name),
            FrameSize::Jumbo => alloc::format!("{}_ETH_Frame_Jumbo", source_name),
        }
    }

    /// Maximum frame bytes stored for this size class.
    pub(crate) fn max_frame_size(&self) -> usize {
        match self {
            FrameSize::Standard => MAX_ETHERNET_FRAME,
            FrameSize::Jumbo => ETH_HEADER_SIZE + MAX_JUMBO_PAYLOAD,
        }
    }

    /// Classify a frame by its length (standard vs jumbo).
    pub(crate) fn from_length(len: usize) -> Self {
        if len > MAX_ETHERNET_FRAME {
            FrameSize::Jumbo
        } else {
            FrameSize::Standard
        }
    }
}
