//! [`ByteRangeReader`].

#[allow(unused_imports)]
use super::*;

/// Trait for reading arbitrary byte ranges from a data source.
///
/// This trait abstracts the data source, allowing the index system to work
/// with local files, HTTP resources, cloud storage, or any other source
/// that supports random access.
///
/// # Implementing Custom Readers
///
/// ```ignore
/// use mdf4_rs::index::ByteRangeReader;
///
/// struct HttpRangeReader {
///     url: String,
///     client: reqwest::blocking::Client,
/// }
///
/// impl ByteRangeReader for HttpRangeReader {
///     type Error = mdf4_rs::Error;
///
///     fn read_range(&mut self, offset: u64, length: u64) -> Result<Vec<u8>, Self::Error> {
///         let end = offset + length - 1;
///         let response = self.client
///             .get(&self.url)
///             .header("Range", format!("bytes={}-{}", offset, end))
///             .send()
///             .map_err(|e| mdf4_rs::Error::BlockSerializationError(e.to_string()))?;
///         response.bytes()
///             .map(|b| b.to_vec())
///             .map_err(|e| mdf4_rs::Error::BlockSerializationError(e.to_string()))
///     }
/// }
/// ```
pub trait ByteRangeReader {
    /// Error type returned by read operations
    type Error;

    /// Read `length` bytes starting at `offset`.
    ///
    /// # Arguments
    /// * `offset` - Byte offset from the start of the data source
    /// * `length` - Number of bytes to read
    ///
    /// # Returns
    /// The requested bytes, or an error if the read fails.
    fn read_range(
        &mut self,
        offset: u64,
        length: u64,
    ) -> core::result::Result<Vec<u8>, Self::Error>;
}
