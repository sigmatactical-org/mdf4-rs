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

#[cfg(test)]
mod tests {
    use super::BusFrame;
    use alloc::vec;
    use alloc::vec::Vec;

    #[derive(Clone)]
    struct Frame(Vec<u8>);

    impl BusFrame for Frame {
        fn to_mdf_bytes(&self) -> Vec<u8> {
            self.0.clone()
        }
    }

    #[test]
    fn default_size_matches_serialized_frame_length() {
        assert_eq!(Frame(vec![1, 2, 3]).mdf_size(), 3);
    }
}
