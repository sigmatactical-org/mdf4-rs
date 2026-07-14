//! [`OverlayStatistics`].

#[allow(unused_imports)]
use super::*;

/// Statistics about a raw CAN capture with DBC overlay.
#[derive(Debug, Clone)]
pub struct OverlayStatistics {
    /// Total number of CAN frames in the capture
    pub total_frames: usize,
    /// Number of unique CAN IDs
    pub unique_can_ids: usize,
    /// Number of DBC messages that have data in this capture
    pub dbc_messages_found: usize,
    /// Total number of messages defined in the DBC
    pub dbc_messages_total: usize,
    /// Earliest timestamp in microseconds
    pub min_timestamp_us: u64,
    /// Latest timestamp in microseconds
    pub max_timestamp_us: u64,
    /// Duration of capture in microseconds
    pub duration_us: u64,
}
