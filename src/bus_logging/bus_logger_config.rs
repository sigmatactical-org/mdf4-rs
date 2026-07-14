//! [`BusLoggerConfig`].

#[allow(unused_imports)]
use super::*;
use crate::blocks::SourceBlock;
use alloc::string::String;

/// Common configuration for bus loggers.
pub struct BusLoggerConfig {
    /// The source/bus/interface name.
    pub source_name: String,
    /// Channel group name pattern (e.g., "{source}_CAN_DataFrame").
    pub group_name: String,
    /// Data channel name (e.g., "CAN_DataFrame", "LIN_Frame").
    pub data_channel_name: String,
    /// Size of the data channel in bits.
    pub data_channel_bits: u32,
    /// Source block for the bus type.
    pub source_block: SourceBlock,
}
