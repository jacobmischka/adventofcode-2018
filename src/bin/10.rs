use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader, Write},
    str::FromStr,
};

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    let path = env::args()
        .nth(1)
        .expect("must pass path to input as first argument");

    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut points: Vec<Point> = reader
        .lines()
        .map(|line| Point::from_str(&line.unwrap()).unwrap())
        .collect();

    let mut i = 0;
    let mut d: isize = 0;
    let mut s = String::new();

    let mut min_x: isize;
    let mut max_x: isize;
    let mut min_y: isize;
    let mut max_y: isize;

    loop {
        min_x = points
            .iter()
            .fold(isize::MAX, |acc, point| acc.min(point.x))
            - 1;
        max_x = points
            .iter()
            .fold(isize::MIN, |acc, point| acc.max(point.x))
            + 1;
        min_y = points
            .iter()
            .fold(isize::MAX, |acc, point| acc.min(point.y))
            - 1;
        max_y = points
            .iter()
            .fold(isize::MIN, |acc, point| acc.max(point.y))
            + 1;

        if (max_x - min_x).abs() < 1000 && (max_y - min_y).abs() < 1000 {
            break;
        }

        i += 100;

        for point in &mut points {
            point.move_self(100);
        }
    }

    loop {
        println!("{}", i);
        draw(points.as_slice(), (min_x, min_y), (max_x, max_y));

        print!("\n\nd: ");
        io::stdout().flush().unwrap();
        s.clear();
        io::stdin().read_line(&mut s).unwrap();
        if !s.trim().is_empty() {
            d = s.trim().parse().unwrap();
        } else {
            println!("{}", d);
        }

        for point in &mut points {
            point.move_self(d);
        }

        min_x = points
            .iter()
            .fold(isize::MAX, |acc, point| acc.min(point.x))
            - 1;
        max_x = points
            .iter()
            .fold(isize::MIN, |acc, point| acc.max(point.x))
            + 1;
        min_y = points
            .iter()
            .fold(isize::MAX, |acc, point| acc.min(point.y))
            - 1;
        max_y = points
            .iter()
            .fold(isize::MIN, |acc, point| acc.max(point.y))
            + 1;

        i += d;
    }
}

fn draw(points: &[Point], (min_x, min_y): (isize, isize), (max_x, max_y): (isize, isize)) {
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            print!(
                "{}",
                if points.iter().any(|point| point.x == x && point.y == y) {
                    '#'
                } else {
                    '.'
                }
            );
        }
        println!();
    }
}

#[derive(Debug, Clone)]
struct Point {
    x: isize,
    y: isize,

    dx: isize,
    dy: isize,
}

impl Point {
    fn move_self(&mut self, multiplier: isize) {
        self.x += self.dx * multiplier;
        self.y += self.dy * multiplier;
    }
}

impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref POINT_RE: Regex = Regex::new(
                r"position=< *(?P<x>.+), *(?P<y>.+)> velocity=< *(?P<dx>.+), *(?P<dy>.+)>"
            )
            .unwrap();
        }

        let caps = POINT_RE
            .captures(s)
            .ok_or_else(|| format!("invalid point: {}", s))?;

        let x: isize = caps
            .name("x")
            .ok_or_else(|| format!("no x: {}", s))?
            .as_str()
            .parse()
            .map_err(|e| format!("invalid x: {} {:?}", s, e))?;
        let y: isize = caps
            .name("y")
            .ok_or_else(|| format!("no y: {}", s))?
            .as_str()
            .parse()
            .map_err(|e| format!("invalid y: {} {:?}", s, e))?;
        let dx: isize = caps
            .name("dx")
            .ok_or_else(|| format!("no dx: {}", s))?
            .as_str()
            .parse()
            .map_err(|e| format!("invalid dx: {} {:?}", s, e))?;
        let dy: isize = caps
            .name("dy")
            .ok_or_else(|| format!("no dy: {}", s))?
            .as_str()
            .parse()
            .map_err(|e| format!("invalid dy: {} {:?}", s, e))?;

        Ok(Point { x, y, dx, dy })
    }
}
