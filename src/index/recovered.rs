use std::collections::{HashMap, HashSet};

use super::time_range::{channel_block, decode};
use super::{BufferedRangeReader, ByteRangeReader, IndexedChannelGroup, MdfIndex, TimeRangeData};
use crate::blocks::{
    BlockHeader, BlockParse, ChannelGroupBlock, DataGroupBlock, HeaderBlock, IdentificationBlock,
};
use crate::{Error, Result};

fn invalid(message: &str) -> Error {
    Error::BlockSerializationError(format!("Cannot recover unfinalized MDF: {message}"))
}

fn read_exact<R: ByteRangeReader<Error = Error>>(
    reader: &mut R,
    offset: u64,
    length: u64,
) -> Result<Vec<u8>> {
    let bytes = reader.read_range(offset, length)?;
    if bytes.len() as u64 != length {
        return Err(invalid("truncated byte range"));
    }
    Ok(bytes)
}

/// An in-memory record index recovered from an unfinalized MDF file.
///
/// Recovery scans direct DT blocks and records one offset per record without
/// modifying the source. It supports the standard finalization flags for
/// channel group counts, final DT length and VLSD byte counts. If the DT
/// length needs recovery, only a single header-only DT extending to EOF is
/// supported. Other flags and block layouts are rejected.
///
/// Fixed-size records may be interleaved with variable-length records, which
/// are counted but cannot be read through this API. The index is valid only
/// while the source remains unchanged.
pub struct RecoveredMdfIndex {
    index: MdfIndex,
    offsets: Vec<Vec<u64>>,
    variable: Vec<bool>,
}

