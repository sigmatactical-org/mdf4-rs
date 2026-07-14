//! [`BusType`].

#[allow(unused_imports)]
use super::*;

/// Bus type constants for SourceBlock.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum BusType {
    /// No bus
    None = 0,
    /// Other bus type
    Other = 1,
    /// CAN bus
    CAN = 2,
    /// LIN bus
    LIN = 3,
    /// MOST bus
    MOST = 4,
    /// FlexRay
    FlexRay = 5,
    /// K-Line
    KLine = 6,
    /// Ethernet
    Ethernet = 7,
    /// USB
    USB = 8,
}
