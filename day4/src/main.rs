use std::collections::HashMap;
use std::str::FromStr;
use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
struct Timestamp {
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
enum Event {
    FallAsleep,
    WakeUp,
    BeginShift(usize)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
struct LogEntry {
    time: Timestamp,
    event: Event
}

impl FromStr for LogEntry {
    type Err = parselib::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static::lazy_static! {
            static ref DATE_RE: Regex = Regex::new(r"\[\d+-(\d\d)-(\d\d) (\d\d):(\d\d)\]").unwrap();
            static ref BEGIN_SHIFT_RE: Regex = Regex::new(r"Guard #(\d+)").unwrap();
        };

        let date_captures = DATE_RE.captures(s).ok_or(parselib::ParseError)?;

        let event = if let Some(capture) = BEGIN_SHIFT_RE.captures(s) {
            Event::BeginShift(capture.get(1).unwrap().as_str().parse().unwrap())
        } else if s.find("falls").is_some() {
            Event::FallAsleep
        } else {
            Event::WakeUp
        };

        return Ok(LogEntry {
            time: Timestamp {
                month: date_captures.get(1).unwrap().as_str().parse().unwrap(),
                day: date_captures.get(2).unwrap().as_str().parse().unwrap(),
                hour: date_captures.get(3).unwrap().as_str().parse().unwrap(),
                minute: date_captures.get(4).unwrap().as_str().parse().unwrap(),
            },
            event: event
        });
    }
}

fn part12(sorted_entries: &[LogEntry]) -> (usize, usize) {
    let mut guard_minutes_asleep = HashMap::new();
    let mut current_guard = 0usize;
    let mut last_event_minute = 0;
    let mut most_minutes = 0;
    let mut most_minutes_guard = 0;
    for entry in sorted_entries {
        match entry.event {
            Event::BeginShift(guard_id) => { current_guard = guard_id; }
            Event::WakeUp => {
                let (ref mut total_minutes, ref mut per_minute) = guard_minutes_asleep.entry(current_guard).or_insert((0usize, [0usize;60]));
                for minute in last_event_minute..entry.time.minute {
                    per_minute[minute as usize] += 1;
                }
                *total_minutes += (entry.time.minute - last_event_minute) as usize;
                if *total_minutes > most_minutes {
                    most_minutes = *total_minutes;
                    most_minutes_guard = current_guard;
                }
            }
            Event::FallAsleep => {}
        };
        last_event_minute = if entry.time.hour != 0 { 0 } else { entry.time.minute };
    }

    let (_, minutes) = guard_minutes_asleep[&most_minutes_guard];
    let (guard_most_asleep_minute, _) = minutes.iter().enumerate().max_by_key(|&(_, v)| v).unwrap();

    // Part 2
    let mut most_asleep_minute = 0;
    let mut most_asleep_count = 0;
    let mut most_asleep_minute_guard = 0;

    for (guard, (_, minutes)) in guard_minutes_asleep {
        for (i, &asleep_count) in minutes.iter().enumerate() {
            if asleep_count > most_asleep_count {
                most_asleep_count = asleep_count;
                most_asleep_minute = i;
                most_asleep_minute_guard = guard;
            }
        }
    }

    return (guard_most_asleep_minute * most_minutes_guard, most_asleep_minute * most_asleep_minute_guard);
}

fn main() {
    let mut input = parselib::parse_lines::<LogEntry, _>("input.txt").expect("Could not parse input");
    input.sort();
    let result = part12(input.as_slice());
    println!("part1: {}", result.0);
    println!("part2: {}", result.1);
}
