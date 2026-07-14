//! [`IndexedChannel`].

#[allow(unused_imports)]
use super::*;
#[cfg(feature = "compression")]
use crate::blocks::DzBlock;
use crate::blocks::{ConversionBlock, DataType};

/// Metadata for a single channel, containing all information needed to decode values.
///
/// This struct captures the essential channel properties from the MDF file's
/// CN blocks, including data type, bit layout, and conversion formula. It enables
/// decoding channel values without re-parsing the original MDF structure.
///
/// # Bit Layout
///
/// Values are extracted using `byte_offset`, `bit_offset`, and `bit_count`:
/// - `byte_offset`: Starting byte within the record (after record ID)
/// - `bit_offset`: Starting bit within that byte (0-7)
/// - `bit_count`: Total number of bits to read
///
/// # Channel Types
///
/// - **Type 0**: Regular data channel
/// - **Type 1**: Variable Length Signal Data (VLSD)
/// - **Type 2**: Master channel (time, angle, etc.)
/// - **Type 3**: Virtual master channel
/// - **Type 4**: Synchronization channel
/// - **Type 5**: Maximum length channel
/// - **Type 6**: Virtual data channel
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IndexedChannel {
    /// Channel name (e.g., "EngineRPM", "Temperature")
    pub name: Option<String>,
    /// Physical unit (e.g., "rpm", "°C", "m/s")
    pub unit: Option<String>,
    /// Data type determining how raw bytes are interpreted
    pub data_type: DataType,
    /// Byte offset within each record (after record ID bytes)
    pub byte_offset: u32,
    /// Bit offset within the starting byte (0-7)
    pub bit_offset: u8,
    /// Number of bits for this channel's raw value
    pub bit_count: u32,
    /// Channel type (0=data, 1=VLSD, 2=master, etc.)
    pub channel_type: u8,
    /// Channel flags indicating invalidation bit presence and other properties
    pub flags: u32,
    /// Position of invalidation bit within invalidation bytes (if used)
    pub pos_invalidation_bit: u32,
    /// Conversion formula to transform raw values to physical units.
    /// If `None`, raw values are used directly.
    pub conversion: Option<ConversionBlock>,
    /// For VLSD channels: file address of signal data blocks
    pub vlsd_data_address: Option<u64>,
}
