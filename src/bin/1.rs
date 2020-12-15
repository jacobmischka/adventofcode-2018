use std::{
    collections::HashSet,
    io::{self, BufRead},
};

fn main() {
    part1();
    part2();
}

fn part1() {
    let mut freq: i32 = 0;
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let mag: i32 = line[1..].parse().unwrap();
        if &line[..1] == "-" {
            freq -= mag
        } else {
            freq += mag
        }
    }

    println!("Frequency is {}", freq);
}

fn part2() {
    let mut freq: i32 = 0;
    let mut freqs = HashSet::new();

    let input: Vec<String> = io::stdin().lock().lines().filter_map(Result::ok).collect();

    loop {
        for line in input.iter() {
            let mag: i32 = line[1..].parse().unwrap();
            if &line[..1] == "-" {
                freq -= mag
            } else {
                freq += mag
            }

            if !freqs.insert(freq) {
                println!("First frequency reached twice is {}", freq);
                return;
            }
        }
    }
}
