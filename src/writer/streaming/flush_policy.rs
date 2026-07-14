//! [`FlushPolicy`].

#[allow(unused_imports)]
use super::*;

/// Policy for automatic flushing of MDF4 data during streaming writes.
///
/// When a flush policy is set, the writer will automatically flush buffered
/// data to disk based on the policy criteria. This is essential for long-running
/// captures where keeping all data in memory is not feasible.
///
/// # Flush Behavior
///
/// When a flush is triggered:
/// 1. All buffered record data is written to the underlying I/O
/// 2. DT block size links are updated
/// 3. The I/O buffer is flushed to disk
///
/// The file remains in a valid state after each flush, with proper DT block
/// sizes recorded. Final DL (Data List) blocks are created during finalization.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum FlushPolicy {
    /// Never auto-flush. Data is only flushed on explicit `flush()` or `finalize()` calls.
    /// This is the default behavior.
    #[default]
    Manual,

    /// Flush after every N records written across all channel groups.
    ///
    /// This is useful when you want predictable flush intervals based on
    /// the number of data points captured.
    ///
    /// # Example
    /// ```ignore
    /// // Flush every 1000 records
    /// FlushPolicy::EveryNRecords(1000)
    /// ```
    EveryNRecords(u64),

    /// Flush after N bytes of record data have been written.
    ///
    /// This is useful when you want to limit memory usage to a specific
    /// amount regardless of record size.
    ///
    /// # Example
    /// ```ignore
    /// // Flush every 1 MB of data
    /// FlushPolicy::EveryNBytes(1024 * 1024)
    /// ```
    EveryNBytes(u64),
}
impl FlushPolicy {
    /// Check if this policy requires automatic flushing.
    pub fn is_auto(&self) -> bool {
        !matches!(self, FlushPolicy::Manual)
    }
}
