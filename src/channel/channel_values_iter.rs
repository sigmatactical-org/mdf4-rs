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

#[cfg(test)]
mod tests {
    use super::ChannelValuesIter;
    use crate::blocks::{BlockHeader, BlockParse, ChannelBlock, DataType};
    use crate::{Error, Result};

    fn channel_block() -> ChannelBlock {
        let header = BlockHeader {
            id: "##CN".into(),
            reserved: 0,
            length: 160,
            link_count: 8,
        };
        let mut bytes = header.to_bytes().unwrap();
        bytes.resize(160, 0);
        let mut block = ChannelBlock::from_bytes(&bytes).unwrap();
        block.data_type = DataType::UnsignedIntegerLE;
        block.bit_count = 8;
        block
    }

    fn iter<'a>(
        records: impl Iterator<Item = Result<&'a [u8]>> + 'a,
        block: &'a ChannelBlock,
        mmap: &'a [u8],
    ) -> ChannelValuesIter<'a> {
        ChannelValuesIter {
            records_iter: Box::new(records),
            block,
            mmap,
            record_id_size: 0,
            cg_data_bytes: 1,
        }
    }

    #[test]
    fn decodes_valid_records_and_maps_invalid_or_undecodable_records_to_none() {
        let block = channel_block();
        let records = [&[42][..], &[][..], &[0][..]].into_iter().map(Ok);
        let mut iterator = iter(records, &block, &[]);
        assert_eq!(
            iterator.next().unwrap().unwrap(),
            Some(crate::types::DecodedValue::UnsignedInteger(42))
        );
        assert_eq!(iterator.next().unwrap().unwrap(), None);
        assert_eq!(
            iterator.next().unwrap().unwrap(),
            Some(crate::types::DecodedValue::UnsignedInteger(0))
        );
        assert!(iterator.next().is_none());
    }

    #[test]
    fn returns_invalid_samples_as_none_and_propagates_record_errors() {
        let mut block = channel_block();
        block.flags = 1;
        let records = [&[42][..]].into_iter().map(Ok);
        assert_eq!(iter(records, &block, &[]).next().unwrap().unwrap(), None);

        block.flags = 0;
        let records = [Err(Error::BlockSerializationError("bad record".into()))].into_iter();
        assert!(iter(records, &block, &[]).next().unwrap().is_err());
    }

    #[test]
    fn propagates_conversion_errors() {
        let mut block = channel_block();
        let mut conversion = crate::blocks::ConversionBlock::identity();
        conversion.conversion_type = crate::blocks::ConversionType::ValueToText;
        conversion.values = vec![1.0];
        conversion.refs = vec![24];
        block.conversion = Some(conversion);

        let mut mmap = vec![0; 48];
        mmap[24..28].copy_from_slice(b"##TX");
        mmap[32..40].copy_from_slice(&128u64.to_le_bytes());
        let records = [&[1][..]].into_iter().map(Ok);
        assert!(iter(records, &block, &mmap).next().unwrap().is_err());
    }
}
