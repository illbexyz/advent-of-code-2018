extern crate regex;

use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;
use std::io::Error;

#[derive(Debug, PartialEq, Eq)]
enum Hour {
    BeforeMidnight,
    AfterMidnight,
}

#[derive(Debug, PartialEq, Eq)]
enum LogEvent {
    FallAsleep,
    BeginShift(i32),
    WakeUp,
}

#[derive(Debug, Eq)]
pub struct LogEntry {
    date: String,
    hour: Hour,
    minutes: i32,
    event: LogEvent,
}

impl Ord for LogEntry {
    fn cmp(&self, other: &LogEntry) -> Ordering {
        match self.date.cmp(&other.date) {
            Ordering::Equal => match self.hour {
                Hour::BeforeMidnight => Ordering::Greater,
                Hour::AfterMidnight => self.minutes.cmp(&other.minutes),
            },
            other => other,
        }
    }
}

impl PartialOrd for LogEntry {
    fn partial_cmp(&self, other: &LogEntry) -> Option<Ordering> {
        Some(match self.date.cmp(&other.date) {
            Ordering::Equal => match self.hour {
                Hour::BeforeMidnight => Ordering::Greater,
                Hour::AfterMidnight => self.minutes.cmp(&other.minutes),
            },
            other => other,
        })
    }
}

impl PartialEq for LogEntry {
    fn eq(&self, other: &LogEntry) -> bool {
        self.date.eq(&other.date)
    }
}

fn read_file(filepath: &str) -> Result<String, Error> {
    fs::read_to_string(filepath)
}

fn parse_logs(log: String) -> Result<Vec<LogEntry>, Error> {
    let re = Regex::new(r"(?m)^\[(\w+-\w+-\w+)\s(\d\d):(\d\d)\]\s(\w+)\s#?(\d+)?").unwrap();
    let mut logs: Vec<LogEntry> = re
        .captures_iter(&log)
        .map(|cap| LogEntry {
            date: String::from(&cap[1]),
            hour: if cap[2].eq("00") {
                Hour::AfterMidnight
            } else {
                Hour::BeforeMidnight
            },
            minutes: cap[3].parse::<i32>().unwrap(),
            event: match cap[4].as_ref() {
                "wakes" => LogEvent::WakeUp,
                "falls" => LogEvent::FallAsleep,
                "Guard" => LogEvent::BeginShift(cap[5].parse::<i32>().unwrap()),
                _ => panic!("Invalid input"),
            },
        })
        .collect();
    logs.sort();
    Ok(logs)
}

pub fn part_one(logs_with_id: &Vec<(i32, &LogEntry)>) -> i32 {
    let mut id_to_mins_asleep = HashMap::new();

    let mut current_start_sleep = 0;
    for (id, l) in logs_with_id.iter() {
        match l.event {
            LogEvent::FallAsleep => current_start_sleep = l.minutes,
            LogEvent::WakeUp => {
                let already_slept_time: i32 = match id_to_mins_asleep.get(&id) {
                    Some(sleep_time) => *sleep_time,
                    None => 0,
                };
                id_to_mins_asleep.insert(id, already_slept_time + l.minutes - current_start_sleep);
            }
            _ => (),
        }
    }

    let (top_sleeper_id, _) = id_to_mins_asleep
        .into_iter()
        .max_by_key(|(_, v)| *v)
        .unwrap();

    let top_sleeper_logs: Vec<_> = logs_with_id
        .iter()
        .filter(|(id, _)| (*id).eq(top_sleeper_id))
        .collect();

    let mut minute_to_sleeptime = HashMap::new();
    for (_, log) in top_sleeper_logs {
        match log.event {
            LogEvent::FallAsleep => current_start_sleep = log.minutes,
            LogEvent::WakeUp => {
                for i in current_start_sleep..log.minutes {
                    let val = match minute_to_sleeptime.get(&i) {
                        Some(value) => value + 1,
                        None => 1,
                    };
                    minute_to_sleeptime.insert(i, val);
                }
            }
            _ => (),
        }
    }

    let most_slept_minute = minute_to_sleeptime
        .iter()
        .max_by_key(|(_minute, frequency)| *frequency);

    let most_slept_minutes = most_slept_minute.unwrap().0;

    top_sleeper_id * most_slept_minutes
}

pub fn part_two(logs_with_id: &Vec<(i32, &LogEntry)>) -> i32 {

    let mut ids: Vec<i32> = logs_with_id
        .iter()
        .map(|(id, _)| *id)
        .collect();

    ids.sort();
    ids.dedup();

    let id_to_most_slept_minute: Vec<(i32, i32, i32)> = ids
        .iter()
        .map(|id| {
            let logs_of_guard: Vec<_> = logs_with_id
                .iter()
                .filter(|(curr_id, _)| (*curr_id).eq(id))
                .collect();

            let mut minute_to_sleeptime = HashMap::new();
            let mut current_start_sleep = 0;

            for (_, log) in logs_of_guard {
                match log.event {
                    LogEvent::FallAsleep => current_start_sleep = log.minutes,
                    LogEvent::WakeUp => {
                        for i in current_start_sleep..log.minutes {
                            let val = match minute_to_sleeptime.get(&i) {
                                Some(value) => value + 1,
                                None => 1,
                            };
                            minute_to_sleeptime.insert(i, val);
                        }
                    }
                    _ => (),
                }
            }

            let (most_slept_minute, sleep_time) = minute_to_sleeptime
                .iter()
                .max_by_key(|(_minute, frequency)| *frequency)
                .unwrap_or((&0, &0));

            (*id, *most_slept_minute, *sleep_time)
        })
        .collect();

    let (id, minute, _time) = id_to_most_slept_minute
        .iter()
        .max_by_key(|(_, _, time)| *time)
        .unwrap();

    id * minute
}

fn main() -> Result<(), std::io::Error> {
    let data = read_file("input.txt")?;
    let logs = parse_logs(data)?;

    let mut current_id = 0;
    let logs_with_id: Vec<(i32, &LogEntry)> = logs
        .iter()
        .map(|entry| match entry.event {
            LogEvent::BeginShift(id) => {
                current_id = id;
                (current_id, entry)
            }
            LogEvent::WakeUp => (current_id, entry),
            LogEvent::FallAsleep => (current_id, entry),
        })
        .collect();

    let (p1, p2) = (part_one(&logs_with_id), part_two(&logs_with_id));

    println!("Part one: {}", p1);
    println!("Part two: {}", p2);

    Ok(())
}