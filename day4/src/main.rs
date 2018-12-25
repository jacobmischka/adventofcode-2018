use chrono::{Duration, NaiveDate, NaiveDateTime};
use lazy_static::lazy_static;
use regex::Regex;

use std::collections::HashMap;

use helpers::get_input;

#[derive(Debug)]
struct Shift {
    began: NaiveDateTime,
    sleeps: Vec<Sleep>,
}

#[derive(Debug)]
struct Sleep(NaiveDateTime, NaiveDateTime);

#[derive(Debug)]
struct Guard {
    id: u32,
    shifts: Vec<Shift>,
}

fn part1(guards: &HashMap<u32, Guard>) {
    let mut longest_sleep = Duration::zero();
    let mut guard_id: Option<u32> = None;

    for (id, guard) in guards.iter() {
        let mut time_asleep = Duration::zero();
        for shift in guard.shifts.iter() {
            for sleep in shift.sleeps.iter() {
                time_asleep = time_asleep + (sleep.1 - sleep.0);
            }
        }

        if time_asleep > longest_sleep {
            longest_sleep = time_asleep;
            guard_id = Some(*id);
        }
    }

    if let Some(guard_id) = guard_id {
        if let Some(guard) = guards.get(&guard_id) {
            if let Some((max_minute, _)) = get_sleep_minute(guard) {
                println!("Strategy 1: {}", max_minute * guard_id);
            }
        }
    }
}

fn get_sleep_minute(guard: &Guard) -> Option<(u32, u32)> {
    let mut minutes: HashMap<u32, u32> = HashMap::new();
    let mut max: Option<(u32, u32)> = None;

    for shift in guard.shifts.iter() {
        for sleep in shift.sleeps.iter() {
            let mut t = sleep.0.clone();
            while t < sleep.1 {
                let minute: u32 = t
                    .format("%M")
                    .to_string()
                    .parse()
                    .expect("Failed parsing minutes into int");
                let count = minutes.entry(minute).or_insert(0);
                *count += 1;

                if let Some((_max_min, max_sleeps)) = max {
                    if *count > max_sleeps {
                        max = Some((minute, *count));
                    }
                } else {
                    max = Some((minute, *count));
                }

                t = t + Duration::minutes(1);
            }
        }
    }

    max
}

fn part2(guards: &HashMap<u32, Guard>) {
    let mut combo: Option<(u32, u32, u32)> = None;
    for (id, ref guard) in guards.iter() {
        if let Some((minute, sleeps)) = get_sleep_minute(&guard) {
            if let Some((_max_id, _max_minute, max_sleeps)) = combo {
                if sleeps > max_sleeps {
                    combo = Some((*id, minute, sleeps));
                }
            } else {
                combo = Some((*id, minute, sleeps));
            }
        }
    }

    if let Some((id, minute, _sleeps)) = combo {
        println!("Strategy 2: {}", id * minute);
    }
}

fn main() {
    lazy_static! {
        static ref GUARD_REGEX: Regex = Regex::new(r"\#[\d]+").unwrap();
    }

    let mut messages = Vec::new();
    for line in get_input().split("\n") {
        if line.len() == 0 {
            continue;
        }

        let timestamp = NaiveDateTime::parse_from_str(&line[..18], "[%Y-%m-%d %H:%M]")
            .expect(&format!("Failed parsing datetime: {}", &line[..18]));
        messages.push((timestamp, line[19..].to_string()))
    }

    messages.sort_by(|a, b| a.0.cmp(&b.0));

    let mut guards: HashMap<u32, Guard> = HashMap::new();
    let mut guard_id: Option<u32> = None;
    let mut fell_asleep: Option<NaiveDateTime> = None;

    for message in messages {
        if message.1.starts_with("Guard") {
            let m = GUARD_REGEX
                .find(&message.1)
                .expect("Couldn't find guard id")
                .as_str();
            let id: u32 = m[1..].parse().expect(&format!("Failed parsing id: {}", m));
            guard_id = Some(id);

            let ref mut guard = guards.entry(id).or_insert(Guard {
                id,
                shifts: Vec::new(),
            });

            guard.shifts.push(Shift {
                began: message.0.clone(),
                sleeps: Vec::new(),
            });
        } else if let Some(id) = guard_id {
            let ref mut guard = guards
                .get_mut(&id)
                .expect(&format!("no guard found for id {}", id));

            if message.1 == "wakes up" {
                if let Some(fell_asleep) = fell_asleep {
                    let i = guard.shifts.len() - 1;
                    let shift = &mut guard.shifts[i];

                    shift.sleeps.push(Sleep(fell_asleep, message.0))
                }
            } else if message.1 == "falls asleep" {
                fell_asleep = Some(message.0)
            }
        }
    }

    part1(&guards);
    part2(&guards);
}
