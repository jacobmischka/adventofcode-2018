use std::{
    collections::{HashMap, HashSet},
    io::{self, BufRead},
};

use regex::Regex;

const NUM_WORKERS: usize = 5;
const BASE_STEP_DURATION: usize = 60;

fn main() {
    let input_re =
        Regex::new(r"Step (?P<from>[A-Z]) must be finished before step (?P<to>[A-Z]) can begin.")
            .unwrap();

    let mut all_steps: HashSet<char> = HashSet::new();
    let mut from_map: HashMap<char, Vec<char>> = HashMap::new();
    let mut to_map: HashMap<char, Vec<char>> = HashMap::new();

    for line in io::stdin().lock().lines().filter_map(Result::ok) {
        let caps = input_re.captures(&line).unwrap();
        let from = caps.name("from").unwrap().as_str().chars().next().unwrap();
        let to = caps.name("to").unwrap().as_str().chars().next().unwrap();

        all_steps.insert(from);
        all_steps.insert(to);
        from_map.entry(from).or_default().push(to);
        to_map.entry(to).or_default().push(from);
    }

    let all_steps = all_steps;
    let to_map = to_map;

    let mut completed: Vec<char> = Vec::new();

    while completed.len() < all_steps.len() {
        let mut available: Vec<char> = all_steps
            .iter()
            .filter(|step| {
                !completed.contains(step)
                    && (match to_map.get(step) {
                        Some(required) => required.iter().all(|r| completed.contains(r)),
                        None => true,
                    })
            })
            .copied()
            .collect();

        available.sort();
        completed.push(available[0]);
    }

    println!(
        "Part 1: {}",
        completed.iter().fold(String::new(), |mut s, c| {
            s.push(*c);
            s
        })
    );

    let mut completed: Vec<char> = Vec::new();
    let mut in_progress: HashMap<char, usize> = HashMap::new();

    let mut t = 0;
    while completed.len() < all_steps.len() {
        in_progress.retain(|step, complete_time| {
            if *complete_time == t {
                completed.push(*step);
                false
            } else {
                true
            }
        });

        let free_workers = NUM_WORKERS - in_progress.len();

        if free_workers > 0 {
            let mut available: Vec<char> = all_steps
                .iter()
                .filter(|step| {
                    !completed.contains(step)
                        && !in_progress.contains_key(step)
                        && (match to_map.get(step) {
                            Some(required) => required.iter().all(|r| completed.contains(r)),
                            None => true,
                        })
                })
                .copied()
                .collect();

            available.sort();

            for next_step in available.into_iter().take(free_workers) {
                in_progress.insert(
                    next_step,
                    t + BASE_STEP_DURATION + (next_step as u32 - 'A' as u32 + 1) as usize,
                );
            }
        }

        t += 1;
    }

    println!("Part 2: {}", t - 1);
}
