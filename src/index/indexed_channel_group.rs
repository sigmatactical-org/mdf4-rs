//! [`IndexedChannelGroup`].

#[allow(unused_imports)]
use super::*;
#[cfg(feature = "compression")]
use crate::blocks::DzBlock;

/// Metadata and layout for a channel group (measurement data collection).
///
/// A channel group represents a collection of channels that share the same
/// time base and record structure. All channels in a group have synchronized
/// samples stored together in fixed-size records.
///
/// # Record Structure
///
/// Each record has the following layout:
/// ```text
/// [Record ID (0-8 bytes)] [Channel Data (record_size bytes)] [Invalidation (invalidation_bytes bytes)]
/// ```
///
/// The total record size is: `record_id_size + record_size + invalidation_bytes`
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IndexedChannelGroup {
    /// Group name (e.g., "CAN1", "EngineData", "GPS")
    pub name: Option<String>,
    /// Group description or comment
    pub comment: Option<String>,
    /// Size of record ID prefix in bytes (0, 1, 2, 4, or 8)
    pub record_id_size: u8,
    /// Size of channel data portion in each record (bytes)
    pub record_size: u32,
    /// Size of invalidation bytes at end of each record
    pub invalidation_bytes: u32,
    /// Total number of records (samples) in this group
    pub record_count: u64,
    /// Channels belonging to this group
    pub channels: Vec<IndexedChannel>,
    /// Data block locations containing this group's records
    pub data_blocks: Vec<DataBlockInfo>,
}
