//! [`FileRangeReader`].

#[allow(unused_imports)]
use super::*;
use crate::{Error, Result};
use std::io::{Read, Seek, SeekFrom};

/// Simple file reader that seeks and reads for each request.
///
/// This reader has minimal memory overhead but may have higher I/O latency
/// when reading many small ranges. For sequential access patterns, consider
/// using [`BufferedRangeReader`] instead.
///
/// # Example
///
/// ```no_run
/// use mdf4_rs::{MdfIndex, FileRangeReader};
///
/// let index = MdfIndex::from_file_streaming("data.mf4")?;
/// let mut reader = FileRangeReader::new("data.mf4")?;
/// let values = index.read_channel_values(0, 0, &mut reader)?;
/// # Ok::<(), mdf4_rs::Error>(())
/// ```
pub struct FileRangeReader {
    file: std::fs::File,
}
impl FileRangeReader {
    /// Open a file for range reading.
    ///
    /// # Arguments
    /// * `file_path` - Path to the file
    ///
    /// # Errors
    /// Returns an error if the file cannot be opened.
    pub fn new(file_path: &str) -> Result<Self> {
        let file = std::fs::File::open(file_path).map_err(Error::IOError)?;
        Ok(Self { file })
    }
}
impl ByteRangeReader for FileRangeReader {
    type Error = Error;

    fn read_range(
        &mut self,
        offset: u64,
        length: u64,
    ) -> core::result::Result<Vec<u8>, Self::Error> {
        self.file
            .seek(SeekFrom::Start(offset))
            .map_err(Error::IOError)?;

        let mut buffer = vec![0u8; length as usize];
        self.file.read_exact(&mut buffer).map_err(Error::IOError)?;

        Ok(buffer)
    }
}
