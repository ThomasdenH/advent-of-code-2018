use crate::day4::log::{LogEntry, LogEntryType};
use failure::Error;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solution() -> Result<u32, Error> {
    let file = File::open("./src/day4/input.txt")?;
    let reader = BufReader::new(file);
    let mut entries = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let line = line.trim();
            line.parse::<LogEntry>().unwrap()
        })
        .collect::<Vec<_>>();

    entries.sort_by(|a, b| a.date.cmp(&b.date));

    let mut guard_minute_count: HashMap<(u32, u8), u32> = HashMap::new();

    let mut date_start = None;
    let mut current_guard = None;
    for entry in entries.iter() {
        if let LogEntryType::BeginShift { guard } = entry.log_type {
            current_guard = Some(guard);
        } else if entry.log_type == LogEntryType::FallsAsleep {
            date_start = Some(entry.date);
        } else {
            let guard = current_guard.expect("No current guard");
            let minute_start = date_start.expect("No starting time").minute;
            let minute_end = entry.date.minute;
            for minute in minute_start..minute_end {
                let key = (guard, minute);
                guard_minute_count.insert(key, guard_minute_count.get(&key).unwrap_or(&0) + 1);
            }
            date_start = None;
        }
    }

    let (guard, minute) = guard_minute_count
        .iter()
        .max_by_key(|(_, count)| *count)
        .unwrap()
        .0;
    Ok(guard * u32::from(*minute))
}

#[test]
fn test_solution() {
    assert_eq!(solution().unwrap(), 4727);
}
