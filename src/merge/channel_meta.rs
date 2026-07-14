//! [`ChannelMeta`].

#[allow(unused_imports)]
use super::*;
use crate::blocks::DataType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ChannelMeta {
    pub(crate) name: Option<String>,
    pub(crate) data_type: DataType,
    pub(crate) bit_offset: u8,
    pub(crate) byte_offset: u32,
    pub(crate) bit_count: u32,
    pub(crate) channel_type: u8,
}
