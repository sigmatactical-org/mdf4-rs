//! [`AsamCanGroup`].

#[allow(unused_imports)]
use super::*;

/// Information about an ASAM CAN_DataFrame channel group.
#[derive(Debug)]
pub(crate) struct AsamCanGroup {
    /// Index in the MdfIndex channel_groups
    pub(crate) group_index: usize,
    /// Timestamp channel index
    pub(crate) timestamp_channel: usize,
    /// CAN_DataFrame channel index (ByteArray)
    pub(crate) dataframe_channel: usize,
}
