# Changelog

## Features

- Add `RecoveredMdfIndex` for read-only recovery of supported unfinalized direct-DT recordings, including interleaved fixed-size and variable-length records, recovered record counts, and scalar time-range reads. Unsupported finalization flags, layouts, and variable-length group reads return errors.
- Add streaming inclusive time-range reads for selected numeric channels, with an optional time channel or record-index timestamps, bounded uncompressed reads, and compressed-block support when enabled.
- Preserve channel component/array-definition addresses in indexes, including deserialization defaults for older serialized indexes.

## Bug fixes

- **DZ blocks:** Validate compressed lengths against the platform size and declared block length, detect length overflow, and limit decompression to the declared output size. Correct inverse transposition for data whose final row is incomplete.
- **Range readers:** Check range-end arithmetic for overflow and convert requested lengths to `usize` safely in both file-backed and buffered readers.
- **Channel decoding:** Reject invalid integer widths, float widths, and bit offsets instead of risking invalid shifts. Use a wider intermediate when decoding unaligned 64-bit integers, apply bit offsets when decoding floats, and check byte-range arithmetic.
- **Streaming index construction:** Detect cycles in data-group, channel-group, channel, data-block, and fragment chains. Report unexpected data block types and unsupported nested conversions instead of silently stopping or skipping them.
- **Indexed channel reads:** Reject record-ID groups that the reader cannot correctly distinguish, validate record sizes and data-block ranges, and preserve records that span adjacent data blocks.

Regression tests cover unfinalized interleaved OBD recordings, malformed and truncated recovery inputs, malformed DZ lengths, bounded decompression, partial-row transposition, unaligned numeric decoding, cyclic data lists, range overflow, unsupported record IDs, and records split across blocks.
