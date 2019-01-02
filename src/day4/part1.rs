use crate::day4::log::{LogEntry, LogEntryType};
use failure::{Error};
use std::fs::File;
use std::collections::HashMap;
use std::io::{BufReader, BufRead};

pub fn solution() -> Result<u32, Error> {
    let file = File::open("./src/day4/input.txt")?;
    let reader = BufReader::new(file);
    let mut entries = reader.lines()
        .map(|line| {
            let line = line.unwrap();
            let line = line.trim();
            println!("{}", line);
            line.parse::<LogEntry>().unwrap()
        })
        .collect::<Vec<_>>();

    entries.sort_by(|a, b| a.date.cmp(&b.date));

    let mut date_start = None;
    let mut current_guard = None;

    let mut minutes_asleep: HashMap<u32, u32> = HashMap::new();

    for entry in entries.iter() {
        if let LogEntryType::BeginShift{ guard } = entry.log_type {
            current_guard = Some(guard);
        } else if entry.log_type == LogEntryType::FallsAsleep {
            date_start = Some(entry.date);
        } else {
            let curr_minutes_asleep = u32::from(entry.date.minutes_diff(&date_start.unwrap()));
            let guard = current_guard.expect("No current guard");
            minutes_asleep.insert(guard, minutes_asleep.get(&guard).unwrap_or(&0) + curr_minutes_asleep);
            date_start = None;
        }
    }

    // Guard with most minutes
    let guard_most_minutes_asleep = entries.iter()
        .filter_map(|entry| match entry.log_type {
            LogEntryType::BeginShift{ guard } => Some(guard),
            _ => None
        })
        .map(|guard| (guard, minutes_asleep.get(&guard).unwrap_or(&0)))
        .max_by_key(|(_, minutes_asleep)| *minutes_asleep)
        .expect("No guard found")
        .0;

    let mut times_asleep_per_minute: HashMap<u8, u32> = HashMap::new();
    for entry in entries.iter() {
        if let LogEntryType::BeginShift{ guard } = entry.log_type {
            current_guard = Some(guard);
        } else if entry.log_type == LogEntryType::FallsAsleep {
            date_start = Some(entry.date);
        } else {
            let guard = current_guard.expect("No current guard");
            if guard == guard_most_minutes_asleep {
                for minute in date_start.expect("No starting date").minute..entry.date.minute {
                    times_asleep_per_minute.insert(minute, times_asleep_per_minute.get(&minute).unwrap_or(&0) + 1);
                }
            }
            date_start = None;
        }
    }

    let mut max_minute = 0u8;
    for minute in 1u8..60 {
        if *times_asleep_per_minute.get(&minute).unwrap_or(&0) > *times_asleep_per_minute.get(&max_minute).unwrap_or(&0) {
            max_minute = minute;
        }
    }

    println!("{}", u32::from(max_minute) * guard_most_minutes_asleep);

    Ok(u32::from(max_minute) * guard_most_minutes_asleep)
}

#[test]
fn test_solution() {
    assert_eq!(solution().unwrap(), 0);
}
