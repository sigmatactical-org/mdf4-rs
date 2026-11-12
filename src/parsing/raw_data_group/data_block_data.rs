//! [`DataBlockData`].

#[allow(unused_imports)]
use super::*;

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

#[cfg(test)]
mod tests {
    use super::DataBlockData;

    #[test]
    fn borrowed_data_exposes_slice_length_and_empty_state() {
        let data = DataBlockData::Borrowed(&[1, 2, 3]);
        assert_eq!(data.as_slice(), &[1, 2, 3]);
        assert_eq!(data.len(), 3);
        assert!(!data.is_empty());

        let empty = DataBlockData::Borrowed(&[]);
        assert_eq!(empty.len(), 0);
        assert!(empty.is_empty());
    }

    #[cfg(feature = "compression")]
    #[test]
    fn owned_data_exposes_slice_length_and_empty_state() {
        let data = DataBlockData::Owned(vec![4, 5]);
        assert_eq!(data.as_slice(), &[4, 5]);
        assert_eq!(data.len(), 2);
        assert!(!data.is_empty());

        let empty = DataBlockData::Owned(Vec::new());
        assert!(empty.is_empty());
    }
}
