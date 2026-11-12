use crate::{
    Error, Result,
    blocks::common::{
        BlockHeader, BlockParse, debug_assert_aligned, padding_to_align_8, validate_block_id,
        validate_buffer_size,
    },
};
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Metadata Block (##MD) - stores XML metadata.
///
/// Metadata blocks contain XML-formatted metadata that provides additional
/// context about channels, channel groups, or the file itself. The XML
/// follows the ASAM MDF schema.
#[derive(Debug, Clone)]
pub struct MetadataBlock {
    pub header: BlockHeader,
    /// The XML content (without null terminator).
    pub xml: String,
}

impl BlockParse<'_> for MetadataBlock {
    const ID: &'static str = "##MD";

    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let header = Self::parse_header(bytes)?;

        let data_len = (header.length as usize).saturating_sub(24);
        validate_buffer_size(bytes, 24 + data_len)?;

        let data = &bytes[24..24 + data_len];

        // Parse XML efficiently: try UTF-8 first, fall back to lossy conversion
        let xml = match core::str::from_utf8(data) {
            Ok(s) => s.trim_matches('\0').to_string(),
            Err(_) => String::from_utf8_lossy(data).trim_matches('\0').to_string(),
        };

        Ok(Self { header, xml })
    }
}

impl MetadataBlock {
    /// Creates a new MetadataBlock with the provided XML content.
    ///
    /// Automatically calculates the correct block size based on the XML length,
    /// ensuring proper 8-byte alignment.
    pub fn new(xml: &str) -> Self {
        let block_len = Self::calculate_block_len(xml);

        Self {
            header: BlockHeader {
                id: String::from("##MD"),
                reserved: 0,
                length: block_len as u64,
                link_count: 0,
            },
            xml: String::from(xml),
        }
    }

    /// Creates an empty MetadataBlock with a minimal valid size.
    pub fn new_empty() -> Self {
        Self::new("")
    }

    /// Calculates the block length for a given XML string.
    fn calculate_block_len(xml: &str) -> usize {
        let xml_bytes = xml.as_bytes();
        let needs_null = xml_bytes.is_empty() || xml_bytes.last() != Some(&0);
        let xml_size = xml_bytes.len() + if needs_null { 1 } else { 0 };
        let unpadded_size = 24 + xml_size;
        unpadded_size + padding_to_align_8(unpadded_size)
    }

    /// Serializes the MetadataBlock to bytes according to MDF 4.1 specification.
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        validate_block_id(&self.header, "##MD")?;

        let xml_bytes = self.xml.as_bytes();
        let needs_null = xml_bytes.is_empty() || xml_bytes.last() != Some(&0);
        let total_size = Self::calculate_block_len(&self.xml);

        if self.header.length as usize != total_size {
            return Err(Error::BlockSerializationError(format!(
                "MetadataBlock header.length ({}) does not match calculated size ({})",
                self.header.length, total_size
            )));
        }

        let mut buffer = Vec::with_capacity(total_size);

        // Header (24 bytes)
        buffer.extend_from_slice(&self.header.to_bytes()?);

        // XML content
        buffer.extend_from_slice(xml_bytes);
        if needs_null {
            buffer.push(0);
        }

        // Padding to 8-byte alignment
        buffer.resize(total_size, 0);

        debug_assert_aligned(buffer.len());
        Ok(buffer)
    }
}

impl Default for MetadataBlock {
    fn default() -> Self {
        Self::new("")
    }
}

#[cfg(test)]
mod tests {
    use super::MetadataBlock;
    use crate::blocks::common::BlockParse;

    #[test]
    fn constructors_and_binary_round_trip_preserve_xml() {
        let block = MetadataBlock::new("<metadata/>");
        let bytes = block.to_bytes().unwrap();
        let parsed = MetadataBlock::from_bytes(&bytes).unwrap();
        assert_eq!(parsed.xml, "<metadata/>");
        assert_eq!(parsed.header.length as usize, bytes.len());
        assert_eq!(MetadataBlock::new_empty().xml, "");
        assert_eq!(MetadataBlock::default().xml, "");
    }

    #[test]
    fn serialization_and_parsing_reject_invalid_blocks() {
        let mut block = MetadataBlock::new("<m/>");
        block.header.id = "##TX".into();
        assert!(block.to_bytes().is_err());

        let mut block = MetadataBlock::new("<m/>");
        block.header.length += 8;
        assert!(block.to_bytes().is_err());

        let valid = MetadataBlock::new("<m/>").to_bytes().unwrap();
        assert!(MetadataBlock::from_bytes(&valid[..24]).is_err());

        let mut invalid_utf8 = valid;
        invalid_utf8[24] = 0xff;
        let parsed = MetadataBlock::from_bytes(&invalid_utf8).unwrap();
        assert!(parsed.xml.contains('\u{fffd}'));
    }
}
