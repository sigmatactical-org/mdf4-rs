//! [`ScheduleEntryType`].

#[allow(unused_imports)]
use super::*;

/// LIN schedule table entry type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ScheduleEntryType {
    /// Unconditional frame.
    Unconditional = 0,
    /// Event-triggered frame.
    EventTriggered = 1,
    /// Sporadic frame.
    Sporadic = 2,
    /// Diagnostic request (Master Request Frame).
    DiagnosticRequest = 3,
    /// Diagnostic response (Slave Response Frame).
    DiagnosticResponse = 4,
}
