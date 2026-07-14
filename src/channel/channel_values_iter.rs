//! [`ChannelValuesIter`].

#[allow(unused_imports)]
use super::*;
use crate::{
    Result,
    blocks::ChannelBlock,
    parsing::decoder::{DecodedValue, decode_channel_value_with_validity},
};

/// Streaming iterator over decoded channel values.
///
/// Created by [`Channel::iter_values()`]. This iterator decodes values on-demand
/// without loading all data into memory, making it suitable for large MDF4 files.
pub struct ChannelValuesIter<'a> {
    pub(crate) records_iter: Box<dyn Iterator<Item = Result<&'a [u8]>> + 'a>,
    pub(crate) block: &'a ChannelBlock,
    pub(crate) mmap: &'a [u8],
    pub(crate) record_id_size: usize,
    pub(crate) cg_data_bytes: u32,
}
impl<'a> Iterator for ChannelValuesIter<'a> {
    type Item = Result<Option<DecodedValue>>;

    fn next(&mut self) -> Option<Self::Item> {
        let rec_result = self.records_iter.next()?;

        Some(match rec_result {
            Ok(rec) => {
                // Decode with validity checking
                if let Some(decoded) = decode_channel_value_with_validity(
                    rec,
                    self.record_id_size,
                    self.cg_data_bytes,
                    self.block,
                ) {
                    if decoded.is_valid {
                        // Value is valid, apply conversion
                        match self.block.apply_conversion_value(decoded.value, self.mmap) {
                            Ok(phys) => Ok(Some(phys)),
                            Err(e) => Err(e),
                        }
                    } else {
                        // Value is invalid according to invalidation bit
                        Ok(None)
                    }
                } else {
                    // Decoding failed
                    Ok(None)
                }
            }
            Err(e) => Err(e),
        })
    }
}
