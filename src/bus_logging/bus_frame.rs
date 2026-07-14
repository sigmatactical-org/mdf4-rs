//! [`BusFrame`].

#[allow(unused_imports)]
use super::*;
use alloc::vec::Vec;

/// Trait for frame types that can be serialized to bytes.
pub trait BusFrame: Clone {
    /// Serialize the frame to bytes for MDF storage.
    fn to_mdf_bytes(&self) -> Vec<u8>;

    /// Get the frame size in bytes.
    fn mdf_size(&self) -> usize {
        self.to_mdf_bytes().len()
    }
}
