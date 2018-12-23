use itertools::Itertools;

use std::collections::HashMap;

use helpers::get_input;

fn main() {
    part1();
    part2();
}

fn part1() {
    let mut counts = HashMap::new();
    let mut twos = 0;
    let mut threes = 0;
    for line in get_input().split_whitespace() {
        for c in line.chars() {
            counts.insert(
                c,
                match counts.get(&c) {
                    Some(count) => count + 1,
                    None => 1,
                },
            );
        }

        let mut has_two = false;
        let mut has_three = false;

        for (_, count) in counts.drain() {
            if count == 2 {
                has_two = true;
            } else if count == 3 {
                has_three = true;
            }
        }

        if has_two {
            twos += 1;
        }
        if has_three {
            threes += 1;
        }
    }

    println!("Checksum: {}", twos * threes);
}

fn part2() {
    for pair in get_input().split_whitespace().combinations(2) {
        let len = pair[0].len();
        let mut samesies = Vec::new();
        for (c1, c2) in pair[0].chars().zip(pair[1].chars()) {
            if c1 == c2 {
                samesies.push(c1);
            }
        }

        if samesies.len() == len - 1 {
            let common: String = samesies.into_iter().collect();
            println!("Common: {}", common);
            return;
        }
    }
}
