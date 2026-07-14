//! [`DataBlockData`].

#[allow(unused_imports)]
use super::*;
#[cfg(feature = "compression")]
use crate::blocks::DzBlock;

/// Either a reference to existing data or owned decompressed data.
#[derive(Debug)]
pub enum DataBlockData<'a> {
    /// Reference to memory-mapped data (uncompressed DT/DV blocks).
    Borrowed(&'a [u8]),
    /// Owned decompressed data (from DZ blocks).
    #[cfg(feature = "compression")]
    Owned(Vec<u8>),
}
impl<'a> DataBlockData<'a> {
    /// Get the data as a slice.
    pub fn as_slice(&self) -> &[u8] {
        match self {
            DataBlockData::Borrowed(s) => s,
            #[cfg(feature = "compression")]
            DataBlockData::Owned(v) => v.as_slice(),
        }
    }

    /// Get the length of the data.
    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    /// Check if the data is empty.
    pub fn is_empty(&self) -> bool {
        self.as_slice().is_empty()
    }
}
