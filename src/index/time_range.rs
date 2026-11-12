use super::{ByteRangeReader, IndexedChannel, MdfIndex};
use crate::blocks::{BlockHeader, ChannelBlock};
#[cfg(feature = "compression")]
use crate::blocks::{BlockParse, DzBlock};
use crate::parsing::decoder::decode_channel_value_with_validity;
use crate::{Error, Result};

/// Selected timestamps and channel values from one indexed group.
pub struct TimeRangeData {
    /// Timestamps in record order.
    pub times: Vec<f64>,
    /// Values in the same order as the requested channel indices.
    pub values: Vec<Vec<Option<f64>>>,
}

pub(super) fn channel_block(channel: &IndexedChannel) -> ChannelBlock {
    ChannelBlock {
        header: BlockHeader {
            id: "##CN".into(),
            reserved: 0,
            length: 160,
            link_count: 8,
        },
        next_ch_addr: 0,
        component_addr: channel.component_addr,
        name_addr: 0,
        source_addr: 0,
        conversion_addr: 0,
        data_addr: 0,
        unit_addr: 0,
        comment_addr: 0,
        channel_type: channel.channel_type,
        sync_type: 0,
        data_type: channel.data_type,
        bit_offset: channel.bit_offset,
        byte_offset: channel.byte_offset,
        bit_count: channel.bit_count,
        flags: channel.flags,
        pos_invalidation_bit: channel.pos_invalidation_bit,
        precision: 0,
        reserved1: 0,
        attachment_count: 0,
        min_raw_value: 0.0,
        max_raw_value: 0.0,
        lower_limit: 0.0,
        upper_limit: 0.0,
        lower_ext_limit: 0.0,
        upper_ext_limit: 0.0,
        name: channel.name.clone(),
        conversion: channel.conversion.clone(),
    }
}

pub(super) fn decode(
    record: &[u8],
    record_size: u32,
    channel: &ChannelBlock,
) -> Result<Option<f64>> {
    match decode_channel_value_with_validity(record, 0, record_size, channel) {
        Some(decoded) if decoded.is_valid => {
            let value = match &channel.conversion {
                Some(conversion) => conversion.apply_decoded(decoded.value, &[])?,
                None => decoded.value,
            };
            Ok(value.as_f64())
        }
        _ => Ok(Some(f64::NAN)),
    }
}

