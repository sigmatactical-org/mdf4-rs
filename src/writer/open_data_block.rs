//! [`OpenDataBlock`].

#[allow(unused_imports)]
use super::*;
use crate::blocks::ChannelBlock;
use alloc::string::String;
use alloc::vec::Vec;
use data::ChannelEncoder;

/// Helper structure tracking an open data block during writing.
pub(crate) struct OpenDataBlock {
    pub(crate) dg_id: String,
    pub(crate) dt_id: String,
    pub(crate) start_pos: u64,
    pub(crate) record_size: usize,
    pub(crate) record_count: u64,
    /// Total number of records written across all DT blocks for this group
    pub(crate) total_record_count: u64,
    pub(crate) channels: Vec<ChannelBlock>,
    pub(crate) dt_ids: Vec<String>,
    pub(crate) dt_positions: Vec<u64>,
    pub(crate) dt_sizes: Vec<u64>,
    /// Scratch buffer reused for record encoding
    pub(crate) record_buf: Vec<u8>,
    /// Template filled with constant values used to initialise each record
    pub(crate) record_template: Vec<u8>,
    /// Precomputed per-channel encoders
    pub(crate) encoders: Vec<ChannelEncoder>,
}
