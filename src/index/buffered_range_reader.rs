//! [`BufferedRangeReader`].

#[allow(unused_imports)]
use super::*;
use crate::{Error, Result};
use std::io::{Read, Seek, SeekFrom};

/// Buffered file reader with read-ahead caching for better I/O performance.
///
/// This reader maintains an internal buffer and prefetches data to minimize
/// system calls when reading many small ranges sequentially.
pub struct BufferedRangeReader {
    file: std::fs::File,
    buffer: Vec<u8>,
    buffer_start: u64,
    buffer_end: u64,
    buffer_capacity: usize,
}
impl BufferedRangeReader {
    /// Create a new buffered reader with the default buffer size (64 KB).
    pub fn new(file_path: &str) -> Result<Self> {
        Self::with_capacity(file_path, 64 * 1024)
    }

    /// Create a new buffered reader with a custom buffer size.
    pub fn with_capacity(file_path: &str, capacity: usize) -> Result<Self> {
        let file = std::fs::File::open(file_path).map_err(Error::IOError)?;
        Ok(Self {
            file,
            buffer: Vec::with_capacity(capacity),
            buffer_start: 0,
            buffer_end: 0,
            buffer_capacity: capacity,
        })
    }

    /// Fill the internal buffer starting at the given offset.
    fn fill_buffer(&mut self, offset: u64) -> Result<()> {
        self.file
            .seek(SeekFrom::Start(offset))
            .map_err(Error::IOError)?;

        self.buffer.clear();
        self.buffer.resize(self.buffer_capacity, 0);

        let bytes_read = self.file.read(&mut self.buffer).map_err(Error::IOError)?;
        self.buffer.truncate(bytes_read);
        self.buffer_start = offset;
        self.buffer_end = offset + bytes_read as u64;

        Ok(())
    }
}
impl ByteRangeReader for BufferedRangeReader {
    type Error = Error;

    fn read_range(
        &mut self,
        offset: u64,
        length: u64,
    ) -> core::result::Result<Vec<u8>, Self::Error> {
        let end = offset + length;

        // Check if the requested range is fully within the buffer
        if offset >= self.buffer_start && end <= self.buffer_end {
            let start_idx = (offset - self.buffer_start) as usize;
            let end_idx = start_idx + length as usize;
            return Ok(self.buffer[start_idx..end_idx].to_vec());
        }

        // If the request is larger than our buffer, read directly
        if length as usize > self.buffer_capacity {
            self.file
                .seek(SeekFrom::Start(offset))
                .map_err(Error::IOError)?;
            let mut buffer = vec![0u8; length as usize];
            self.file.read_exact(&mut buffer).map_err(Error::IOError)?;
            return Ok(buffer);
        }

        // Fill buffer starting at the requested offset
        self.fill_buffer(offset)?;

        // Now read from buffer
        if end <= self.buffer_end {
            let start_idx = (offset - self.buffer_start) as usize;
            let end_idx = start_idx + length as usize;
            Ok(self.buffer[start_idx..end_idx].to_vec())
        } else {
            // Buffer didn't have enough data (near end of file)
            Err(Error::TooShortBuffer {
                actual: (self.buffer_end - offset) as usize,
                expected: length as usize,
                file: file!(),
                line: line!(),
            })
        }
    }
}
