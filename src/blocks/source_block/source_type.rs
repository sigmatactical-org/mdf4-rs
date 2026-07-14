//! [`SourceType`].

#[allow(unused_imports)]
use super::*;

/// Source type constants for SourceBlock.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SourceType {
    /// Other source type
    Other = 0,
    /// Electronic Control Unit
    ECU = 1,
    /// Bus (CAN, LIN, etc.)
    Bus = 2,
    /// I/O device
    IO = 3,
    /// Tool
    Tool = 4,
    /// User-defined
    User = 5,
}
