use crate::day4::log::{LogEntry, LogEntryType};
use aoc_runner_derive::*;
use std::collections::HashMap;

mod date;
mod log;

#[aoc_generator(day4)]
fn generator(input: &str) -> Vec<LogEntry> {
    input
        .lines()
        .map(|line| line.parse::<LogEntry>().unwrap())
        .collect()
}

#[aoc(day4, part1)]
fn part1(entries: &[LogEntry]) -> u32 {
    let mut entries = entries.to_vec();
    entries.sort_by(|a, b| a.date.cmp(&b.date));

    let mut date_start = None;
    let mut current_guard = None;

    let mut minutes_asleep: HashMap<u32, u32> = HashMap::new();

    for entry in entries.iter() {
        if let LogEntryType::BeginShift { guard } = entry.log_type {
            current_guard = Some(guard);
        } else if entry.log_type == LogEntryType::FallsAsleep {
            date_start = Some(entry.date);
        } else {
            let curr_minutes_asleep = u32::from(entry.date.minutes_diff(date_start.unwrap()));
            let guard = current_guard.expect("No current guard");
            minutes_asleep.insert(
                guard,
                minutes_asleep.get(&guard).unwrap_or(&0) + curr_minutes_asleep,
            );
            date_start = None;
        }
    }

    // Guard with most minutes
    let guard_most_minutes_asleep = entries
        .iter()
        .filter_map(|entry| match entry.log_type {
            LogEntryType::BeginShift { guard } => Some(guard),
            _ => None,
        })
        .map(|guard| (guard, minutes_asleep.get(&guard).unwrap_or(&0)))
        .max_by_key(|(_, minutes_asleep)| *minutes_asleep)
        .expect("No guard found")
        .0;

    let mut times_asleep_per_minute: HashMap<u8, u32> = HashMap::new();
    for entry in entries.iter() {
        if let LogEntryType::BeginShift { guard } = entry.log_type {
            current_guard = Some(guard);
        } else if entry.log_type == LogEntryType::FallsAsleep {
            date_start = Some(entry.date);
        } else {
            let guard = current_guard.expect("No current guard");
            if guard == guard_most_minutes_asleep {
                for minute in date_start.expect("No starting date").minute..entry.date.minute {
                    times_asleep_per_minute.insert(
                        minute,
                        times_asleep_per_minute.get(&minute).unwrap_or(&0) + 1,
                    );
                }
            }
            date_start = None;
        }
    }

    let mut max_minute = 0u8;
    for minute in 1u8..60 {
        if *times_asleep_per_minute.get(&minute).unwrap_or(&0)
            > *times_asleep_per_minute.get(&max_minute).unwrap_or(&0)
        {
            max_minute = minute;
        }
    }

    u32::from(max_minute) * guard_most_minutes_asleep
}

#[aoc(day4, part2)]
fn part2(entries: &[LogEntry]) -> u32 {
    let mut entries = entries.to_vec();
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
    guard * u32::from(*minute)
}

#[test]
fn test_part1_example() {
    let input_string = "[1518-11-01 00:00] Guard #10 begins shift\n\
                        [1518-11-01 00:05] falls asleep\n\
                        [1518-11-01 00:25] wakes up\n\
                        [1518-11-01 00:30] falls asleep\n\
                        [1518-11-01 00:55] wakes up\n\
                        [1518-11-01 23:58] Guard #99 begins shift\n\
                        [1518-11-02 00:40] falls asleep\n\
                        [1518-11-02 00:50] wakes up\n\
                        [1518-11-03 00:05] Guard #10 begins shift\n\
                        [1518-11-03 00:24] falls asleep\n\
                        [1518-11-03 00:29] wakes up\n\
                        [1518-11-04 00:02] Guard #99 begins shift\n\
                        [1518-11-04 00:36] falls asleep\n\
                        [1518-11-04 00:46] wakes up\n\
                        [1518-11-05 00:03] Guard #99 begins shift\n\
                        [1518-11-05 00:45] falls asleep\n\
                        [1518-11-05 00:55] wakes up\n";
    let input = generator(&input_string);
    let result = part1(&input);
    assert_eq!(result, 240);
}

#[test]
fn test_part1() {
    let input_string = crate::util::read_file_to_string("./input/2018/day4.txt");
    let input = generator(&input_string);
    let result = part1(&input);
    assert_eq!(result, 73646);
}
#[test]
fn test_part2_example() {
    let input_string = "[1518-11-01 00:00] Guard #10 begins shift\n\
                        [1518-11-01 00:05] falls asleep\n\
                        [1518-11-01 00:25] wakes up\n\
                        [1518-11-01 00:30] falls asleep\n\
                        [1518-11-01 00:55] wakes up\n\
                        [1518-11-01 23:58] Guard #99 begins shift\n\
                        [1518-11-02 00:40] falls asleep\n\
                        [1518-11-02 00:50] wakes up\n\
                        [1518-11-03 00:05] Guard #10 begins shift\n\
                        [1518-11-03 00:24] falls asleep\n\
                        [1518-11-03 00:29] wakes up\n\
                        [1518-11-04 00:02] Guard #99 begins shift\n\
                        [1518-11-04 00:36] falls asleep\n\
                        [1518-11-04 00:46] wakes up\n\
                        [1518-11-05 00:03] Guard #99 begins shift\n\
                        [1518-11-05 00:45] falls asleep\n\
                        [1518-11-05 00:55] wakes up\n";
    let input = generator(&input_string);
    let result = part2(&input);
    assert_eq!(result, 4455);
}

#[test]
fn test_part2() {
    let input_string = crate::util::read_file_to_string("./input/2018/day4.txt");
    let input = generator(&input_string);
    let result = part2(&input);
    assert_eq!(result, 4727);
}
