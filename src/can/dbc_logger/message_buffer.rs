//! [`MessageBuffer`].

use super::super::dbc_compat::SignalInfo;
#[allow(unused_imports)]
use super::*;
use alloc::vec::Vec;

/// Buffer for a single message's decoded data.
#[derive(Debug)]
pub(crate) struct MessageBuffer {
    /// Signal information extracted from DBC
    pub(crate) signals: Vec<SignalInfo>,
    /// Mapping from buffer signal index to message signal index
    /// Used for zero-alloc decode: decode_buf[msg_idx] -> buffer signal
    pub(crate) signal_indices: Vec<usize>,
    /// Timestamps for each frame (microseconds)
    pub(crate) timestamps: Vec<u64>,
    /// Raw values per signal (outer vec = signals, inner vec = samples)
    pub(crate) raw_values: Vec<Vec<i64>>,
    /// Physical values per signal (outer vec = signals, inner vec = samples)
    pub(crate) physical_values: Vec<Vec<f64>>,
}
impl MessageBuffer {
    /// Buffer for one DBC message's signal columns.
    pub(crate) fn new(signals: Vec<SignalInfo>, signal_indices: Vec<usize>) -> Self {
        let num_signals = signals.len();
        Self {
            signals,
            signal_indices,
            timestamps: Vec::new(),
            raw_values: (0..num_signals).map(|_| Vec::new()).collect(),
            physical_values: (0..num_signals).map(|_| Vec::new()).collect(),
        }
    }

    /// Append physical samples straight from the shared decode buffer, selecting
    /// this buffer's signals via `signal_indices`. Keeps the hot path allocation-free
    /// (no intermediate `Vec` per frame).
    pub(crate) fn push_physical_indexed(&mut self, timestamp_us: u64, decode_buf: &[f64]) {
        self.timestamps.push(timestamp_us);
        for (out, &idx) in self
            .physical_values
            .iter_mut()
            .zip(self.signal_indices.iter())
        {
            out.push(decode_buf.get(idx).copied().unwrap_or(0.0));
        }
    }

    /// Append raw samples straight from the shared decode buffer (see
    /// [`Self::push_physical_indexed`]).
    pub(crate) fn push_raw_indexed(&mut self, timestamp_us: u64, decode_buf: &[i64]) {
        self.timestamps.push(timestamp_us);
        for (out, &idx) in self.raw_values.iter_mut().zip(self.signal_indices.iter()) {
            out.push(decode_buf.get(idx).copied().unwrap_or(0));
        }
    }

    /// Drop all buffered rows.
    pub(crate) fn clear(&mut self) {
        self.timestamps.clear();
        for v in &mut self.raw_values {
            v.clear();
        }
        for v in &mut self.physical_values {
            v.clear();
        }
    }

    /// Number of buffered frames.
    pub(crate) fn frame_count(&self) -> usize {
        self.timestamps.len()
    }
}
