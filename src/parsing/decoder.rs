use crate::blocks::{ChannelBlock, DataType};

// Re-export DecodedValue from types module for backward compatibility
pub use crate::types::DecodedValue;

// Flag bit positions for cn_flags
const CN_FLAG_ALL_INVALID: u32 = 0x01; // Bit 0: All values are invalid
const CN_FLAG_INVAL_BIT_VALID: u32 = 0x02; // Bit 1: Invalidation bit is valid

/// Result of decoding a channel value, including validity status.
#[derive(Debug, Clone, PartialEq)]
pub struct DecodedChannelValue {
    pub value: DecodedValue,
    pub is_valid: bool,
}

/// Checks if a channel value is valid based on invalidation bits.
///
/// According to MDF 4.1 spec section 4.21.5.1:
/// - If cn_flags bit 0 is set (1), all values are invalid
/// - If cn_flags bits 0 and 1 are both clear (0), all values are valid
/// - Otherwise, must check the invalidation bit in the record
///
/// # Parameters
/// - `record`: The complete record bytes including record ID, data, and invalidation bytes
/// - `record_id_size`: Number of bytes for the record ID
/// - `cg_data_bytes`: Number of bytes for the data portion (samples_byte_nr from channel group)
/// - `channel`: The channel block containing flags and invalidation bit position
///
/// # Returns
/// `true` if the value is valid, `false` if invalid
pub fn check_value_validity(
    record: &[u8],
    record_id_size: usize,
    cg_data_bytes: u32,
    channel: &ChannelBlock,
) -> bool {
    // Check cn_flags first for shortcuts
    if channel.flags & CN_FLAG_ALL_INVALID != 0 {
        // Bit 0 set: all values are invalid
        return false;
    }

    if channel.flags & (CN_FLAG_ALL_INVALID | CN_FLAG_INVAL_BIT_VALID) == 0 {
        // Bits 0 and 1 both clear: all values are valid
        return true;
    }

    // Must check the invalidation bit in the record
    // Location: record_id + data_bytes + (cn_inval_bit_pos >> 3)
    let inval_byte_offset =
        record_id_size + cg_data_bytes as usize + (channel.pos_invalidation_bit >> 3) as usize;
    let inval_bit_index = (channel.pos_invalidation_bit & 0x07) as usize;

    if inval_byte_offset < record.len() {
        let inval_byte = record[inval_byte_offset];
        let bit_is_set = (inval_byte >> inval_bit_index) & 0x01 != 0;
        // If the invalidation bit is set (1), the value is INVALID
        !bit_is_set
    } else {
        // No invalidation byte available, assume valid
        true
    }
}

/// Decodes a channel's sample from a record (legacy function without validity checking).
///
/// This function takes the raw record data, skips over the record ID,
/// and then uses channel metadata (offsets, bit settings, and data type)
/// from the given `ChannelBlock` to decode the sample. It supports numeric
/// types (unsigned/signed integers, floats), strings (Latin1, UTF-8, UTF-16LE,
/// UTF-16BE), byte arrays, and MIME samples/streams.
///
/// # Parameters
/// - `record`: A slice containing the entire record's bytes.
/// - `record_id_size`: The number of bytes reserved at the beginning of the record for the record ID.
/// - `channel`: A reference to the channel metadata used for decoding.
///
/// # Returns
/// An `Option<DecodedValue>` containing the decoded sample, or `None` if there isn't enough data.
///
/// # Note
/// This function does NOT check invalidation bits. For full MDF spec compliance,
/// use `decode_channel_value_with_validity` instead.
pub fn decode_channel_value(
    record: &[u8],
    record_id_size: usize,
    channel: &ChannelBlock,
) -> Option<DecodedValue> {
    decode_value_internal(record, record_id_size, channel)
}

/// Decodes a channel's sample from a record with validity checking.
///
/// This function performs the full MDF 4.1 spec-compliant decoding including
/// invalidation bit checking. It returns both the decoded value and whether
/// the value is valid according to the invalidation bits.
///
/// # Parameters
/// - `record`: A slice containing the entire record's bytes (including invalidation bytes)
/// - `record_id_size`: The number of bytes reserved at the beginning of the record for the record ID
/// - `cg_data_bytes`: Number of data bytes in the record (samples_byte_nr from channel group)
/// - `channel`: A reference to the channel metadata used for decoding
///
/// # Returns
/// An `Option<DecodedChannelValue>` containing the decoded sample and validity status,
/// or `None` if there isn't enough data to decode.
pub fn decode_channel_value_with_validity(
    record: &[u8],
    record_id_size: usize,
    cg_data_bytes: u32,
    channel: &ChannelBlock,
) -> Option<DecodedChannelValue> {
    let value = decode_value_internal(record, record_id_size, channel)?;
    let is_valid = check_value_validity(record, record_id_size, cg_data_bytes, channel);

    Some(DecodedChannelValue { value, is_valid })
}