impl RecoveredMdfIndex {
    /// Scan and recover an unfinalized file using buffered file reads.
    pub fn from_file(path: &str) -> Result<Self> {
        let size = std::fs::metadata(path).map_err(Error::IOError)?.len();
        let mut reader = BufferedRangeReader::new(path)?;
        Self::from_reader(&mut reader, size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DataType, DecodedValue, MdfWriter};

    struct MemoryReader(Vec<u8>);

    impl ByteRangeReader for MemoryReader {
        type Error = Error;

        fn read_range(&mut self, offset: u64, length: u64) -> Result<Vec<u8>> {
            let start = usize::try_from(offset).map_err(|_| invalid("offset overflow"))?;
            let len = usize::try_from(length).map_err(|_| invalid("length overflow"))?;
            let end = start
                .checked_add(len)
                .ok_or_else(|| invalid("range overflow"))?;
            self.0
                .get(start..end)
                .map(<[u8]>::to_vec)
                .ok_or_else(|| invalid("truncated test data"))
        }
    }

    fn u64_at(bytes: &[u8], offset: usize) -> usize {
        u64::from_le_bytes(bytes[offset..offset + 8].try_into().unwrap()) as usize
    }

    fn put_u64(bytes: &mut [u8], offset: usize, value: usize) {
        bytes[offset..offset + 8].copy_from_slice(&(value as u64).to_le_bytes());
    }

    fn fixture() -> Vec<u8> {
        let mut writer = MdfWriter::in_memory();
        writer.init_mdf_file().unwrap();
        let group = writer.add_channel_group(None, |_| {}).unwrap();
        let time = writer
            .add_channel(&group, None, |channel| {
                channel.name = Some("time".into());
                channel.data_type = DataType::FloatLE;
                channel.bit_count = 64;
            })
            .unwrap();
        writer.set_time_channel(&time).unwrap();
        writer
            .add_channel(&group, Some(&time), |channel| {
                channel.name = Some("value".into());
                channel.data_type = DataType::FloatLE;
                channel.bit_count = 64;
            })
            .unwrap();
        writer.start_data_block_for_cg(&group, 0).unwrap();
        for (t, value) in [(2.0, 20.0), (1.0, 10.0)] {
            writer
                .write_record(
                    &group,
                    &[DecodedValue::Float(t), DecodedValue::Float(value)],
                )
                .unwrap();
        }
        writer.finish_data_block(&group).unwrap();
        writer.finalize().unwrap();
        let mut data = writer.into_inner().into_inner();
        let dg = u64_at(&data, 88);
        let cg = u64_at(&data, dg + 32);
        let dt = u64_at(&data, dg + 40);
        let records = data[dt + 24..dt + 24 + 32].to_vec();
        data[..8].copy_from_slice(b"UnFinMF ");
        data[60..62].copy_from_slice(&0x25u16.to_le_bytes());
        data[dg + 56] = 1;
        put_u64(&mut data, cg + 72, 1);
        put_u64(&mut data, cg + 80, 0);

        while data.len() % 8 != 0 {
            data.push(0);
        }
        let variable_cg = data.len();
        let mut block = data[cg..cg + 104].to_vec();
        block[24..72].fill(0);
        put_u64(&mut block, 72, 2);
        put_u64(&mut block, 80, 0);
        block[88..90].copy_from_slice(&1u16.to_le_bytes());
        block[96..104].fill(0);
        data.extend_from_slice(&block);
        put_u64(&mut data, cg + 24, variable_cg);

        let new_dt = data.len();
        let header = data[dt..dt + 24].to_vec();
        data.extend_from_slice(&header);
        put_u64(&mut data, new_dt + 8, 24);
        put_u64(&mut data, dg + 40, new_dt);
        data.push(1);
        data.extend_from_slice(&records[..16]);
        data.push(2);
        data.extend_from_slice(&3u32.to_le_bytes());
        data.extend_from_slice(&[11, 12, 13]);
        data.push(1);
        data.extend_from_slice(&records[16..]);
        data
    }

    #[test]
    fn recovers_interleaved_records_and_reads_ranges() {
        let bytes = fixture();
        let mut reader = MemoryReader(bytes.clone());
        let index = RecoveredMdfIndex::from_reader(&mut reader, bytes.len() as u64).unwrap();
        assert_eq!(
            index
                .channel_groups()
                .iter()
                .map(|g| g.record_count)
                .collect::<Vec<_>>(),
            [2, 1]
        );
        let result = index
            .read_group_time_range(0, &[1], Some(0), Some(1.0), Some(1.0), &mut reader)
            .unwrap();
        assert_eq!(result.times, [1.0]);
        assert_eq!(result.values, [vec![Some(10.0)]]);
        let result = index
            .read_group_time_range(0, &[1], None, Some(0.0), Some(0.0), &mut reader)
            .unwrap();
        assert_eq!(result.times, [0.0]);
        assert_eq!(result.values, [vec![Some(20.0)]]);
        assert!(
            index
                .read_group_time_range(1, &[], None, None, None, &mut reader)
                .is_err()
        );
        assert!(
            index
                .read_group_time_range(2, &[], None, None, None, &mut reader)
                .is_err()
        );
        assert!(
            index
                .read_group_time_range(0, &[2], None, None, None, &mut reader)
                .is_err()
        );
        assert!(
            index
                .read_group_time_range(0, &[], Some(2), None, None, &mut reader)
                .is_err()
        );
        assert!(
            index
                .read_group_time_range(0, &[], None, Some(f64::NAN), None, &mut reader)
                .is_err()
        );
        assert!(
            index
                .read_group_time_range(0, &[], None, Some(2.0), Some(1.0), &mut reader)
                .is_err()
        );
    }

    #[test]
    fn rejects_unsupported_and_truncated_files() {
        let mut bytes = fixture();
        bytes[60] |= 2;
        let mut reader = MemoryReader(bytes.clone());
        assert!(RecoveredMdfIndex::from_reader(&mut reader, bytes.len() as u64).is_err());
        let mut bytes = fixture();
        bytes.pop();
        let mut reader = MemoryReader(bytes.clone());
        assert!(RecoveredMdfIndex::from_reader(&mut reader, bytes.len() as u64).is_err());
        let mut bytes = fixture();
        bytes[..8].copy_from_slice(b"MDF     ");
        let mut reader = MemoryReader(bytes.clone());
        assert!(RecoveredMdfIndex::from_reader(&mut reader, bytes.len() as u64).is_err());
    }
}

impl RecoveredMdfIndex {
    /// Scan and recover an unfinalized file using a byte range reader.
    pub fn from_reader<R: ByteRangeReader<Error = Error>>(
        reader: &mut R,
        file_size: u64,
    ) -> Result<Self> {
        let id = IdentificationBlock::from_bytes(&read_exact(reader, 0, 64)?)?;
        if id.file_id.trim() != "UnFinMF" {
            return Err(invalid("file is not unfinalized"));
        }
        if id.unfinalized_flags & !0x25 != 0 || id.custom_flags != 0 {
            return Err(invalid("unsupported finalization flags"));
        }
        let header = HeaderBlock::from_bytes(&read_exact(reader, 64, 104)?)?;
        let mut index = MdfIndex::from_reader(reader, file_size)?;
        let mut offsets = vec![Vec::new(); index.channel_groups.len()];
        let mut variable_groups = vec![false; index.channel_groups.len()];
        let mut group_index = 0;
        let mut dg_addr = header.first_dg_addr;
        let mut seen_dg = HashSet::new();
        let mut open_ended_data = false;
        while dg_addr != 0 {
            if !seen_dg.insert(dg_addr) {
                return Err(Error::LinkCycle { address: dg_addr });
            }
            let dg = DataGroupBlock::from_bytes(&read_exact(reader, dg_addr, 64)?)?;
            let id_size = dg.record_id_size as usize;
            if !matches!(id_size, 0 | 1 | 2 | 4 | 8) {
                return Err(invalid("unsupported record ID size"));
            }
            let mut groups = HashMap::new();
            let mut cg_addr = dg.first_cg_addr;
            let mut seen_cg = HashSet::new();
            while cg_addr != 0 {
                if !seen_cg.insert(cg_addr) {
                    return Err(Error::LinkCycle { address: cg_addr });
                }
                let cg = ChannelGroupBlock::from_bytes(&read_exact(reader, cg_addr, 104)?)?;
                if group_index >= offsets.len() || (id_size == 0 && !groups.is_empty()) {
                    return Err(invalid("invalid channel group layout"));
                }
                let variable = cg.flags & 1 != 0;
                variable_groups[group_index] = variable;
                let size = u64::from(cg.record_size) + u64::from(cg.invalidation_size);
                if !variable && size == 0 {
                    return Err(invalid("zero-sized record"));
                }
                if groups
                    .insert(cg.record_id, (group_index, size, variable))
                    .is_some()
                {
                    return Err(invalid("duplicate record ID"));
                }
                group_index += 1;
                cg_addr = cg.next_cg_addr;
            }
            if dg.data_block_addr != 0 {
                let block = BlockHeader::from_bytes(&read_exact(reader, dg.data_block_addr, 24)?)?;
                if block.id != "##DT" {
                    return Err(invalid("only direct DT blocks are supported"));
                }
                if id.unfinalized_flags & 4 != 0 && block.length != 24 {
                    return Err(invalid("ambiguous unfinalized DT block length"));
                }
                let start = dg
                    .data_block_addr
                    .checked_add(24)
                    .ok_or_else(|| invalid("data offset overflow"))?;
                let end = if block.length == 24 && id.unfinalized_flags & 4 != 0 {
                    if open_ended_data {
                        return Err(invalid("multiple unfinalized DT blocks"));
                    }
                    open_ended_data = true;
                    file_size
                } else {
                    dg.data_block_addr
                        .checked_add(block.length)
                        .ok_or_else(|| invalid("data length overflow"))?
                };
                if end < start || end > file_size {
                    return Err(invalid("invalid data block length"));
                }
                let mut position = start;
                while position < end {
                    if end - position < id_size as u64 {
                        return Err(invalid("truncated record ID"));
                    }
                    let mut record_id = [0u8; 8];
                    if id_size != 0 {
                        record_id[..id_size].copy_from_slice(&read_exact(
                            reader,
                            position,
                            id_size as u64,
                        )?);
                    }
                    position += id_size as u64;
                    let key = if id_size == 0 {
                        *groups
                            .keys()
                            .next()
                            .ok_or_else(|| invalid("data block has no channel group"))?
                    } else {
                        u64::from_le_bytes(record_id)
                    };
                    let &(group, fixed_size, variable) = groups
                        .get(&key)
                        .ok_or_else(|| invalid("unknown record ID"))?;
                    let size = if variable {
                        if end - position < 4 {
                            return Err(invalid("truncated variable-length record"));
                        }
                        let length: [u8; 4] = read_exact(reader, position, 4)?
                            .try_into()
                            .map_err(|_| invalid("truncated variable-length record"))?;
                        position += 4;
                        u32::from_le_bytes(length) as u64
                    } else {
                        fixed_size
                    };
                    if size > end - position {
                        return Err(invalid("truncated record"));
                    }
                    offsets[group].push(position);
                    position += size;
                }
            }
            dg_addr = dg.next_dg_addr;
        }
        if group_index != offsets.len() {
            return Err(invalid("channel group count mismatch"));
        }
        for (group, records) in index.channel_groups.iter_mut().zip(&offsets) {
            group.record_count = records.len() as u64;
        }
        Ok(Self {
            index,
            offsets,
            variable: variable_groups,
        })
    }

