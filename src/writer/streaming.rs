//! Streaming write configuration for long-running MDF4 captures.
//!
//! This module provides [`FlushPolicy`] for configuring automatic flushing
//! of MDF4 data during capture, enabling memory-efficient logging of long
//! recordings.
//!
//! # Use Cases
//!
//! - Hours-long vehicle data logging without running out of memory
//! - Crash-safe logging where partial files remain valid MDF4
//! - Embedded systems with limited RAM
//!
//! # Example
//!
//! ```ignore
//! use mdf4_rs::{MdfWriter, FlushPolicy};
//!
//! let mut writer = MdfWriter::new("output.mf4")?
//!     .with_flush_policy(FlushPolicy::EveryNRecords(1000));
//!
//! // Records are automatically flushed to disk every 1000 records
//! for i in 0..10000 {
//!     writer.write_record(&cg_id, &values)?;
//! }
//! ```

mod flush_policy;
mod flush_state;
pub use flush_policy::FlushPolicy;
pub(crate) use flush_state::FlushState;

/// Configuration for streaming MDF4 writes.
#[derive(Debug, Clone, Default)]
pub struct StreamingConfig {
    /// The flush policy to use.
    pub policy: FlushPolicy,
}

impl StreamingConfig {
    /// Create a new streaming configuration with manual flush policy.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a streaming configuration that flushes every N records.
    pub fn every_n_records(n: u64) -> Self {
        Self {
            policy: FlushPolicy::EveryNRecords(n),
        }
    }

    /// Create a streaming configuration that flushes every N bytes.
    pub fn every_n_bytes(n: u64) -> Self {
        Self {
            policy: FlushPolicy::EveryNBytes(n),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flush_policy_default() {
        assert_eq!(FlushPolicy::default(), FlushPolicy::Manual);
    }

    #[test]
    fn test_flush_policy_is_auto() {
        assert!(!FlushPolicy::Manual.is_auto());
        assert!(FlushPolicy::EveryNRecords(100).is_auto());
        assert!(FlushPolicy::EveryNBytes(1024).is_auto());
    }

    #[test]
    fn test_flush_state_should_flush() {
        let mut state = FlushState::default();

        // Manual policy never triggers
        state.record_write(1000, 10000);
        assert!(!state.should_flush(&FlushPolicy::Manual));

        // EveryNRecords triggers at threshold
        assert!(!state.should_flush(&FlushPolicy::EveryNRecords(1001)));
        assert!(state.should_flush(&FlushPolicy::EveryNRecords(1000)));
        assert!(state.should_flush(&FlushPolicy::EveryNRecords(500)));

        // EveryNBytes triggers at threshold
        assert!(!state.should_flush(&FlushPolicy::EveryNBytes(10001)));
        assert!(state.should_flush(&FlushPolicy::EveryNBytes(10000)));
        assert!(state.should_flush(&FlushPolicy::EveryNBytes(5000)));
    }

    #[test]
    fn test_flush_state_reset() {
        let mut state = FlushState::default();
        state.record_write(100, 1000);
        assert_eq!(state.records_since_flush, 100);
        assert_eq!(state.bytes_since_flush, 1000);

        state.on_flush();
        assert_eq!(state.records_since_flush, 0);
        assert_eq!(state.bytes_since_flush, 0);
        assert_eq!(state.total_records, 100); // Total preserved
        assert_eq!(state.total_bytes, 1000);
        assert_eq!(state.flush_count, 1);
    }

    #[test]
    fn test_streaming_config_constructors() {
        let config = StreamingConfig::every_n_records(500);
        assert_eq!(config.policy, FlushPolicy::EveryNRecords(500));

        let config = StreamingConfig::every_n_bytes(1024);
        assert_eq!(config.policy, FlushPolicy::EveryNBytes(1024));
    }
}