/// Internal function that performs the actual value decoding.
///
/// This is the core decoding logic separated out so it can be used by both
/// the legacy function and the new validity-aware function.
fn decode_value_internal(
    record: &[u8],
    record_id_size: usize,
    channel: &ChannelBlock,
) -> Option<DecodedValue> {
    // Calculate the starting offset of this channel's data.
    let base_offset = record_id_size.checked_add(channel.byte_offset as usize)?;
    let bit_offset = channel.bit_offset as usize;
    let bit_count = channel.bit_count as usize;
    if bit_offset >= 8 {
        return None;
    }
    if matches!(
        channel.data_type,
        DataType::UnsignedIntegerLE
            | DataType::UnsignedIntegerBE
            | DataType::SignedIntegerLE
            | DataType::SignedIntegerBE
    ) && !(1..=64).contains(&bit_count)
    {
        return None;
    }
    if matches!(channel.data_type, DataType::FloatLE | DataType::FloatBE)
        && !matches!(bit_count, 32 | 64)
    {
        return None;
    }

    let slice: &[u8] = if channel.channel_type == 1 && channel.data_addr != 0 {
        // VLSD: the entire record *is* the payload
        record
    } else {
        // For non-numeric types, assume the field is stored in whole bytes.
        let num_bytes = if matches!(
            channel.data_type,
            DataType::StringLatin1
                | DataType::StringUtf8
                | DataType::StringUtf16LE
                | DataType::StringUtf16BE
                | DataType::ByteArray
                | DataType::MimeSample
                | DataType::MimeStream
        ) {
            bit_count / 8
        } else {
            (bit_offset + bit_count).div_ceil(8).max(1)
        };

        let end_offset = base_offset.checked_add(num_bytes)?;
        if end_offset > record.len() {
            return None;
        }
        &record[base_offset..end_offset]
    };

    match &channel.data_type {
        DataType::UnsignedIntegerLE => {
            let raw = slice
                .iter()
                .rev()
                .fold(0u128, |acc, &b| (acc << 8) | b as u128);
            let shifted = (raw >> bit_offset) as u64;
            let mask = if bit_count >= 64 {
                u64::MAX
            } else {
                (1u64 << bit_count) - 1
            };
            Some(DecodedValue::UnsignedInteger(shifted & mask))
        }
        DataType::UnsignedIntegerBE => {
            let raw = slice.iter().fold(0u128, |acc, &b| (acc << 8) | b as u128);
            let shifted = (raw >> bit_offset) as u64;
            let mask = if bit_count >= 64 {
                u64::MAX
            } else {
                (1u64 << bit_count) - 1
            };
            Some(DecodedValue::UnsignedInteger(shifted & mask))
        }
        DataType::SignedIntegerLE => {
            let raw = slice
                .iter()
                .rev()
                .fold(0u128, |acc, &b| (acc << 8) | b as u128);
            let shifted = (raw >> bit_offset) as u64;
            let mask = if bit_count >= 64 {
                u64::MAX
            } else {
                (1u64 << bit_count) - 1
            };
            let unsigned = shifted & mask;
            let sign_bit = 1u64 << (bit_count - 1);
            let signed = if unsigned & sign_bit != 0 {
                (unsigned as i64) | (!(mask as i64))
            } else {
                unsigned as i64
            };
            Some(DecodedValue::SignedInteger(signed))
        }
        DataType::SignedIntegerBE => {
            let raw = slice.iter().fold(0u128, |acc, &b| (acc << 8) | b as u128);
            let shifted = (raw >> bit_offset) as u64;
            let mask = if bit_count >= 64 {
                u64::MAX
            } else {
                (1u64 << bit_count) - 1
            };
            let unsigned = shifted & mask;
            let sign_bit = 1u64 << (bit_count - 1);
            let signed = if unsigned & sign_bit != 0 {
                (unsigned as i64) | (!(mask as i64))
            } else {
                unsigned as i64
            };
            Some(DecodedValue::SignedInteger(signed))
        }
        DataType::FloatLE => {
            let raw = slice
                .iter()
                .rev()
                .fold(0u128, |acc, &b| (acc << 8) | b as u128)
                >> bit_offset;
            if bit_count == 32 {
                Some(DecodedValue::Float(f32::from_bits(raw as u32) as f64))
            } else if bit_count == 64 {
                Some(DecodedValue::Float(f64::from_bits(raw as u64)))
            } else {
                None
            }
        }
        DataType::FloatBE => {
            let raw = slice.iter().fold(0u128, |acc, &b| (acc << 8) | b as u128) >> bit_offset;
            if bit_count == 32 {
                Some(DecodedValue::Float(f32::from_bits(raw as u32) as f64))
            } else if bit_count == 64 {
                Some(DecodedValue::Float(f64::from_bits(raw as u64)))
            } else {
                None
            }
        }

        DataType::StringLatin1 => {
            // Latin1: each byte maps directly to a character.
            let s: String = slice.iter().map(|&b| b as char).collect();
            Some(DecodedValue::String(s.trim_end_matches('\0').to_string()))
        }
        DataType::StringUtf8 => match std::str::from_utf8(slice) {
            Ok(s) => Some(DecodedValue::String(s.trim_end_matches('\0').to_string())),
            Err(_) => Some(DecodedValue::String(String::from("<Invalid UTF8>"))),
        },
        DataType::StringUtf16LE => {
            if !slice.len().is_multiple_of(2) {
                return None;
            }
            let u16_data: Vec<u16> = slice
                .chunks_exact(2)
                .map(|chunk| u16::from_le_bytes(chunk.try_into().unwrap()))
                .collect();
            match String::from_utf16(&u16_data) {
                Ok(s) => Some(DecodedValue::String(s.trim_end_matches('\0').to_string())),
                Err(_) => Some(DecodedValue::String(String::from("<Invalid UTF16LE>"))),
            }
        }
        DataType::StringUtf16BE => {
            if !slice.len().is_multiple_of(2) {
                return None;
            }
            let u16_data: Vec<u16> = slice
                .chunks_exact(2)
                .map(|chunk| u16::from_be_bytes(chunk.try_into().unwrap()))
                .collect();
            match String::from_utf16(&u16_data) {
                Ok(s) => Some(DecodedValue::String(s.trim_end_matches('\0').to_string())),
                Err(_) => Some(DecodedValue::String(String::from("<Invalid UTF16BE>"))),
            }
        }
        DataType::ByteArray => Some(DecodedValue::ByteArray(slice.to_vec())),
        DataType::MimeSample => Some(DecodedValue::MimeSample(slice.to_vec())),
        DataType::MimeStream => Some(DecodedValue::MimeStream(slice.to_vec())),
        _ => Some(DecodedValue::Unknown),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_unaligned_64_bit_integers() {
        let value = 0xfedc_ba98_7654_3210u64;
        let encoded = u128::from(value) << 4;
        for (data_type, bytes) in [
            (
                DataType::UnsignedIntegerLE,
                encoded.to_le_bytes()[..9].to_vec(),
            ),
            (
                DataType::UnsignedIntegerBE,
                encoded.to_be_bytes()[7..].to_vec(),
            ),
        ] {
            let channel = ChannelBlock {
                data_type,
                bit_count: 64,
                bit_offset: 4,
                ..Default::default()
            };
            assert_eq!(
                decode_channel_value(&bytes, 0, &channel),
                Some(DecodedValue::UnsignedInteger(value))
            );
        }

        let channel = ChannelBlock {
            data_type: DataType::SignedIntegerLE,
            bit_count: 64,
            bit_offset: 4,
            ..Default::default()
        };
        assert_eq!(
            decode_channel_value(&encoded.to_le_bytes()[..9], 0, &channel),
            Some(DecodedValue::SignedInteger(value as i64))
        );
    }

    #[test]
    fn rejects_invalid_integer_layouts() {
        for (width, offset) in [(0, 0), (65, 0), (64, 8), (64, 255)] {
            let channel = ChannelBlock {
                data_type: DataType::SignedIntegerLE,
                bit_count: width,
                bit_offset: offset,
                ..Default::default()
            };
            assert_eq!(decode_channel_value(&[0; 40], 0, &channel), None);
        }
    }

    #[test]
    fn decodes_unaligned_floats() {
        let value = 42.5f64;
        let encoded = u128::from(value.to_bits()) << 3;
        for (data_type, bytes) in [
            (DataType::FloatLE, encoded.to_le_bytes()[..9].to_vec()),
            (DataType::FloatBE, encoded.to_be_bytes()[7..].to_vec()),
        ] {
            let channel = ChannelBlock {
                data_type,
                bit_count: 64,
                bit_offset: 3,
                ..Default::default()
            };
            assert_eq!(
                decode_channel_value(&bytes, 0, &channel),
                Some(DecodedValue::Float(value))
            );
        }
    }

    fn decode(data_type: DataType, bit_count: u32, record: &[u8]) -> Option<DecodedValue> {
        let channel = ChannelBlock {
            data_type,
            bit_count,
            ..Default::default()
        };
        decode_channel_value(record, 0, &channel)
    }

    #[test]
    fn decodes_big_endian_signed_and_unsigned_values() {
        assert_eq!(
            decode(DataType::UnsignedIntegerBE, 8, &[0xab]),
            Some(DecodedValue::UnsignedInteger(0xab))
        );
        assert_eq!(
            decode(DataType::SignedIntegerBE, 8, &[0x7f]),
            Some(DecodedValue::SignedInteger(127))
        );
        assert_eq!(
            decode(DataType::SignedIntegerBE, 8, &[0xff]),
            Some(DecodedValue::SignedInteger(-1))
        );
    }

    #[test]
    fn decodes_32_bit_floats_in_both_byte_orders() {
        let value = 42.5f32;
        assert_eq!(
            decode(DataType::FloatLE, 32, &value.to_bits().to_le_bytes()),
            Some(DecodedValue::Float(value as f64))
        );
        assert_eq!(
            decode(DataType::FloatBE, 32, &value.to_bits().to_be_bytes()),
            Some(DecodedValue::Float(value as f64))
        );
        assert_eq!(decode(DataType::FloatLE, 16, &[0, 0]), None);
    }

    #[test]
    fn decodes_string_encodings_and_invalid_inputs() {
        assert_eq!(
            decode(DataType::StringLatin1, 16, &[0xe9, 0]),
            Some(DecodedValue::String("é".to_string()))
        );
        assert_eq!(
            decode(DataType::StringUtf8, 24, b"ok\0"),
            Some(DecodedValue::String("ok".to_string()))
        );
        assert_eq!(
            decode(DataType::StringUtf8, 8, &[0xff]),
            Some(DecodedValue::String("<Invalid UTF8>".to_string()))
        );
        assert_eq!(
            decode(DataType::StringUtf16LE, 32, &[b'A', 0, 0, 0]),
            Some(DecodedValue::String("A".to_string()))
        );
        assert_eq!(
            decode(DataType::StringUtf16BE, 32, &[0, b'A', 0, 0]),
            Some(DecodedValue::String("A".to_string()))
        );
        assert_eq!(
            decode(DataType::StringUtf16LE, 16, &[0, 0xd8]),
            Some(DecodedValue::String("<Invalid UTF16LE>".to_string()))
        );
        assert_eq!(
            decode(DataType::StringUtf16BE, 16, &[0xd8, 0]),
            Some(DecodedValue::String("<Invalid UTF16BE>".to_string()))
        );
        assert_eq!(decode(DataType::StringUtf16LE, 8, &[b'A']), None);
        assert_eq!(decode(DataType::StringUtf16BE, 8, &[b'A']), None);
    }

    #[test]
    fn decodes_mime_and_unknown_types() {
        assert_eq!(
            decode(DataType::MimeSample, 16, &[1, 2]),
            Some(DecodedValue::MimeSample(vec![1, 2]))
        );
        assert_eq!(
            decode(DataType::MimeStream, 16, &[3, 4]),
            Some(DecodedValue::MimeStream(vec![3, 4]))
        );
        assert_eq!(
            decode(DataType::CanOpenDate, 64, &[0; 8]),
            Some(DecodedValue::Unknown)
        );
    }

    #[test]
    fn handles_variable_length_payloads_and_invalid_ranges() {
        let channel = ChannelBlock {
            channel_type: 1,
            data_addr: 1,
            data_type: DataType::StringUtf8,
            bit_count: 16,
            ..Default::default()
        };
        assert_eq!(
            decode_channel_value(b"ok", 0, &channel),
            Some(DecodedValue::String("ok".to_string()))
        );

        let out_of_bounds = ChannelBlock {
            byte_offset: 2,
            bit_count: 8,
            ..Default::default()
        };
        assert_eq!(decode_channel_value(&[1], 0, &out_of_bounds), None);

        let overflowing_start = ChannelBlock {
            byte_offset: 1,
            bit_count: 8,
            ..Default::default()
        };
        assert_eq!(
            decode_channel_value(&[1], usize::MAX, &overflowing_start),
            None
        );

        let overflowing_end = ChannelBlock {
            bit_count: 8,
            ..Default::default()
        };
        assert_eq!(
            decode_channel_value(&[], usize::MAX, &overflowing_end),
            None
        );
    }

    #[test]
    fn checks_validity_flags_and_invalidation_bits() {
        let all_invalid = ChannelBlock {
            flags: CN_FLAG_ALL_INVALID,
            ..Default::default()
        };
        assert!(!check_value_validity(&[0; 3], 0, 2, &all_invalid));

        let all_valid = ChannelBlock::default();
        assert!(check_value_validity(&[0; 3], 0, 2, &all_valid));

        let invalidation = ChannelBlock {
            flags: CN_FLAG_INVAL_BIT_VALID,
            pos_invalidation_bit: 1,
            ..Default::default()
        };
        assert!(check_value_validity(&[0, 0, 0], 0, 2, &invalidation));
        assert!(!check_value_validity(&[0, 0, 0b10], 0, 2, &invalidation));
        assert!(check_value_validity(&[0, 0], 0, 2, &invalidation));
    }
}