impl MdfIndex {
    /// Read selected numeric channels within an inclusive timestamp range.
    ///
    /// This streaming reader supports groups without record IDs. It scans time
    /// values in file order, reading uncompressed data in bounded chunks and
    /// decompressing at most one compressed block at a time. Groups without a
    /// time channel use their original record indices as timestamps.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use mdf4_rs::index::{FileRangeReader, MdfIndex};
    ///
    /// let index = MdfIndex::from_file("measurements.mf4")?;
    /// let mut reader = FileRangeReader::new("measurements.mf4")?;
    /// let data = index.read_group_time_range(
    ///     0,
    ///     &[1],
    ///     Some(0),
    ///     Some(1.0),
    ///     Some(2.0),
    ///     &mut reader,
    /// )?;
    /// # Ok::<(), mdf4_rs::Error>(())
    /// ```
    pub fn read_group_time_range<R: ByteRangeReader<Error = Error>>(
        &self,
        group_index: usize,
        channel_indices: &[usize],
        time_index: Option<usize>,
        start: Option<f64>,
        end: Option<f64>,
        reader: &mut R,
    ) -> Result<TimeRangeData> {
        let invalid = |message: &str| Error::BlockSerializationError(message.to_owned());
        if start.is_some_and(f64::is_nan)
            || end.is_some_and(f64::is_nan)
            || matches!((start, end), (Some(start), Some(end)) if start > end)
        {
            return Err(invalid("Invalid time range"));
        }
        let group = self
            .channel_groups
            .get(group_index)
            .ok_or_else(|| invalid("Invalid group index"))?;
        if group.record_id_size != 0 {
            return Err(invalid("Time-range streaming does not support record IDs"));
        }
        let record_size = usize::try_from(group.record_size)
            .ok()
            .and_then(|size| size.checked_add(group.invalidation_bytes as usize))
            .filter(|&size| size != 0)
            .ok_or_else(|| invalid("Invalid MDF record size"))?;
        let record_size_u64 =
            u64::try_from(record_size).map_err(|_| invalid("MDF record size overflow"))?;
        let channels = channel_indices
            .iter()
            .map(|&index| {
                group
                    .channels
                    .get(index)
                    .map(channel_block)
                    .ok_or_else(|| invalid("Invalid channel index"))
            })
            .collect::<Result<Vec<_>>>()?;
        let time_channel = time_index
            .map(|index| {
                group
                    .channels
                    .get(index)
                    .map(channel_block)
                    .ok_or_else(|| invalid("Invalid time channel index"))
            })
            .transpose()?;
        let first_record = if time_index.is_none() {
            start.map_or(0, |bound| bound.ceil().max(0.0) as u64)
        } else {
            0
        };
        let last_record = if time_index.is_none() {
            end.map_or(u64::MAX, |bound| {
                if bound < 0.0 {
                    0
                } else {
                    (bound.floor() as u64).saturating_add(1)
                }
            })
        } else {
            u64::MAX
        };
        let first_byte = first_record.saturating_mul(record_size_u64);
        let last_byte = last_record.saturating_mul(record_size_u64);
        let mut result = TimeRangeData {
            times: Vec::new(),
            values: vec![Vec::new(); channels.len()],
        };
        let mut pending = Vec::new();
        let mut record_index = first_record;
        let mut process_record = |record: &[u8]| -> Result<()> {
            let time = match &time_channel {
                Some(channel) => decode(record, group.record_size, channel)?
                    .ok_or_else(|| invalid("MDF time channel is not numeric"))?,
                None => record_index as f64,
            };
            record_index = record_index.saturating_add(1);
            if (start.is_some() || end.is_some())
                && (time.is_nan()
                    || start.is_some_and(|bound| time < bound)
                    || end.is_some_and(|bound| time > bound))
            {
                return Ok(());
            }
            result.times.push(time);
            for (column, channel) in result.values.iter_mut().zip(&channels) {
                column.push(decode(record, group.record_size, channel)?);
            }
            Ok(())
        };
        let mut process = |mut bytes: &[u8]| -> Result<()> {
            if !pending.is_empty() {
                let count = (record_size - pending.len()).min(bytes.len());
                pending.extend_from_slice(&bytes[..count]);
                bytes = &bytes[count..];
                if pending.len() == record_size {
                    process_record(&pending)?;
                    pending.clear();
                }
            }
            let mut records = bytes.chunks_exact(record_size);
            for record in &mut records {
                process_record(record)?;
            }
            pending.extend_from_slice(records.remainder());
            Ok(())
        };
        let mut stream_offset = 0u64;
        for block in &group.data_blocks {
            if block.is_compressed {
                #[cfg(feature = "compression")]
                {
                    if block.size < 48 {
                        return Err(invalid("Invalid compressed block size"));
                    }
                    let header = reader.read_range(block.file_offset, 48)?;
                    if header.len() != 48 {
                        return Err(invalid("Truncated compressed block header"));
                    }
                    let original_length = u64::from_le_bytes(
                        header[32..40]
                            .try_into()
                            .map_err(|_| invalid("Invalid compressed block header"))?,
                    );
                    let block_end = stream_offset
                        .checked_add(original_length)
                        .ok_or_else(|| invalid("Data size overflow"))?;
                    let start_offset = first_byte.max(stream_offset);
                    let end_offset = last_byte.min(block_end);
                    if start_offset < end_offset {
                        let bytes = reader.read_range(block.file_offset, block.size)?;
                        let data = DzBlock::from_bytes(&bytes)?.decompress()?;
                        let start = usize::try_from(start_offset - stream_offset)
                            .map_err(|_| invalid("Compressed data offset overflow"))?;
                        let end = usize::try_from(end_offset - stream_offset)
                            .map_err(|_| invalid("Compressed data offset overflow"))?;
                        process(data.get(start..end).ok_or_else(|| {
                            invalid("Compressed data length does not match its header")
                        })?)?;
                    }
                    stream_offset = block_end;
                }
                #[cfg(not(feature = "compression"))]
                return Err(invalid("Compressed blocks require the compression feature"));
            } else {
                let length = block
                    .size
                    .checked_sub(24)
                    .ok_or_else(|| invalid("Invalid data block size"))?;
                let block_end = stream_offset
                    .checked_add(length)
                    .ok_or_else(|| invalid("Data size overflow"))?;
                let start_offset = first_byte.max(stream_offset);
                let end_offset = last_byte.min(block_end);
                if start_offset < end_offset {
                    let mut offset = block
                        .file_offset
                        .checked_add(24)
                        .and_then(|offset| offset.checked_add(start_offset - stream_offset))
                        .ok_or_else(|| invalid("Invalid block offset"))?;
                    let mut remaining = end_offset - start_offset;
                    while remaining > 0 {
                        let length = remaining.min(1024 * 1024);
                        let bytes = reader.read_range(offset, length)?;
                        if bytes.len() as u64 != length {
                            return Err(invalid("Truncated data block"));
                        }
                        process(&bytes)?;
                        offset = offset
                            .checked_add(length)
                            .ok_or_else(|| invalid("Invalid block offset"))?;
                        remaining -= length;
                    }
                }
                stream_offset = block_end;
            }
        }
        if !stream_offset.is_multiple_of(record_size_u64) || !pending.is_empty() {
            return Err(invalid("Truncated MDF record"));
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blocks::DataType;
    use crate::index::{DataBlockInfo, IndexedChannelGroup};

    struct MemoryReader {
        bytes: Vec<u8>,
        largest_read: u64,
        total_read: u64,
    }

    impl ByteRangeReader for MemoryReader {
        type Error = Error;

        fn read_range(&mut self, offset: u64, length: u64) -> Result<Vec<u8>> {
            self.largest_read = self.largest_read.max(length);
            self.total_read += length;
            let start = usize::try_from(offset)
                .map_err(|_| Error::BlockSerializationError("Offset overflow".into()))?;
            let length = usize::try_from(length)
                .map_err(|_| Error::BlockSerializationError("Length overflow".into()))?;
            let end = start
                .checked_add(length)
                .ok_or_else(|| Error::BlockSerializationError("Range overflow".into()))?;
            self.bytes
                .get(start..end)
                .map(<[u8]>::to_vec)
                .ok_or(Error::TooShortBuffer {
                    actual: self.bytes.len().saturating_sub(start),
                    expected: length,
                    file: file!(),
                    line: line!(),
                })
        }
    }

    fn channel(name: &str, offset: u32) -> IndexedChannel {
        IndexedChannel {
            name: Some(name.to_string()),
            unit: None,
            data_type: DataType::FloatLE,
            byte_offset: offset,
            bit_offset: 0,
            bit_count: 64,
            channel_type: 0,
            component_addr: 0,
            flags: 0,
            pos_invalidation_bit: 0,
            conversion: None,
            vlsd_data_address: None,
        }
    }

    fn make_index(
        record_size: u32,
        channels: Vec<IndexedChannel>,
        data_blocks: Vec<DataBlockInfo>,
    ) -> MdfIndex {
        MdfIndex {
            file_size: 0,
            channel_groups: vec![IndexedChannelGroup {
                name: None,
                comment: None,
                record_id_size: 0,
                record_size,
                invalidation_bytes: 0,
                record_count: 0,
                channels,
                data_blocks,
            }],
        }
    }

    #[test]
    fn reads_fragmented_records_and_preserves_original_indices() {
        let mut records = Vec::new();
        for (time, value) in [(2.0_f64, 20.0_f64), (0.0, 0.0), (1.0, 10.0)] {
            records.extend_from_slice(&time.to_le_bytes());
            records.extend_from_slice(&value.to_le_bytes());
        }
        let mut bytes = vec![0; 24];
        bytes.extend_from_slice(&records[..13]);
        let second_offset = bytes.len() as u64;
        bytes.extend_from_slice(&[0; 24]);
        bytes.extend_from_slice(&records[13..]);
        let index = make_index(
            16,
            vec![channel("time", 0), channel("value", 8)],
            vec![
                DataBlockInfo {
                    file_offset: 0,
                    size: 37,
                    is_compressed: false,
                },
                DataBlockInfo {
                    file_offset: second_offset,
                    size: 24 + (records.len() - 13) as u64,
                    is_compressed: false,
                },
            ],
        );
        let mut reader = MemoryReader {
            bytes,
            largest_read: 0,
            total_read: 0,
        };
        let result = index
            .read_group_time_range(0, &[1], Some(0), Some(1.0), Some(2.0), &mut reader)
            .unwrap();
        assert_eq!(result.times, [2.0, 1.0]);
        assert_eq!(result.values, [vec![Some(20.0), Some(10.0)]]);

        let result = index
            .read_group_time_range(0, &[1], None, Some(1.0), Some(2.0), &mut reader)
            .unwrap();
        assert_eq!(result.times, [1.0, 2.0]);
        assert_eq!(result.values, [vec![Some(0.0), Some(10.0)]]);
    }

    #[test]
    fn limits_uncompressed_reads_to_the_requested_record_range() {
        let record_count = 150_000;
        let bytes = vec![0; 24 + record_count * 8];
        let index = make_index(
            8,
            vec![channel("value", 0)],
            vec![DataBlockInfo {
                file_offset: 0,
                size: bytes.len() as u64,
                is_compressed: false,
            }],
        );
        let mut reader = MemoryReader {
            bytes,
            largest_read: 0,
            total_read: 0,
        };
        let result = index
            .read_group_time_range(0, &[0], None, Some(100.0), Some(101.0), &mut reader)
            .unwrap();
        assert_eq!(result.times, [100.0, 101.0]);
        assert_eq!(reader.total_read, 16);
        assert!(reader.largest_read <= 1024 * 1024);
    }

    #[test]
    fn rejects_invalid_time_range_and_indices() {
        let mut index = make_index(8, vec![channel("time", 0)], Vec::new());
        let mut reader = MemoryReader {
            bytes: Vec::new(),
            largest_read: 0,
            total_read: 0,
        };
        assert!(
            index
                .read_group_time_range(0, &[], None, Some(2.0), Some(1.0), &mut reader)
                .is_err()
        );
        assert!(
            index
                .read_group_time_range(0, &[], None, Some(f64::NAN), None, &mut reader)
                .is_err()
        );
        assert!(
            index
                .read_group_time_range(1, &[], None, None, None, &mut reader)
                .is_err()
        );
        assert!(
            index
                .read_group_time_range(0, &[1], None, None, None, &mut reader)
                .is_err()
        );
        assert!(
            index
                .read_group_time_range(0, &[], Some(1), None, None, &mut reader)
                .is_err()
        );
        index.channel_groups[0].record_id_size = 1;
        assert!(
            index
                .read_group_time_range(0, &[], None, None, None, &mut reader)
                .is_err()
        );
        index.channel_groups[0].record_id_size = 0;
        index.channel_groups[0].record_size = 0;
        assert!(
            index
                .read_group_time_range(0, &[], None, None, None, &mut reader)
                .is_err()
        );
    }

    #[test]
    fn rejects_short_data_reads() {
        struct ShortReader;
        impl ByteRangeReader for ShortReader {
            type Error = Error;

            fn read_range(&mut self, _offset: u64, _length: u64) -> Result<Vec<u8>> {
                Ok(Vec::new())
            }
        }

        let index = make_index(
            8,
            vec![channel("value", 0)],
            vec![DataBlockInfo {
                file_offset: 0,
                size: 32,
                is_compressed: false,
            }],
        );
        assert!(
            index
                .read_group_time_range(0, &[0], None, None, None, &mut ShortReader)
                .is_err()
        );
    }

    #[cfg(feature = "compression")]
    #[test]
    fn reads_records_split_between_compressed_blocks() {
        use miniz_oxide::deflate::compress_to_vec_zlib;

        let mut records = Vec::new();
        for value in [10.0_f64, 20.0, 30.0] {
            records.extend_from_slice(&value.to_le_bytes());
        }
        let mut bytes = Vec::new();
        let mut blocks = Vec::new();
        for fragment in [&records[..11], &records[11..]] {
            let compressed = compress_to_vec_zlib(fragment, 6);
            let size = 48 + compressed.len() as u64;
            blocks.push(DataBlockInfo {
                file_offset: bytes.len() as u64,
                size,
                is_compressed: true,
            });
            bytes.extend_from_slice(b"##DZ");
            bytes.extend_from_slice(&0u32.to_le_bytes());
            bytes.extend_from_slice(&size.to_le_bytes());
            bytes.extend_from_slice(&0u64.to_le_bytes());
            bytes.extend_from_slice(b"DT");
            bytes.extend_from_slice(&[0, 0]);
            bytes.extend_from_slice(&0u32.to_le_bytes());
            bytes.extend_from_slice(&(fragment.len() as u64).to_le_bytes());
            bytes.extend_from_slice(&(compressed.len() as u64).to_le_bytes());
            bytes.extend_from_slice(&compressed);
        }
        let index = make_index(8, vec![channel("value", 0)], blocks);
        let mut reader = MemoryReader {
            bytes,
            largest_read: 0,
            total_read: 0,
        };

        let result = index
            .read_group_time_range(0, &[0], None, Some(1.0), Some(1.0), &mut reader)
            .unwrap();
        assert_eq!(result.times, [1.0]);
        assert_eq!(result.values, [vec![Some(20.0)]]);
    }

    #[cfg(not(feature = "compression"))]
    #[test]
    fn rejects_compressed_blocks_without_compression_support() {
        let index = make_index(
            8,
            vec![channel("value", 0)],
            vec![DataBlockInfo {
                file_offset: 0,
                size: 48,
                is_compressed: true,
            }],
        );
        let mut reader = MemoryReader {
            bytes: vec![0; 48],
            largest_read: 0,
            total_read: 0,
        };
        assert!(
            index
                .read_group_time_range(0, &[0], None, None, None, &mut reader)
                .is_err()
        );
    }
}
