//! [`VecWriter`].

#[allow(unused_imports)]
use super::*;
use crate::Result;

/// A writer that writes to an in-memory buffer.
///
/// This is available in both `std` and `no_std` environments, making it useful
/// for creating MDF data in memory before writing to external storage.
pub struct VecWriter {
    buffer: alloc::vec::Vec<u8>,
    position: u64,
}
impl VecWriter {
    /// Create a new VecWriter with an empty buffer.
    pub fn new() -> Self {
        Self {
            buffer: alloc::vec::Vec::new(),
            position: 0,
        }
    }

    /// Create a new VecWriter with the given initial capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buffer: alloc::vec::Vec::with_capacity(capacity),
            position: 0,
        }
    }

    /// Consume the writer and return the underlying buffer.
    pub fn into_inner(self) -> alloc::vec::Vec<u8> {
        self.buffer
    }

    /// Get a reference to the underlying buffer.
    pub fn as_slice(&self) -> &[u8] {
        &self.buffer
    }

    /// Get the total length of the written data.
    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    /// Check if the buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }
}
impl Default for VecWriter {
    fn default() -> Self {
        Self::new()
    }
}
impl MdfWrite for VecWriter {
    fn write_all(&mut self, bytes: &[u8]) -> Result<()> {
        let pos = self.position as usize;
        let end = pos + bytes.len();

        // Extend buffer if needed
        if end > self.buffer.len() {
            self.buffer.resize(end, 0);
        }

        // Copy bytes to buffer
        self.buffer[pos..end].copy_from_slice(bytes);
        self.position = end as u64;
        Ok(())
    }

    fn seek(&mut self, pos: u64) -> Result<u64> {
        self.position = pos;
        Ok(self.position)
    }

    fn position(&self) -> u64 {
        self.position
    }

    fn flush(&mut self) -> Result<()> {
        // No-op for in-memory buffer
        Ok(())
    }
}
