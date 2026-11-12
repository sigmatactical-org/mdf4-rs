//! Error types for MDF4 operations.
//!
//! This module defines the [`Error`] enum which represents all possible failures
//! that can occur when reading, writing, or processing MDF files.
//!
//! # Example
//!
//! ```no_run
//! # #[cfg(feature = "std")]
//! use mdf4_rs::{MDF, Error, Result};
//!
//! # #[cfg(feature = "std")]
//! fn process_file(path: &str) -> Result<()> {
//!     match MDF::from_file(path) {
//!         Ok(mdf) => {
//!             println!("Loaded {} channel groups", mdf.channel_groups().len());
//!             Ok(())
//!         }
//!         Err(Error::IOError(e)) => {
//!             eprintln!("File I/O error: {}", e);
//!             Err(Error::IOError(e))
//!         }
//!         Err(Error::FileIdentifierError(id)) => {
//!             eprintln!("Not a valid MDF file: {}", id);
//!             Err(Error::FileIdentifierError(id))
//!         }
//!         Err(e) => Err(e),
//!     }
//! }
//! ```

use core::fmt;

#[cfg(feature = "alloc")]
use alloc::string::String;

/// Errors that can occur during MDF file operations.
///
/// This enum covers all failure modes including I/O errors, parsing failures,
/// and structural issues in the MDF file.
#[derive(Debug)]
pub enum Error {
    /// Buffer provided for parsing was too small.
    ///
    /// This typically indicates file corruption or an incomplete read.
    TooShortBuffer {
        /// Actual number of bytes available
        actual: usize,
        /// Minimum number of bytes required
        expected: usize,
        /// Source file where the error was detected
        file: &'static str,
        /// Line number where the error was detected
        line: u32,
    },

    /// The file identifier is not "MDF     " as required by the specification.
    ///
    /// This can occur when trying to open a non-MDF file or a file using an
    /// unsupported variant like "UnFinMF" (unfinalized MDF).
    FileIdentifierError(String),

    /// The MDF version is not supported (requires 4.1 or later).
    FileVersioningError(String),

    /// A block identifier did not match the expected value.
    ///
    /// Each MDF block starts with a 4-character identifier (e.g., "##HD" for
    /// the header block). This error indicates structural corruption.
    BlockIDError {
        /// The identifier that was found
        actual: String,
        /// The identifier that was expected
        expected: String,
    },

    /// An I/O error occurred while reading or writing the file.
    ///
    /// Only available with the `std` feature.
    #[cfg(feature = "std")]
    IOError(std::io::Error),

    /// A write operation failed (no_std version).
    ///
    /// Only available without the `std` feature.
    #[cfg(not(feature = "std"))]
    WriteError,

    /// The version string in the identification block could not be parsed.
    InvalidVersionString(String),

    /// Failed to link blocks together during file writing.
    ///
    /// This typically indicates a programming error where blocks are
    /// referenced before being written.
    BlockLinkError(String),

    /// Failed to serialize a block to bytes.
    BlockSerializationError(String),

    /// A conversion chain exceeded the maximum allowed depth.
    ///
    /// MDF supports chained conversions where one conversion references another.
    /// This error prevents infinite loops from malformed files.
    ConversionChainTooDeep {
        /// The maximum depth that was exceeded
        max_depth: usize,
    },

    /// A cycle was detected in a conversion chain.
    ///
    /// This indicates file corruption where conversion blocks form a loop.
    ConversionChainCycle {
        /// The address where the cycle was detected
        address: u64,
    },

