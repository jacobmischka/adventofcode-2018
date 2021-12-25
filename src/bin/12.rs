use adventofcode_2018::get_input;

use std::{collections::HashMap, iter};

const OFFSET: usize = 1000;
const REPLACEMENT_SIZE: usize = 5;

fn main() {
    let s = get_input().unwrap();
    let mut lines = s.lines();

    let start = lines.next().unwrap().split_whitespace().nth(2).unwrap();

    let mut state: String = iter::repeat('.')
        .take(OFFSET)
        .chain(&mut start.chars())
        .chain(iter::repeat('.').take(OFFSET))
        .collect();

    let replacements: HashMap<&str, &str> = lines
        .skip(1)
        .map(|line| {
            let mut pieces = line.split_whitespace();
            (pieces.next().unwrap(), pieces.nth(1).unwrap())
        })
        .collect();

    let mut seen = HashMap::new();

    let mut gen: usize = 0;
    let mut gen_remaining: usize = 50_000_000_000;
    let mut pot_offset: isize = 0;

    while gen_remaining > 0 {
        gen += 1;
        gen_remaining -= 1;
        let mut next: String = iter::repeat('.').take(state.len()).collect();
        'outer: for i in 0..(state.len() - REPLACEMENT_SIZE) {
            for (s, c) in &replacements {
                if &state[i..(i + REPLACEMENT_SIZE)] == *s {
                    next.replace_range(i + 2..=i + 2, c);
                    continue 'outer;
                }
            }
        }
        state = next;

        // println!("{}: {}", gen, &state);

        if gen == 20 {
            println!(
                "Part 1: {}",
                state
                    .char_indices()
                    .fold(0isize, |acc, (i, c)| if c == '#' {
                        acc + i as isize - OFFSET as isize
                    } else {
                        acc
                    })
            );
        }

        let mut on = state
            .char_indices()
            .filter_map(|(i, c)| if c == '#' { Some(i) } else { None });
        let i = on.next().unwrap();
        let j = on.last().unwrap();

        let significant = state[i..=j].to_owned();

        if let Some((prev_gen_rem, prev_start)) = seen.get(&significant) {
            let start_step = i - prev_start;
            let step = prev_gen_rem - gen_remaining;

            let num_steps = gen_remaining / step;
            let remainder = gen_remaining % step;

            pot_offset = (start_step * num_steps) as isize;
            gen_remaining = remainder;
        }

        seen.insert(significant, (gen_remaining, i));
    }

    println!(
        "Part 2: {}",
        state
            .char_indices()
            .fold(0isize, |acc, (i, c)| if c == '#' {
                acc + i as isize - OFFSET as isize + pot_offset
            } else {
                acc
            })
    );
}
