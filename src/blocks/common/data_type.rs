//! [`DataType`].

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DataType {
    UnsignedIntegerLE,
    UnsignedIntegerBE,
    SignedIntegerLE,
    SignedIntegerBE,
    FloatLE,
    FloatBE,
    StringLatin1,
    StringUtf8,
    StringUtf16LE,
    StringUtf16BE,
    ByteArray,
    MimeSample,
    MimeStream,
    CanOpenDate,
    CanOpenTime,
    ComplexLE,
    ComplexBE,
    Unknown(()),
}
impl DataType {
    /// Converts the DataType enum value to its corresponding u8 representation
    /// according to the MDF 4.1 specification.
    ///
    /// # Returns
    /// The u8 value corresponding to this DataType
    ///
    /// # Note
    /// For ComplexLE, ComplexBE, and Unknown variants, we use values that match
    /// the MDF 4.1 specification (15, 16) or a default (0) for Unknown.
    pub fn to_u8(&self) -> u8 {
        match self {
            DataType::UnsignedIntegerLE => 0,
            DataType::UnsignedIntegerBE => 1,
            DataType::SignedIntegerLE => 2,
            DataType::SignedIntegerBE => 3,
            DataType::FloatLE => 4,
            DataType::FloatBE => 5,
            DataType::StringLatin1 => 6,
            DataType::StringUtf8 => 7,
            DataType::StringUtf16LE => 8,
            DataType::StringUtf16BE => 9,
            DataType::ByteArray => 10,
            DataType::MimeSample => 11,
            DataType::MimeStream => 12,
            DataType::CanOpenDate => 13,
            DataType::CanOpenTime => 14,
            DataType::ComplexLE => 15, // Complex numbers, little-endian
            DataType::ComplexBE => 16, // Complex numbers, big-endian
            DataType::Unknown(_) => 0, // Default to 0 for unknown types
        }
    }

    /// Convert a numeric representation to the corresponding `DataType`.
    /// Values outside the known range yield `DataType::Unknown`.
    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => DataType::UnsignedIntegerLE,
            1 => DataType::UnsignedIntegerBE,
            2 => DataType::SignedIntegerLE,
            3 => DataType::SignedIntegerBE,
            4 => DataType::FloatLE,
            5 => DataType::FloatBE,
            6 => DataType::StringLatin1,
            7 => DataType::StringUtf8,
            8 => DataType::StringUtf16LE,
            9 => DataType::StringUtf16BE,
            10 => DataType::ByteArray,
            11 => DataType::MimeSample,
            12 => DataType::MimeStream,
            13 => DataType::CanOpenDate,
            14 => DataType::CanOpenTime,
            15 => DataType::ComplexLE,
            16 => DataType::ComplexBE,
            _ => DataType::Unknown(()),
        }
    }

    /// Returns a typical bit width for this data type.
    /// This is used when creating channels without an explicit bit count.
    pub fn default_bits(&self) -> u32 {
        match self {
            DataType::UnsignedIntegerLE
            | DataType::UnsignedIntegerBE
            | DataType::SignedIntegerLE
            | DataType::SignedIntegerBE => 32,
            DataType::FloatLE | DataType::FloatBE => 32,
            DataType::StringLatin1
            | DataType::StringUtf8
            | DataType::StringUtf16LE
            | DataType::StringUtf16BE
            | DataType::ByteArray
            | DataType::MimeSample
            | DataType::MimeStream => 8,
            DataType::CanOpenDate | DataType::CanOpenTime => 64,
            DataType::ComplexLE | DataType::ComplexBE => 64,
            DataType::Unknown(_) => 8,
        }
    }
}
impl core::fmt::Display for DataType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            DataType::UnsignedIntegerLE => write!(f, "uint (LE)"),
            DataType::UnsignedIntegerBE => write!(f, "uint (BE)"),
            DataType::SignedIntegerLE => write!(f, "int (LE)"),
            DataType::SignedIntegerBE => write!(f, "int (BE)"),
            DataType::FloatLE => write!(f, "float (LE)"),
            DataType::FloatBE => write!(f, "float (BE)"),
            DataType::StringLatin1 => write!(f, "string (Latin-1)"),
            DataType::StringUtf8 => write!(f, "string (UTF-8)"),
            DataType::StringUtf16LE => write!(f, "string (UTF-16 LE)"),
            DataType::StringUtf16BE => write!(f, "string (UTF-16 BE)"),
            DataType::ByteArray => write!(f, "byte array"),
            DataType::MimeSample => write!(f, "MIME sample"),
            DataType::MimeStream => write!(f, "MIME stream"),
            DataType::CanOpenDate => write!(f, "CANopen date"),
            DataType::CanOpenTime => write!(f, "CANopen time"),
            DataType::ComplexLE => write!(f, "complex (LE)"),
            DataType::ComplexBE => write!(f, "complex (BE)"),
            DataType::Unknown(_) => write!(f, "unknown"),
        }
    }
}
