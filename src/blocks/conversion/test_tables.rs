use super::base::ConversionBlock;
use super::types::ConversionType;
use crate::blocks::text_block::TextBlock;
use crate::types::DecodedValue;
use std::collections::BTreeMap;

fn conversion(conversion_type: ConversionType) -> ConversionBlock {
    let mut block = ConversionBlock::identity();
    block.conversion_type = conversion_type;
    block
}

fn add_text(data: &mut Vec<u8>, text: &str) -> u64 {
    let address = data.len() as u64;
    data.extend_from_slice(&TextBlock::new(text).to_bytes().unwrap());
    address
}

#[test]
fn table_lookup_interpolates_clamps_and_uses_nearest_neighbor() {
    use super::table_lookup::{apply_table_lookup, lookup_table};

    let table = [0.0, 0.0, 10.0, 100.0];
    assert_eq!(lookup_table(&table, -1.0, true), Some(0.0));
    assert_eq!(lookup_table(&table, 5.0, true), Some(50.0));
    assert_eq!(lookup_table(&table, 11.0, true), Some(100.0));
    assert_eq!(lookup_table(&table, 4.0, false), Some(0.0));
    assert_eq!(lookup_table(&table, 6.0, false), Some(100.0));
    assert_eq!(lookup_table(&table, 5.0, false), Some(0.0));
    assert_eq!(lookup_table(&[0.0, 1.0, 2.0], 1.0, true), None);

    let mut block = conversion(ConversionType::TableLookupInterp);
    block.values = table.to_vec();
    assert_eq!(
        apply_table_lookup(&block, DecodedValue::Float(5.0), true).unwrap(),
        DecodedValue::Float(50.0)
    );
    block.values.clear();
    assert_eq!(
        apply_table_lookup(&block, DecodedValue::SignedInteger(4), true).unwrap(),
        DecodedValue::Float(4.0)
    );
    assert_eq!(
        apply_table_lookup(&block, DecodedValue::String("x".into()), true).unwrap(),
        DecodedValue::String("x".into())
    );
}

#[test]
fn range_lookup_distinguishes_integer_and_float_upper_bounds() {
    use super::table_lookup::apply_range_lookup;

    let mut block = conversion(ConversionType::RangeLookup);
    block.values = vec![0.0, 10.0, 1.0, 10.0, 20.0, 2.0, -1.0];
    assert_eq!(
        apply_range_lookup(&block, DecodedValue::UnsignedInteger(10)).unwrap(),
        DecodedValue::Float(1.0)
    );
    assert_eq!(
        apply_range_lookup(&block, DecodedValue::Float(10.0)).unwrap(),
        DecodedValue::Float(2.0)
    );
    assert_eq!(
        apply_range_lookup(&block, DecodedValue::Float(30.0)).unwrap(),
        DecodedValue::Float(-1.0)
    );
    block.values.pop();
    assert_eq!(
        apply_range_lookup(&block, DecodedValue::Float(3.0)).unwrap(),
        DecodedValue::Float(3.0)
    );
    assert_eq!(
        apply_range_lookup(&block, DecodedValue::ByteArray(vec![1])).unwrap(),
        DecodedValue::ByteArray(vec![1])
    );
}

