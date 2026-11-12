/// Represents the conversion type (cc_type) from a conversion block.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ConversionType {
    /// 0: 1:1 conversion (no change)
    Identity,
    /// 1: Linear conversion
    Linear,
    /// 2: Rational conversion
    Rational,
    /// 3: Algebraic conversion (MCD-2 MC text formula)
    Algebraic,
    /// 4: Value to value tabular look-up with interpolation
    TableLookupInterp,
    /// 5: Value to value tabular look-up without interpolation
    TableLookupNoInterp,
    /// 6: Value range to value tabular look-up
    RangeLookup,
    /// 7: Value to text/scale conversion tabular look-up
    ValueToText,
    /// 8: Value range to text/scale conversion tabular look-up
    RangeToText,
    /// 9: Text to value tabular look-up
    TextToValue,
    /// 10: Text to text tabular look-up (translation)
    TextToText,
    /// 11: Bitfield text table
    BitfieldText,
    /// For any other unrecognized conversion type.
    Unknown(u8),
}

impl ConversionType {
    /// Converts a raw u8 value to the corresponding ConversionType.
    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => ConversionType::Identity,
            1 => ConversionType::Linear,
            2 => ConversionType::Rational,
            3 => ConversionType::Algebraic,
            4 => ConversionType::TableLookupInterp,
            5 => ConversionType::TableLookupNoInterp,
            6 => ConversionType::RangeLookup,
            7 => ConversionType::ValueToText,
            8 => ConversionType::RangeToText,
            9 => ConversionType::TextToValue,
            10 => ConversionType::TextToText,
            11 => ConversionType::BitfieldText,
            other => ConversionType::Unknown(other),
        }
    }

    /// Convert the `ConversionType` to its numeric representation.
    pub fn to_u8(self) -> u8 {
        match self {
            ConversionType::Identity => 0,
            ConversionType::Linear => 1,
            ConversionType::Rational => 2,
            ConversionType::Algebraic => 3,
            ConversionType::TableLookupInterp => 4,
            ConversionType::TableLookupNoInterp => 5,
            ConversionType::RangeLookup => 6,
            ConversionType::ValueToText => 7,
            ConversionType::RangeToText => 8,
            ConversionType::TextToValue => 9,
            ConversionType::TextToText => 10,
            ConversionType::BitfieldText => 11,
            ConversionType::Unknown(v) => v,
        }
    }
}

impl core::fmt::Display for ConversionType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ConversionType::Identity => write!(f, "identity"),
            ConversionType::Linear => write!(f, "linear"),
            ConversionType::Rational => write!(f, "rational"),
            ConversionType::Algebraic => write!(f, "algebraic"),
            ConversionType::TableLookupInterp => write!(f, "table (interpolated)"),
            ConversionType::TableLookupNoInterp => write!(f, "table"),
            ConversionType::RangeLookup => write!(f, "range lookup"),
            ConversionType::ValueToText => write!(f, "value to text"),
            ConversionType::RangeToText => write!(f, "range to text"),
            ConversionType::TextToValue => write!(f, "text to value"),
            ConversionType::TextToText => write!(f, "text to text"),
            ConversionType::BitfieldText => write!(f, "bitfield text"),
            ConversionType::Unknown(v) => write!(f, "unknown({})", v),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ConversionType;

    #[test]
    fn maps_and_formats_all_known_conversion_types() {
        let expected = [
            "identity",
            "linear",
            "rational",
            "algebraic",
            "table (interpolated)",
            "table",
            "range lookup",
            "value to text",
            "range to text",
            "text to value",
            "text to text",
            "bitfield text",
        ];

        for (value, display) in expected.into_iter().enumerate() {
            let conversion = ConversionType::from_u8(value as u8);
            assert_eq!(conversion.to_u8(), value as u8);
            assert_eq!(conversion.to_string(), display);
        }

        let unknown = ConversionType::from_u8(42);
        assert_eq!(unknown, ConversionType::Unknown(42));
        assert_eq!(unknown.to_u8(), 42);
        assert_eq!(unknown.to_string(), "unknown(42)");
    }
}
