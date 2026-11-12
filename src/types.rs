//! Shared types used across the library.
//!
//! This module contains types that are available with just the `alloc` feature,
//! making them usable in both std and no_std environments.

use alloc::string::String;
use alloc::vec::Vec;

/// An enum representing the decoded value of a channel sample.
///
/// This type represents all possible values that can be stored in an MDF channel.
#[derive(Debug, Clone, PartialEq)]
pub enum DecodedValue {
    /// Unsigned integer (up to 64 bits)
    UnsignedInteger(u64),
    /// Signed integer (up to 64 bits)
    SignedInteger(i64),
    /// Floating point value (32 or 64 bit)
    Float(f64),
    /// Text string (UTF-8 or converted from Latin-1)
    String(String),
    /// Raw byte array
    ByteArray(Vec<u8>),
    /// MIME sample data
    MimeSample(Vec<u8>),
    /// MIME stream data
    MimeStream(Vec<u8>),
    /// Unknown or unsupported data type
    Unknown,
}

impl DecodedValue {
    /// Returns true if this is an integer value (signed or unsigned).
    #[inline]
    pub fn is_integer(&self) -> bool {
        matches!(
            self,
            DecodedValue::UnsignedInteger(_) | DecodedValue::SignedInteger(_)
        )
    }

    /// Returns true if this is a floating point value.
    #[inline]
    pub fn is_float(&self) -> bool {
        matches!(self, DecodedValue::Float(_))
    }

    /// Returns true if this is a string value.
    #[inline]
    pub fn is_string(&self) -> bool {
        matches!(self, DecodedValue::String(_))
    }

    /// Returns true if this is a byte array value.
    #[inline]
    pub fn is_bytes(&self) -> bool {
        matches!(
            self,
            DecodedValue::ByteArray(_) | DecodedValue::MimeSample(_) | DecodedValue::MimeStream(_)
        )
    }

    /// Attempts to convert to f64, useful for numeric operations.
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            DecodedValue::UnsignedInteger(v) => Some(*v as f64),
            DecodedValue::SignedInteger(v) => Some(*v as f64),
            DecodedValue::Float(v) => Some(*v),
            _ => None,
        }
    }
}

impl core::fmt::Display for DecodedValue {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            DecodedValue::UnsignedInteger(v) => write!(f, "{}", v),
            DecodedValue::SignedInteger(v) => write!(f, "{}", v),
            DecodedValue::Float(v) => write!(f, "{}", v),
            DecodedValue::String(s) => write!(f, "{}", s),
            DecodedValue::ByteArray(b) => write!(f, "[{} bytes]", b.len()),
            DecodedValue::MimeSample(b) => write!(f, "[MIME sample: {} bytes]", b.len()),
            DecodedValue::MimeStream(b) => write!(f, "[MIME stream: {} bytes]", b.len()),
            DecodedValue::Unknown => write!(f, "<unknown>"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DecodedValue;

    #[test]
    fn identifies_value_categories_and_numeric_values() {
        let unsigned = DecodedValue::UnsignedInteger(5);
        let signed = DecodedValue::SignedInteger(-2);
        let float = DecodedValue::Float(1.5);
        let text = DecodedValue::String("sample".into());
        let bytes = DecodedValue::ByteArray(vec![1, 2]);
        let mime_sample = DecodedValue::MimeSample(vec![1]);
        let mime_stream = DecodedValue::MimeStream(vec![1, 2, 3]);
        let unknown = DecodedValue::Unknown;

        assert!(unsigned.is_integer());
        assert!(signed.is_integer());
        assert!(float.is_float());
        assert!(text.is_string());
        assert!(bytes.is_bytes());
        assert!(mime_sample.is_bytes());
        assert!(mime_stream.is_bytes());
        assert!(!unknown.is_integer());
        assert!(!unknown.is_float());
        assert!(!unknown.is_string());
        assert!(!unknown.is_bytes());
        assert_eq!(unsigned.as_f64(), Some(5.0));
        assert_eq!(signed.as_f64(), Some(-2.0));
        assert_eq!(float.as_f64(), Some(1.5));
        assert_eq!(text.as_f64(), None);
        assert_eq!(bytes.as_f64(), None);
    }

    #[test]
    fn formats_each_decoded_value_variant() {
        assert_eq!(DecodedValue::UnsignedInteger(3).to_string(), "3");
        assert_eq!(DecodedValue::SignedInteger(-3).to_string(), "-3");
        assert_eq!(DecodedValue::Float(1.25).to_string(), "1.25");
        assert_eq!(DecodedValue::String("ok".into()).to_string(), "ok");
        assert_eq!(DecodedValue::ByteArray(vec![1, 2]).to_string(), "[2 bytes]");
        assert_eq!(
            DecodedValue::MimeSample(vec![1]).to_string(),
            "[MIME sample: 1 bytes]"
        );
        assert_eq!(
            DecodedValue::MimeStream(vec![1, 2]).to_string(),
            "[MIME stream: 2 bytes]"
        );
        assert_eq!(DecodedValue::Unknown.to_string(), "<unknown>");
    }
}