#[test]
fn value_and_range_to_text_use_resolved_and_legacy_texts() {
    use super::text::{apply_range_to_text, apply_value_to_text, find_range_to_text_index};

    assert_eq!(find_range_to_text_index(&[], 1.0, true), 0);
    assert_eq!(find_range_to_text_index(&[0.0, 1.0, 1.0], 1.0, true), 0);
    assert_eq!(find_range_to_text_index(&[0.0, 1.0], 1.0, false), 1);

    let mut value_block = conversion(ConversionType::ValueToText);
    value_block.values = vec![1.0];
    value_block.resolved_texts = Some(BTreeMap::from([(0, "resolved".into())]));
    assert_eq!(
        apply_value_to_text(&value_block, DecodedValue::Float(1.0), &[]).unwrap(),
        DecodedValue::String("resolved".into())
    );
    assert_eq!(
        apply_value_to_text(&value_block, DecodedValue::String("input".into()), &[]).unwrap(),
        DecodedValue::String("input".into())
    );

    let mut data = vec![0; 24];
    let text_address = add_text(&mut data, "legacy");
    let mut legacy = conversion(ConversionType::ValueToText);
    legacy.values = vec![7.0];
    legacy.refs = vec![text_address];
    assert_eq!(
        apply_value_to_text(&legacy, DecodedValue::UnsignedInteger(7), &data).unwrap(),
        DecodedValue::String("legacy".into())
    );
    assert_eq!(
        apply_value_to_text(&legacy, DecodedValue::Float(9.0), &[]).unwrap(),
        DecodedValue::Unknown
    );

    let mut range = conversion(ConversionType::RangeToText);
    range.values = vec![0.0, 5.0];
    range.resolved_texts = Some(BTreeMap::from([(0, "range".into()), (1, "default".into())]));
    assert_eq!(
        apply_range_to_text(&range, DecodedValue::Float(5.0), &[]).unwrap(),
        DecodedValue::String("default".into())
    );
    assert_eq!(
        apply_range_to_text(&range, DecodedValue::SignedInteger(5), &[]).unwrap(),
        DecodedValue::String("range".into())
    );
    range.resolved_texts = None;
    range.refs = vec![text_address];
    assert_eq!(
        apply_range_to_text(&range, DecodedValue::Float(2.0), &data).unwrap(),
        DecodedValue::String("legacy".into())
    );
}

#[test]
fn legacy_text_conversions_cover_nested_and_default_paths() {
    use super::text::{
        apply_range_to_text, apply_text_to_text, apply_text_to_value, apply_value_to_text,
    };

    let mut conversion_data = vec![0; 24];
    let nested_address = conversion_data.len() as u64;
    conversion_data.extend_from_slice(&ConversionBlock::linear(1.0, 2.0).to_bytes().unwrap());

    let mut value_block = conversion(ConversionType::ValueToText);
    value_block.values = vec![1.0];
    value_block.refs = vec![nested_address];
    assert_eq!(
        apply_value_to_text(&value_block, DecodedValue::Float(1.0), &conversion_data).unwrap(),
        DecodedValue::Float(3.0)
    );

    let mut range_block = conversion(ConversionType::RangeToText);
    range_block.values = vec![0.0, 2.0];
    range_block.refs = vec![nested_address];
    assert_eq!(
        apply_range_to_text(&range_block, DecodedValue::Float(1.0), &conversion_data).unwrap(),
        DecodedValue::Float(3.0)
    );

    let mut text_data = vec![0; 24];
    let key_address = add_text(&mut text_data, "on");
    let output_address = add_text(&mut text_data, "enabled");
    let default_address = add_text(&mut text_data, "unknown");

    let mut text_value = conversion(ConversionType::TextToValue);
    text_value.refs = vec![key_address];
    text_value.values = vec![1.0, -1.0];
    assert_eq!(
        apply_text_to_value(&text_value, DecodedValue::String("on".into()), &text_data).unwrap(),
        DecodedValue::Float(1.0)
    );
    assert_eq!(
        apply_text_to_value(&text_value, DecodedValue::String("off".into()), &text_data).unwrap(),
        DecodedValue::Float(-1.0)
    );

    let mut text_text = conversion(ConversionType::TextToText);
    text_text.refs = vec![key_address, output_address, default_address];
    assert_eq!(
        apply_text_to_text(&text_text, DecodedValue::String("on".into()), &text_data).unwrap(),
        DecodedValue::String("enabled".into())
    );
    assert_eq!(
        apply_text_to_text(&text_text, DecodedValue::String("off".into()), &text_data).unwrap(),
        DecodedValue::String("unknown".into())
    );
}

