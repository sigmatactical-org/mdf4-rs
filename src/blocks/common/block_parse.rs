//! [`BlockParse`].

#[allow(unused_imports)]
use super::*;
use crate::{Error, Result};
use alloc::string::ToString;

/// Parse a block of this type from raw MDF bytes.
pub trait BlockParse<'a>: Sized {
    const ID: &'static str;

    fn parse_header(bytes: &[u8]) -> Result<BlockHeader> {
        let header = BlockHeader::from_bytes(&bytes[0..24])?;
        if header.id != Self::ID {
            return Err(Error::BlockIDError {
                actual: header.id.clone(),
                expected: Self::ID.to_string(),
            });
        }
        Ok(header)
    }

    fn from_bytes(bytes: &'a [u8]) -> Result<Self>;
}
