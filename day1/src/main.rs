use std::collections::HashSet;

use helpers::get_input;

fn main() {
    part1();
    part2();
}

fn part1() {
    let mut freq: i32 = 0;
    for line in get_input().split_whitespace() {
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

    let input = get_input();

    loop {
        for line in input.split_whitespace() {
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