#[test]
fn text_to_value_and_text_to_text_support_resolved_mappings_and_defaults() {
    use super::text::{apply_text_to_text, apply_text_to_value};

    let mut value_block = conversion(ConversionType::TextToValue);
    value_block.refs = vec![1, 2];
    value_block.values = vec![10.0, 20.0, -1.0];
    value_block.resolved_texts = Some(BTreeMap::from([(0, "on".into()), (1, "off".into())]));
    assert_eq!(
        apply_text_to_value(&value_block, DecodedValue::String("off".into()), &[]).unwrap(),
        DecodedValue::Float(20.0)
    );
    assert_eq!(
        apply_text_to_value(&value_block, DecodedValue::String("other".into()), &[]).unwrap(),
        DecodedValue::Float(-1.0)
    );
    assert_eq!(
        apply_text_to_value(&value_block, DecodedValue::Float(3.0), &[]).unwrap(),
        DecodedValue::Float(3.0)
    );

    let mut text_block = conversion(ConversionType::TextToText);
    text_block.refs = vec![1, 2, 3];
    text_block.resolved_texts = Some(BTreeMap::from([
        (0, "key".into()),
        (1, "translated".into()),
        (2, "fallback".into()),
    ]));
    assert_eq!(
        apply_text_to_text(&text_block, DecodedValue::String("key".into()), &[]).unwrap(),
        DecodedValue::String("translated".into())
    );
    assert_eq!(
        apply_text_to_text(&text_block, DecodedValue::String("missing".into()), &[]).unwrap(),
        DecodedValue::String("fallback".into())
    );
    assert_eq!(
        apply_text_to_text(&text_block, DecodedValue::UnsignedInteger(1), &[]).unwrap(),
        DecodedValue::UnsignedInteger(1)
    );
}

#[test]
fn bitfield_text_combines_resolved_conversion_results_and_fallbacks() {
    use super::bitfield::apply_bitfield_text;

    let mut nested = conversion(ConversionType::ValueToText);
    nested.values = vec![1.0];
    nested.resolved_texts = Some(BTreeMap::from([(0, "enabled".into())]));

    let mut block = conversion(ConversionType::BitfieldText);
    block.values = vec![f64::from_bits(1)];
    block.refs = vec![1];
    block.resolved_conversions = Some(BTreeMap::from([(0, Box::new(nested.clone()))]));
    assert_eq!(
        apply_bitfield_text(&block, DecodedValue::UnsignedInteger(1), &[]).unwrap(),
        DecodedValue::String("enabled".into())
    );
    assert_eq!(
        apply_bitfield_text(&block, DecodedValue::String("input".into()), &[]).unwrap(),
        DecodedValue::String("input".into())
    );

    block.resolved_conversions = None;
    block.default_conversion = Some(Box::new(nested));
    assert_eq!(
        apply_bitfield_text(&block, DecodedValue::SignedInteger(1), &[]).unwrap(),
        DecodedValue::String("enabled".into())
    );
}

#[test]
fn bitfield_text_reads_legacy_nested_conversion_and_text_blocks() {
    use super::bitfield::apply_bitfield_text;

    let mut data = vec![0; 24];
    let conversion_address = data.len() as u64;
    let text_address = conversion_address + 96;
    let mut nested = conversion(ConversionType::ValueToText);
    nested.values = vec![1.0];
    nested.value_count = 1;
    nested.refs = vec![text_address];
    nested.ref_count = 1;
    data.extend_from_slice(&nested.to_bytes().unwrap());
    data.resize(text_address as usize, 0);
    add_text(&mut data, "enabled");

    let mut block = conversion(ConversionType::BitfieldText);
    block.values = vec![f64::from_bits(1)];
    block.refs = vec![conversion_address];
    assert_eq!(
        apply_bitfield_text(&block, DecodedValue::UnsignedInteger(1), &data).unwrap(),
        DecodedValue::String("enabled".into())
    );
}
