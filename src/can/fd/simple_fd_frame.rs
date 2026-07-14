//! [`SimpleFdFrame`].

#[allow(unused_imports)]
use super::*;

/// A simple CAN FD frame implementation.
#[cfg(feature = "can")]
#[derive(Debug, Clone)]
pub struct SimpleFdFrame {
    id: embedded_can::Id,
    data: [u8; MAX_FD_DATA_LEN],
    len: usize,
    flags: FdFlags,
    is_fd: bool,
}
#[cfg(feature = "can")]
impl SimpleFdFrame {
    /// Create a new classic CAN frame (up to 8 bytes).
    pub fn new_classic(id: impl Into<embedded_can::Id>, data: &[u8]) -> Option<Self> {
        if data.len() > 8 {
            return None;
        }
        let mut frame_data = [0u8; MAX_FD_DATA_LEN];
        frame_data[..data.len()].copy_from_slice(data);
        Some(Self {
            id: id.into(),
            data: frame_data,
            len: data.len(),
            flags: FdFlags::default(),
            is_fd: false,
        })
    }

    /// Create a new CAN FD frame (up to 64 bytes).
    pub fn new_fd_frame(
        id: impl Into<embedded_can::Id>,
        data: &[u8],
        flags: FdFlags,
    ) -> Option<Self> {
        if data.len() > MAX_FD_DATA_LEN {
            return None;
        }
        let mut frame_data = [0u8; MAX_FD_DATA_LEN];
        frame_data[..data.len()].copy_from_slice(data);
        Some(Self {
            id: id.into(),
            data: frame_data,
            len: data.len(),
            flags,
            is_fd: true,
        })
    }
}
#[cfg(feature = "can")]
impl FdFrame for SimpleFdFrame {
    fn new_fd(id: impl Into<embedded_can::Id>, data: &[u8], flags: FdFlags) -> Option<Self> {
        Self::new_fd_frame(id, data, flags)
    }

    fn is_fd(&self) -> bool {
        self.is_fd
    }

    fn fd_flags(&self) -> FdFlags {
        self.flags
    }

    fn id(&self) -> embedded_can::Id {
        self.id
    }

    fn dlc(&self) -> usize {
        len_to_dlc(self.len) as usize
    }

    fn data(&self) -> &[u8] {
        &self.data[..self.len]
    }
}
// Implement embedded_can::Frame for SimpleFdFrame for classic CAN compatibility
#[cfg(feature = "can")]
impl embedded_can::Frame for SimpleFdFrame {
    fn new(id: impl Into<embedded_can::Id>, data: &[u8]) -> Option<Self> {
        Self::new_classic(id, data)
    }

    fn new_remote(id: impl Into<embedded_can::Id>, dlc: usize) -> Option<Self> {
        if dlc > 8 {
            return None;
        }
        Some(Self {
            id: id.into(),
            data: [0u8; MAX_FD_DATA_LEN],
            len: dlc,
            flags: FdFlags::default(),
            is_fd: false,
        })
    }

    fn is_extended(&self) -> bool {
        matches!(self.id, embedded_can::Id::Extended(_))
    }

    fn is_remote_frame(&self) -> bool {
        false // CAN FD doesn't support remote frames
    }

    fn id(&self) -> embedded_can::Id {
        self.id
    }

    fn dlc(&self) -> usize {
        self.len.min(8)
    }

    fn data(&self) -> &[u8] {
        &self.data[..self.len.min(8)]
    }
}
