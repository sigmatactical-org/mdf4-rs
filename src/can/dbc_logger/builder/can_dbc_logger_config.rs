//! [`CanDbcLoggerConfig`].

#[allow(unused_imports)]
use super::*;

/// Configuration for CanDbcLogger.
#[derive(Debug, Clone)]
pub struct CanDbcLoggerConfig {
    /// Store raw values with conversion blocks instead of physical values.
    /// Default: false (store physical values as f64)
    pub store_raw_values: bool,

    /// Include unit information in MDF channels.
    /// Default: true
    pub include_units: bool,

    /// Include min/max limits in MDF channels.
    /// Default: true
    pub include_limits: bool,

    /// Include conversion blocks (for raw value mode).
    /// Default: true
    pub include_conversions: bool,

    /// Include value descriptions as ValueToText conversions.
    /// When enabled, DBC VAL_ entries are converted to MDF4 ValueToText blocks.
    /// Default: true
    pub include_value_descriptions: bool,
}
impl Default for CanDbcLoggerConfig {
    fn default() -> Self {
        Self {
            store_raw_values: false,
            include_units: true,
            include_limits: true,
            include_conversions: true,
            include_value_descriptions: true,
        }
    }
}
