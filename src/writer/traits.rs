//! Abstract I/O traits for the MDF writer.
//!
//! This module provides traits that abstract away the underlying I/O implementation,
//! allowing the writer to work with both file-based I/O (with `std`) and in-memory
//! buffers (with just `alloc`).

mod vec_writer;
pub use vec_writer::VecWriter;

use crate::Result;

/// Trait for write operations used by MdfWriter.
///
/// This trait abstracts the write and seek operations needed for MDF file creation.
/// It is implemented for standard file-based I/O when the `std` feature is enabled,
/// and for in-memory buffers in `no_std` environments.
pub trait MdfWrite {
    /// Write all bytes to the destination.
    fn write_all(&mut self, bytes: &[u8]) -> Result<()>;

    /// Seek to an absolute position.
    fn seek(&mut self, pos: u64) -> Result<u64>;

    /// Get the current position.
    fn position(&self) -> u64;

    /// Flush any buffered data.
    fn flush(&mut self) -> Result<()>;
}

#[cfg(feature = "std")]
mod std_impl {
    use super::MdfWrite;
    use crate::Result;
    use std::fs::File;
    use std::io::{BufWriter, Seek, SeekFrom, Write};

    /// A wrapper that implements MdfWrite for standard file I/O.
    pub struct FileWriter {
        inner: BufWriter<File>,
        position: u64,
    }

    impl FileWriter {
        /// Create a new FileWriter for the given file path.
        pub fn new(path: &str) -> Result<Self> {
            Self::with_capacity(path, 1_048_576) // 1 MB default buffer
        }

        /// Create a new FileWriter with the specified buffer capacity.
        pub fn with_capacity(path: &str, capacity: usize) -> Result<Self> {
            let file = File::create(path)?;
            let inner = BufWriter::with_capacity(capacity, file);
            Ok(Self { inner, position: 0 })
        }
    }

    impl MdfWrite for FileWriter {
        fn write_all(&mut self, bytes: &[u8]) -> Result<()> {
            self.inner.write_all(bytes)?;
            self.position += bytes.len() as u64;
            Ok(())
        }

        fn seek(&mut self, pos: u64) -> Result<u64> {
            self.inner.seek(SeekFrom::Start(pos))?;
            self.position = pos;
            Ok(self.position)
        }

        fn position(&self) -> u64 {
            self.position
        }

        fn flush(&mut self) -> Result<()> {
            self.inner.flush()?;
            Ok(())
        }
    }
}

#[cfg(feature = "std")]
pub use std_impl::FileWriter;
