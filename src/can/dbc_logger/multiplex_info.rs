//! [`MultiplexInfo`].

#[allow(unused_imports)]
use super::*;
use alloc::collections::BTreeSet;

/// Information about a multiplexed message.
#[derive(Debug)]
pub(crate) struct MultiplexInfo {
    /// Index of the multiplexor switch signal in the message
    pub(crate) switch_index: usize,
    /// All mux values used by signals in this message
    pub(crate) mux_values: BTreeSet<u64>,
}
