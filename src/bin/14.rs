use adventofcode_2018::get_input;

fn main() {
    let recipes: usize = get_input().unwrap().trim().parse().unwrap();
    let input_digits = digits(recipes);

    let mut scores = vec![3, 7];
    let mut elf_1 = 0;
    let mut elf_2 = 1;

    'outer: loop {
        let sum = scores[elf_1] + scores[elf_2];
        let mut new_scores = digits(sum);
        let num_new_scores = new_scores.len();
        scores.append(&mut new_scores);
        elf_1 = (elf_1 + scores[elf_1] + 1) % scores.len();
        elf_2 = (elf_2 + scores[elf_2] + 1) % scores.len();
        if scores.len() == recipes + 10 {
            print!("Part 1: ");
            for i in 0..10 {
                print!("{}", scores[recipes + i]);
            }
            println!();
        }

        for j in 0..num_new_scores {
            if (scores.len() - j) >= input_digits.len() {
                let mut matches = true;
                for i in 1..=input_digits.len() {
                    if scores[scores.len() - i - j] != input_digits[input_digits.len() - i] {
                        matches = false;
                    }
                }

                if matches {
                    println!("Part 2: {}", scores.len() - input_digits.len() - j);
                    break 'outer;
                }
            }
        }
    }
}

fn digits(mut val: usize) -> Vec<usize> {
    if val == 0 {
        return vec![0];
    }

    let mut v = Vec::new();
    while val > 0 {
        v.push(val % 10);
        val /= 10;
    }

    v.reverse();

    v
}

#[allow(unused)]
fn dump_recipes(scores: &[usize], elf_1: usize, elf_2: usize) {
    for (i, score) in scores.iter().enumerate() {
        eprint!(
            "{}{}{}",
            if i == elf_1 {
                '('
            } else if i == elf_2 {
                '['
            } else {
                ' '
            },
            score,
            if i == elf_1 {
                ')'
            } else if i == elf_2 {
                ']'
            } else {
                ' '
            }
        );
    }

    eprintln!()
}
