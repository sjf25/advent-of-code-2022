use std::io::{self, BufRead};
use std::collections::{HashSet};

fn grid_contains(grid: &HashSet<(i32, i32)>, max_y: i32, coord: &(i32, i32)) -> bool {
    coord.1 == max_y + 2 || grid.contains(coord)
}

fn sand_drop(grid: &mut HashSet<(i32, i32)>, max_y: i32) -> u64 {
    let mut sand_count = 0;
    while !grid.contains(&(500, 0)) {
        let mut sand = (500, 0);
        loop {
            let down = (sand.0, sand.1 + 1);
            let left = (sand.0 - 1, sand.1 + 1);
            let right = (sand.0 + 1, sand.1 + 1);
            if !grid_contains(&grid, max_y, &down) {
                sand = down;
            }
            else if !grid_contains(&grid, max_y, &left) {
                sand = left;
            }
            else if !grid_contains(&grid, max_y, &right) {
                sand = right;
            }
            else {
                grid.insert(sand);
                sand_count += 1;
                break;
            }
        }
    }
    sand_count
}

fn main() {
    let paths: Vec<_> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap()
             .split(" -> ")
             .map(|coord_in| {
                 let coords: Vec<_> = coord_in.split(',')
                     .map(|i| i.parse::<i32>().unwrap())
                     .collect();
                 (coords[0], coords[1])
             })
             .collect::<Vec<_>>())
        .collect();

    let mut grid = HashSet::new();

    for p in &paths {
        let mut prev = p[0];
        for curr in p.iter().skip(1) {
            let (prev_x, prev_y) = prev;
            let (curr_x, curr_y) = curr;

            for dx in 0..=prev_x-curr_x {
                grid.insert((prev_x - dx, prev_y));
            }
            for dx in 0..=curr_x-prev_x {
                grid.insert((prev_x + dx, prev_y));
            }
            for dy in 0..=prev_y-curr_y {
                grid.insert((prev_x, prev_y - dy));
            }
            for dy in 0..=curr_y-prev_y {
                grid.insert((prev_x, prev_y + dy));
            }

            prev = *curr;
        }
    }

    let mut max_y = i32::MIN;
    for rock in &grid {
        max_y = max_y.max(rock.1);
    }

    let count = sand_drop(&mut grid, max_y);

    println!("{count}");
}
