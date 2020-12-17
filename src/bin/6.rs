use std::{
    collections::{HashMap, HashSet},
    io::{self, BufRead},
};

const SAFE_REGION_DISTANCE: u32 = 10000;

fn main() {
    let points: Vec<Point> = io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .map(|s| {
            let mut iter = s.split(", ");
            Point(
                iter.next().unwrap().parse().unwrap(),
                iter.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    let min_x = points.iter().map(|Point(x, _)| *x).min().unwrap();
    let max_x = points.iter().map(|Point(x, _)| *x).max().unwrap();
    let min_y = points.iter().map(|Point(_, y)| *y).min().unwrap();
    let max_y = points.iter().map(|Point(_, y)| *y).max().unwrap();

    let mut inner_points: HashSet<&Point> = points.iter().collect();
    let mut closest_map: HashMap<&Point, Vec<Point>> = HashMap::new();

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            let p = Point(x, y);

            let (_, closest_points): (_, Vec<&Point>) = points.iter().fold(
                (u32::MAX, Vec::new()),
                |(distance, mut closest_points), point| {
                    let d = p.distance(point);
                    if d == distance {
                        closest_points.push(point);
                        (d, closest_points)
                    } else if d < distance {
                        (d, vec![point])
                    } else {
                        (distance, closest_points)
                    }
                },
            );

            if closest_points.len() == 1 {
                if x == min_x || x == max_x || y == min_y || y == max_y {
                    inner_points.retain(|point| !closest_points.contains(point));
                } else {
                    closest_map
                        .entry(closest_points.first().unwrap())
                        .or_default()
                        .push(p);
                }
            }
        }
    }

    let part_1 = closest_map
        .iter()
        .filter(|(point, _)| inner_points.contains(*point))
        .map(|(_, closest)| closest.len())
        .max()
        .unwrap();

    println!("Part 1: {}", part_1);

    let mid_x = (max_x + min_x) / 2;
    let mid_y = (max_y + min_y) / 2;

    let mut safe_locations = HashSet::new();

    let mut x = mid_x;
    loop {
        let mut y = mid_y;
        let p = Point(x, y);
        let total_distance: u32 = points.iter().map(|point| p.distance(point)).sum();
        if total_distance >= SAFE_REGION_DISTANCE {
            break;
        }

        loop {
            let p = Point(x, y);
            let total_distance: u32 = points.iter().map(|point| p.distance(point)).sum();
            if total_distance >= SAFE_REGION_DISTANCE {
                break;
            }
            safe_locations.insert(p);
            y += 1;
        }

        y = mid_y - 1;
        loop {
            let p = Point(x, y);
            let total_distance: u32 = points.iter().map(|point| p.distance(point)).sum();
            if total_distance >= SAFE_REGION_DISTANCE {
                break;
            }
            safe_locations.insert(p);
            y -= 1;
        }

        x += 1;
    }

    let mut x = mid_x - 1;
    loop {
        let mut y = mid_y - 1;
        let p = Point(x, y);
        let total_distance: u32 = points.iter().map(|point| p.distance(point)).sum();
        if total_distance >= SAFE_REGION_DISTANCE {
            break;
        }

        loop {
            let p = Point(x, y);
            let total_distance: u32 = points.iter().map(|point| p.distance(point)).sum();
            if total_distance >= SAFE_REGION_DISTANCE {
                break;
            }
            safe_locations.insert(p);
            y += 1;
        }

        y = mid_y - 1;
        loop {
            let p = Point(x, y);
            let total_distance: u32 = points.iter().map(|point| p.distance(point)).sum();
            if total_distance >= SAFE_REGION_DISTANCE {
                break;
            }
            safe_locations.insert(p);
            y -= 1;
        }
        x -= 1;
    }

    println!("Part 2: {}", safe_locations.len());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point(u32, u32);

impl Point {
    fn distance(&self, other: &Point) -> u32 {
        self.0.max(other.0) - self.0.min(other.0) + self.1.max(other.1) - self.1.min(other.1)
    }
}
