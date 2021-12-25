use adventofcode_2018::get_input;

fn main() {
    let s = get_input().unwrap();
    let serial_num: i64 = s.parse().unwrap();

    let mut grid = [[0i64; 301]; 301];
    for x in 0..=300 {
        for y in 0..=300 {
            grid[x][y] = power_level(serial_num, (x, y));
        }
    }

    let (max_coords, _) = find_max(&grid, 3);

    println!("Part 1: {},{}", max_coords.0, max_coords.1);

    let mut max_power = i64::MIN;
    let mut max_coords = (0, 0);
    let mut max_size = 0;
    for size in 1..=50 {
        let (coords, power) = find_max(&grid, size);
        if power > max_power {
            max_power = power;
            max_coords = coords;
            max_size = size;
        }
    }

    println!("Part 2: {},{},{}", max_coords.0, max_coords.1, max_size);
}

fn find_max(grid: &[[i64; 301]; 301], square_size: usize) -> ((usize, usize), i64) {
    let mut max_coords = (0, 0);
    let mut max_power = i64::MIN;

    for x in 0..=(300 - square_size) {
        for y in 0..=(300 - square_size) {
            let mut power = 0;
            for i in 0..square_size {
                for j in 0..square_size {
                    power += grid[x + i][y + j];
                }
            }

            if power > max_power {
                max_power = power;
                max_coords = (x, y);
            }
        }
    }

    (max_coords, max_power)
}

fn power_level(serial_num: i64, (x, y): (usize, usize)) -> i64 {
    let rack_id = x as i64 + 10;
    let mut power = rack_id * y as i64 + serial_num;
    power *= rack_id;

    let sign = power.signum();
    (power / 100) % 10 * sign - 5
}
