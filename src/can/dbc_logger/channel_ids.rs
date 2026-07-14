//! [`ChannelIds`].

#[allow(unused_imports)]
use super::*;
use alloc::string::String;
use alloc::vec::Vec;

/// Channel IDs stored after MDF initialization.
/// Reserved for future use (e.g., updating channel metadata after initialization).
#[allow(dead_code)]
pub(crate) struct ChannelIds {
    pub(crate) time_channel: String,
    pub(crate) signal_channels: Vec<String>,
}
