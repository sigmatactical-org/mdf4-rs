//! [`SignalInfo`].

#[allow(unused_imports)]
use super::*;
use crate::DataType;
use crate::blocks::ConversionBlock;

/// Information about a DBC signal for MDF4 channel creation.
///
/// This struct bundles all the information needed to create an MDF4 channel
/// from a DBC signal definition.
#[derive(Debug, Clone)]
pub struct SignalInfo {
    /// Signal name
    pub name: alloc::string::String,
    /// Physical unit (e.g., "rpm", "°C")
    pub unit: Option<alloc::string::String>,
    /// MDF4 data type for raw values
    pub data_type: DataType,
    /// Bit count for raw value storage
    pub bit_count: u32,
    /// Linear conversion (if needed)
    pub conversion: Option<ConversionBlock>,
    /// Physical minimum value
    pub min: f64,
    /// Physical maximum value
    pub max: f64,
    /// DBC factor for manual decoding
    #[allow(dead_code)] // Used by raw_to_physical/physical_to_raw methods
    pub factor: f64,
    /// DBC offset for manual decoding
    #[allow(dead_code)] // Used by raw_to_physical/physical_to_raw methods
    pub offset: f64,
    /// Whether the signal is unsigned
    pub unsigned: bool,
}
impl SignalInfo {
    /// Creates SignalInfo from a DBC signal.
    ///
    /// Extracts all relevant information from the signal and converts
    /// it to MDF4-compatible formats.
    pub fn from_signal(signal: &dbc_rs::Signal) -> Self {
        Self {
            name: alloc::string::String::from(signal.name()),
            unit: signal.unit().map(alloc::string::String::from),
            data_type: signal_to_data_type(signal),
            bit_count: signal_to_bit_count(signal),
            conversion: signal_to_conversion_with_range(signal),
            min: signal.min(),
            max: signal.max(),
            factor: signal.factor(),
            offset: signal.offset(),
            unsigned: signal.is_unsigned(),
        }
    }

    /// Check if this signal needs a conversion block.
    #[allow(dead_code)] // Public API for library users
    pub fn needs_conversion(&self) -> bool {
        self.conversion.is_some()
    }

    /// Check if this is an identity conversion (factor=1, offset=0).
    #[allow(dead_code)] // Public API for library users
    pub fn is_identity(&self) -> bool {
        self.factor == 1.0 && self.offset == 0.0
    }

    /// Convert a raw integer value to physical value.
    #[allow(dead_code)] // Public API for library users
    #[inline]
    pub fn raw_to_physical(&self, raw: i64) -> f64 {
        self.offset + self.factor * (raw as f64)
    }

    /// Convert a physical value to raw integer value.
    #[allow(dead_code)] // Public API for library users
    #[inline]
    pub fn physical_to_raw(&self, physical: f64) -> i64 {
        let raw = (physical - self.offset) / self.factor;
        // Round to nearest integer (no_std compatible)
        if raw >= 0.0 {
            (raw + 0.5) as i64
        } else {
            (raw - 0.5) as i64
        }
    }
}
