//! [`ResolvedDataBlock`].

#[allow(unused_imports)]
use super::*;
#[cfg(feature = "compression")]
use crate::blocks::DzBlock;

/// A data block that may contain borrowed or owned data.
///
/// This allows handling both regular DT/DV blocks (zero-copy) and
/// decompressed DZ blocks (owned data) with a unified interface.
#[derive(Debug)]
pub struct ResolvedDataBlock<'a> {
    /// Original block type ID (e.g., "##DT", "##DV").
    pub block_id: &'static str,
    /// The data contents (may be borrowed or owned).
    pub data: DataBlockData<'a>,
}
impl<'a> ResolvedDataBlock<'a> {
    /// Iterate over raw records of fixed size.
    ///
    /// # Arguments
    /// * `record_size` - Size in bytes of one record (including record ID)
    ///
    /// # Returns
    /// An iterator yielding each raw record slice.
    pub fn records(&self, record_size: usize) -> impl Iterator<Item = &[u8]> {
        self.data.as_slice().chunks_exact(record_size)
    }
}