    /// A cycle was detected while walking a linked list of blocks (e.g. the
    /// data-group, channel-group, channel, or data-list chain).
    ///
    /// This indicates file corruption where block links form a loop, which
    /// would otherwise cause the parser to loop forever.
    LinkCycle {
        /// The link address that was visited twice
        address: u64,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::TooShortBuffer {
                actual,
                expected,
                file,
                line,
            } => write!(
                f,
                "Buffer too small at {file}:{line}: need at least {expected} bytes, got {actual}"
            ),
            Error::FileIdentifierError(id) => {
                write!(
                    f,
                    r#"Invalid file identifier: Expected "MDF     ", found {id}"#
                )
            }
            Error::FileVersioningError(ver) => {
                write!(f, r#"File version too low: Expected "> 4.1", found {ver}"#)
            }
            Error::BlockIDError { actual, expected } => {
                write!(
                    f,
                    "Invalid block identifier: Expected {expected:?}, got {actual:?}"
                )
            }
            #[cfg(feature = "std")]
            Error::IOError(e) => write!(f, "I/O error: {e}"),
            #[cfg(not(feature = "std"))]
            Error::WriteError => write!(f, "Write error"),
            Error::InvalidVersionString(s) => write!(f, "Invalid version string: {s}"),
            Error::BlockLinkError(s) => write!(f, "Block linking error: {s}"),
            Error::BlockSerializationError(s) => write!(f, "Block serialization error: {s}"),
            Error::ConversionChainTooDeep { max_depth } => {
                write!(
                    f,
                    "Conversion chain too deep: maximum depth of {max_depth} exceeded"
                )
            }
            Error::ConversionChainCycle { address } => {
                write!(
                    f,
                    "Conversion chain cycle detected at block address {address:#x}"
                )
            }
            Error::LinkCycle { address } => {
                write!(f, "Block link cycle detected at address {address:#x}")
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::IOError(e) => Some(e),
            _ => None,
        }
    }
}

#[cfg(feature = "std")]
impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IOError(err)
    }
}

/// A specialized Result type for MDF operations.
///
/// This is defined as `core::result::Result<T, Error>` for convenience.
pub type Result<T> = core::result::Result<T, Error>;

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::Error;

    #[test]
    fn displays_each_error_variant() {
        let cases = [
            (
                Error::TooShortBuffer {
                    actual: 1,
                    expected: 2,
                    file: "sample",
                    line: 7,
                },
                "Buffer too small at sample:7: need at least 2 bytes, got 1",
            ),
            (
                Error::FileIdentifierError("bad".into()),
                "Invalid file identifier: Expected \"MDF     \", found bad",
            ),
            (
                Error::FileVersioningError("4.0".into()),
                "File version too low: Expected \"> 4.1\", found 4.0",
            ),
            (
                Error::BlockIDError {
                    actual: "##XX".into(),
                    expected: "##HD".into(),
                },
                "Invalid block identifier: Expected \"##HD\", got \"##XX\"",
            ),
            (
                Error::IOError(std::io::Error::other("disk")),
                "I/O error: disk",
            ),
            (
                Error::InvalidVersionString("bad".into()),
                "Invalid version string: bad",
            ),
            (
                Error::BlockLinkError("missing".into()),
                "Block linking error: missing",
            ),
            (
                Error::BlockSerializationError("invalid".into()),
                "Block serialization error: invalid",
            ),
            (
                Error::ConversionChainTooDeep { max_depth: 20 },
                "Conversion chain too deep: maximum depth of 20 exceeded",
            ),
            (
                Error::ConversionChainCycle { address: 16 },
                "Conversion chain cycle detected at block address 0x10",
            ),
            (
                Error::LinkCycle { address: 32 },
                "Block link cycle detected at address 0x20",
            ),
        ];

        for (error, expected) in cases {
            assert_eq!(error.to_string(), expected);
        }
    }

    #[test]
    fn exposes_only_io_errors_as_sources_and_converts_io_errors() {
        let io_error = std::io::Error::other("disk");
        let error = Error::from(io_error);
        assert!(std::error::Error::source(&error).is_some());

        let parse_error = Error::BlockSerializationError("invalid".into());
        assert!(std::error::Error::source(&parse_error).is_none());
    }
}
