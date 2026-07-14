//! [`MergedGroup`].

#[allow(unused_imports)]
use super::*;
use crate::parsing::decoder::DecodedValue;

/// One output channel group accumulated across input files.
pub(crate) struct MergedGroup {
    pub(crate) meta: GroupMeta,
    pub(crate) data: Vec<Vec<DecodedValue>>, // per channel
}
