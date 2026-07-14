//! [`MessageInfo`].

#[allow(unused_imports)]
use super::*;

/// Information about a DBC message for MDF4 channel group creation.
#[allow(dead_code)] // Public API for library users working with DBC message metadata
#[derive(Debug, Clone)]
pub struct MessageInfo {
    /// CAN message ID
    pub id: u32,
    /// Message name
    pub name: alloc::string::String,
    /// Data length code (bytes)
    pub dlc: u8,
    /// Transmitting ECU name
    pub sender: alloc::string::String,
    /// Signal information for each signal in the message
    pub signals: alloc::vec::Vec<SignalInfo>,
    /// Whether this is an extended (29-bit) CAN ID
    pub is_extended: bool,
}
#[allow(dead_code)] // Public API methods for library users
impl MessageInfo {
    /// Creates MessageInfo from a DBC message.
    pub fn from_message(message: &dbc_rs::Message) -> Self {
        let id = message.id();
        let is_extended = (id & 0x8000_0000) != 0;
        let raw_id = if is_extended { id & 0x1FFF_FFFF } else { id };

        Self {
            id: raw_id,
            name: alloc::string::String::from(message.name()),
            dlc: message.dlc(),
            sender: alloc::string::String::from(message.sender()),
            signals: message
                .signals()
                .iter()
                .map(SignalInfo::from_signal)
                .collect(),
            is_extended,
        }
    }

    /// Get the number of signals in this message.
    pub fn signal_count(&self) -> usize {
        self.signals.len()
    }
}
