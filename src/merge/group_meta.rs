//! [`GroupMeta`].

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct GroupMeta {
    pub(crate) record_id_size: u8,
    pub(crate) channels: Vec<ChannelMeta>,
}
