use mdf4_rs::index::{BufferedRangeReader, RecoveredMdfIndex};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let path = args
        .next()
        .unwrap_or_else(|| "tests/data/11-bit-obd2.MF4".to_string());
    let group: usize = args.next().as_deref().unwrap_or("0").parse()?;
    let channel: usize = args.next().as_deref().unwrap_or("0").parse()?;
    let time_range = match args.next() {
        None => None,
        Some(time) => {
            let start = args.next().ok_or("missing range start")?.parse::<f64>()?;
            let end = args.next().ok_or("missing range end")?.parse::<f64>()?;
            Some((time.parse::<usize>()?, start, end))
        }
    };
    if args.next().is_some() {
        return Err(
            "usage: recover_unfinalized [path [group channel [time-channel start end]]]".into(),
        );
    }

    let index = RecoveredMdfIndex::from_file(&path)?;
    let metadata = index
        .channel_groups()
        .get(group)
        .ok_or("invalid group index")?;
    let selected = metadata
        .channels
        .get(channel)
        .ok_or("invalid channel index")?;
    println!(
        "Group {group}: {:?}, {} records; channel {channel}: {:?}",
        metadata.name, metadata.record_count, selected.name
    );

    let mut reader = BufferedRangeReader::new(&path)?;
    let records =
        index.read_group_time_range(group, &[channel], None, Some(0.0), Some(4.0), &mut reader)?;
    println!("First five record indices and values:");
    for (record, value) in records.times.iter().zip(&records.values[0]) {
        println!("  {record}: {value:?}");
    }

    if let Some((time, start, end)) = time_range {
        let records = index.read_group_time_range(
            group,
            &[channel],
            Some(time),
            Some(start),
            Some(end),
            &mut reader,
        )?;
        println!("Time range [{start}, {end}]:");
        for (timestamp, value) in records.times.iter().zip(&records.values[0]) {
            println!("  {timestamp}: {value:?}");
        }
    }
    Ok(())
}
