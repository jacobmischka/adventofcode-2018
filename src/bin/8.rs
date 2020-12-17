use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let input: Vec<u8> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let (tree, _) = Node::new(&input.as_slice());

    println!("Part 1: {}", tree.total_metadata_sum());
    println!("Part 2: {}", tree.value());
}

#[derive(Debug, Clone)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<u8>,
}

impl Node {
    fn new(mut buf: &[u8]) -> (Self, &[u8]) {
        let num_children = buf[0];
        let num_metadata = buf[1];

        let mut children = Vec::new();
        buf = &buf[2..];
        for _ in 0..num_children {
            let (child, remaining) = Node::new(&buf);
            children.push(child);
            buf = remaining;
        }

        let metadata = buf[..num_metadata as usize].iter().copied().collect();

        (Node { children, metadata }, &buf[num_metadata as usize..])
    }

    fn total_metadata_sum(&self) -> u32 {
        self.children.iter().fold(
            self.metadata.iter().copied().sum::<u8>() as u32,
            |acc, c| acc + c.total_metadata_sum(),
        )
    }

    fn value(&self) -> u32 {
        if self.children.is_empty() {
            self.total_metadata_sum()
        } else {
            self.metadata
                .iter()
                .fold(0, |acc, i| match self.children.get(*i as usize - 1) {
                    None => acc,
                    Some(child) => acc + child.value(),
                })
        }
    }
}
