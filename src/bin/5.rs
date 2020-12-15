use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();

    println!("Part 1: {}", react_polymer(input.trim()).len());

    let mut shortest = usize::MAX;
    for c in 'a'..='z' {
        shortest = shortest.min(
            react_polymer(
                &input
                    .trim()
                    .replace(c, "")
                    .replace(c.to_ascii_uppercase(), ""),
            )
            .len(),
        )
    }

    println!("Part 2: {}", shortest);
}

fn react_polymer(s: &str) -> Vec<char> {
    let mut units: Vec<char> = s.chars().collect();
    loop {
        let mut pair_begins = None;
        for (i, pair) in units.windows(2).enumerate() {
            if pair[0].eq_ignore_ascii_case(&pair[1])
                && pair[0].is_uppercase() != pair[1].is_uppercase()
            {
                pair_begins = Some(i);
                break;
            }
        }

        if let Some(i) = pair_begins {
            units.remove(i);
            units.remove(i);
        } else {
            break;
        }
    }

    units
}
