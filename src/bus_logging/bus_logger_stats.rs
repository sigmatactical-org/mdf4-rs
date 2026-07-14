//! [`BusLoggerStats`].

#[allow(unused_imports)]
use super::*;

/// Statistics for a bus logger.
#[derive(Debug, Clone, Default)]
pub struct BusLoggerStats {
    /// Total number of frames logged.
    pub total_frames: usize,
    /// Number of transmitted frames.
    pub tx_frames: usize,
    /// Number of received frames.
    pub rx_frames: usize,
    /// Number of error frames.
    pub error_frames: usize,
}