    /// Recovered group metadata in file order.
    pub fn channel_groups(&self) -> &[IndexedChannelGroup] {
        &self.index.channel_groups
    }

    /// Read numeric channels within an inclusive time or record-index range.
    ///
    /// If `time_index` is `None`, record indices are used as timestamps.
    /// Results preserve record order; invalid samples become NaN. The supplied
    /// reader must address the same unchanged file used to create the index.
    pub fn read_group_time_range<R: ByteRangeReader<Error = Error>>(
        &self,
        group_index: usize,
        channel_indices: &[usize],
        time_index: Option<usize>,
        start: Option<f64>,
        end: Option<f64>,
        reader: &mut R,
    ) -> Result<TimeRangeData> {
        if start.is_some_and(f64::is_nan)
            || end.is_some_and(f64::is_nan)
            || matches!((start, end), (Some(start), Some(end)) if start > end)
        {
            return Err(invalid("invalid time range"));
        }
        let group = self
            .index
            .channel_groups
            .get(group_index)
            .ok_or_else(|| invalid("invalid group index"))?;
        if self.variable[group_index] {
            return Err(invalid(
                "variable-length channel group reads are not supported",
            ));
        }
        let channels = channel_indices
            .iter()
            .map(|&i| {
                group
                    .channels
                    .get(i)
                    .map(channel_block)
                    .ok_or_else(|| invalid("invalid channel index"))
            })
            .collect::<Result<Vec<_>>>()?;
        let time_channel = time_index
            .map(|i| {
                group
                    .channels
                    .get(i)
                    .map(channel_block)
                    .ok_or_else(|| invalid("invalid time channel index"))
            })
            .transpose()?;
        let size = u64::from(group.record_size) + u64::from(group.invalidation_bytes);
        let mut result = TimeRangeData {
            times: Vec::new(),
            values: vec![Vec::new(); channels.len()],
        };
        for (record_index, &offset) in self.offsets[group_index].iter().enumerate() {
            if time_index.is_none()
                && (start.is_some_and(|v| (record_index as f64) < v)
                    || end.is_some_and(|v| (record_index as f64) > v))
            {
                continue;
            }
            let record = read_exact(reader, offset, size)?;
            let timestamp = match &time_channel {
                Some(channel) => decode(&record, group.record_size, channel)?
                    .ok_or_else(|| invalid("time channel is not numeric"))?,
                None => record_index as f64,
            };
            if (start.is_some() || end.is_some())
                && (timestamp.is_nan()
                    || start.is_some_and(|v| timestamp < v)
                    || end.is_some_and(|v| timestamp > v))
            {
                continue;
            }
            result.times.push(timestamp);
            for (column, channel) in result.values.iter_mut().zip(&channels) {
                column.push(decode(&record, group.record_size, channel)?);
            }
        }
        Ok(result)
    }
}
