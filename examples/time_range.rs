use mdf4_rs::index::{FileRangeReader, MdfIndex};
use mdf4_rs::{DataType, DecodedValue, MdfWriter, Result};

fn main() -> Result<()> {
    let mdf_file = "time_range_example.mf4";
    let mut writer = MdfWriter::new(mdf_file)?;
    writer.init_mdf_file()?;

    let group_id = writer.add_channel_group(None, |_| {})?;
    let time_channel_id = writer.add_channel(&group_id, None, |channel| {
        channel.data_type = DataType::FloatLE;
        channel.name = Some("Time".to_string());
        channel.bit_count = 64;
    })?;
    writer.set_time_channel(&time_channel_id)?;

    writer.add_channel(&group_id, Some(&time_channel_id), |channel| {
        channel.data_type = DataType::UnsignedIntegerLE;
        channel.name = Some("Temperature".to_string());
        channel.bit_count = 16;
    })?;

    writer.start_data_block_for_cg(&group_id, 0)?;
    for record in 0..10 {
        writer.write_record(
            &group_id,
            &[
                DecodedValue::Float(record as f64 * 0.1),
                DecodedValue::UnsignedInteger(20 + record),
            ],
        )?;
    }
    writer.finish_data_block(&group_id)?;
    writer.finalize()?;

    let index = MdfIndex::from_file(mdf_file)?;
    let mut reader = FileRangeReader::new(mdf_file)?;
    let data = index.read_group_time_range(0, &[1], Some(0), Some(0.3), Some(0.5), &mut reader)?;

    for (time, value) in data.times.iter().zip(&data.values[0]) {
        println!("time={time:.1}, temperature={value:?}");
    }

    std::fs::remove_file(mdf_file)?;
    Ok(())
}
