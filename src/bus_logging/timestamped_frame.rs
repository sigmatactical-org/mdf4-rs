//! [`TimestampedFrame`].

#[allow(unused_imports)]
use super::*;

/// A buffered frame entry with timestamp.
pub struct TimestampedFrame<F> {
    /// Timestamp in seconds (ASAM uses float64 seconds).
    pub timestamp_s: f64,
    /// The frame data.
    pub frame: F,
}
impl<F> TimestampedFrame<F> {
    /// Create a new timestamped frame from microseconds.
    #[inline]
    pub fn new(timestamp_us: u64, frame: F) -> Self {
        Self {
            timestamp_s: timestamp_to_seconds(timestamp_us),
            frame,
        }
    }
}
