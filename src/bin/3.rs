//! This is not efficient.

use std::{
    collections::{HashMap, HashSet},
    io::{self, BufRead},
};

struct Claim {
    id: u32,
    left: u32,
    top: u32,
    width: u32,
    height: u32,
}

impl Claim {
    fn from_claim(claim: &str) -> Option<Self> {
        let mut pieces = claim
            .split(|c| [' ', '#', '@', ',', ':', 'x'].contains(&c))
            .filter(|s| s.len() > 0)
            .map(|s| {
                s.parse::<u32>()
                    .expect(&format!("Failed parsing to int: {}", s))
            });

        if let (Some(id), Some(left), Some(top), Some(width), Some(height)) = (
            pieces.next(),
            pieces.next(),
            pieces.next(),
            pieces.next(),
            pieces.next(),
        ) {
            Some(Claim {
                id,
                left,
                top,
                width,
                height,
            })
        } else {
            None
        }
    }

    fn left(&self) -> u32 {
        self.left
    }

    fn top(&self) -> u32 {
        self.top
    }

    fn right(&self) -> u32 {
        self.left + self.width
    }

    fn bottom(&self) -> u32 {
        self.top + self.height
    }
}

fn main() {
    let mut grid: HashMap<(u32, u32), Vec<u32>> = HashMap::new();
    let mut intact = HashSet::new();
    for line in io::stdin().lock().lines().filter_map(Result::ok) {
        if let Some(claim) = Claim::from_claim(&line) {
            intact.insert(claim.id);

            for x in claim.left()..claim.right() {
                for y in claim.top()..claim.bottom() {
                    let ids = grid.entry((x, y)).or_insert(Vec::new());
                    ids.push(claim.id);
                    if ids.len() > 1 {
                        for id in ids {
                            intact.remove(id);
                        }
                    }
                }
            }
        }
    }

    let mut num_overlaps = 0;

    for ids in grid.values() {
        if ids.len() > 1 {
            num_overlaps += 1;
        }
    }

    println!("Overlaps: {}", num_overlaps);
    println!(
        "Intact: {}",
        intact.iter().next().expect("No claims intact")
    );
}
