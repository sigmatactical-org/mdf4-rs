//! [`FlushState`].

#[allow(unused_imports)]
use super::*;

/// Tracks flush state for streaming writes.
#[derive(Debug, Default)]
pub(crate) struct FlushState {
    /// Records written since last flush.
    pub records_since_flush: u64,
    /// Bytes written since last flush.
    pub bytes_since_flush: u64,
    /// Total records written.
    pub total_records: u64,
    /// Total bytes written.
    pub total_bytes: u64,
    /// Number of flushes performed.
    pub flush_count: u64,
}
impl FlushState {
    /// Record that data was written.
    pub fn record_write(&mut self, records: u64, bytes: u64) {
        self.records_since_flush += records;
        self.bytes_since_flush += bytes;
        self.total_records += records;
        self.total_bytes += bytes;
    }

    /// Check if a flush should be triggered based on the policy.
    pub fn should_flush(&self, policy: &FlushPolicy) -> bool {
        match policy {
            FlushPolicy::Manual => false,
            FlushPolicy::EveryNRecords(n) => self.records_since_flush >= *n,
            FlushPolicy::EveryNBytes(n) => self.bytes_since_flush >= *n,
        }
    }

    /// Reset counters after a flush.
    pub fn on_flush(&mut self) {
        self.records_since_flush = 0;
        self.bytes_since_flush = 0;
        self.flush_count += 1;
    }
}
