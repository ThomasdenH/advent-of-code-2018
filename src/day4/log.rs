use crate::day4::date::Date;
use failure::{Error, Fail};
use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct LogEntry {
    pub date: Date,
    pub log_type: LogEntryType,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum LogEntryType {
    BeginShift { guard: u32 },
    FallsAsleep,
    WakesUp,
}

#[derive(Fail, Debug, Eq, PartialEq)]
#[fail(display = "Unknown log format")]
pub struct LogEntryErr;

lazy_static! {
    static ref REGEX_BEGIN_SHIFT: Regex = Regex::new(
        r"(?x)\[
        (?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})
        \ 
        (?P<hour>\d{2}):(?P<minute>\d{2})
        \]\ 
        Guard\ \#(?P<guard>\d+)\ begins\ shift"
    )
    .unwrap();
}

lazy_static! {
    static ref REGEX_FALLS_ASLEEP: Regex = Regex::new(
        r"(?x)\[
        (?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})
        \ 
        (?P<hour>\d{2}):(?P<minute>\d{2})
        \]\ 
        falls\ asleep$"
    )
    .unwrap();
}

lazy_static! {
    static ref REGEX_WAKES_UP: Regex = Regex::new(
        r"(?x)\[
        (?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})
        \ 
        (?P<hour>\d{2}):(?P<minute>\d{2})
        \]\ 
        wakes\ up"
    )
    .unwrap();
}

impl FromStr for LogEntry {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(res) = REGEX_BEGIN_SHIFT.captures(s) {
            Ok(LogEntry {
                date: Date {
                    year: res["year"].parse()?,
                    month: res["month"].parse()?,
                    day: res["day"].parse()?,
                    hour: res["hour"].parse()?,
                    minute: res["minute"].parse()?,
                },
                log_type: LogEntryType::BeginShift {
                    guard: res["guard"].parse()?,
                },
            })
        } else if let Some(res) = REGEX_FALLS_ASLEEP.captures(s) {
            Ok(LogEntry {
                date: Date {
                    year: res["year"].parse()?,
                    month: res["month"].parse()?,
                    day: res["day"].parse()?,
                    hour: res["hour"].parse()?,
                    minute: res["minute"].parse()?,
                },
                log_type: LogEntryType::FallsAsleep,
            })
        } else if let Some(res) = REGEX_WAKES_UP.captures(s) {
            Ok(LogEntry {
                date: Date {
                    year: res["year"].parse()?,
                    month: res["month"].parse()?,
                    day: res["day"].parse()?,
                    hour: res["hour"].parse()?,
                    minute: res["minute"].parse()?,
                },
                log_type: LogEntryType::WakesUp,
            })
        } else {
            Err(LogEntryErr.into())
        }
    }
}

#[test]
fn test_log_parse() {
    assert_eq!(
        "[1518-11-01 00:00] Guard #10 begins shift"
            .parse::<LogEntry>()
            .unwrap(),
        LogEntry {
            date: Date {
                year: 1518,
                month: 11,
                day: 1,
                hour: 0,
                minute: 0
            },
            log_type: LogEntryType::BeginShift { guard: 10 }
        }
    );
    assert_eq!(
        "[1518-09-26 23:59] Guard #2851 begins shift"
            .parse::<LogEntry>()
            .unwrap(),
        LogEntry {
            date: Date {
                year: 1518,
                month: 9,
                day: 26,
                hour: 23,
                minute: 59
            },
            log_type: LogEntryType::BeginShift { guard: 2851 }
        }
    );
    assert_eq!(
        "[1518-11-01 00:05] falls asleep"
            .parse::<LogEntry>()
            .unwrap(),
        LogEntry {
            date: Date {
                year: 1518,
                month: 11,
                day: 1,
                hour: 0,
                minute: 5
            },
            log_type: LogEntryType::FallsAsleep
        }
    );
    assert_eq!(
        "[1518-11-01 00:25] wakes up".parse::<LogEntry>().unwrap(),
        LogEntry {
            date: Date {
                year: 1518,
                month: 11,
                day: 1,
                hour: 0,
                minute: 25
            },
            log_type: LogEntryType::WakesUp
        }
    );
}
