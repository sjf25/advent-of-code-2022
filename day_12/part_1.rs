use std::collections::VecDeque;
use std::io::{self, BufRead};

fn main() {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let grid: Vec<Vec<_>> = io::stdin()
        .lock()
        .lines()
        .enumerate()
        .map(|(row, line)| line.unwrap()
             .chars()
             .enumerate()
             .map(|(col, c)| match c {
                 'a'..='z' => c as u32 - 'a' as u32,
                 'S' => {start = (row, col); 0},
                 'E' => {end = (row, col); (b'z' - b'a') as u32},
                 _ => panic!("unexpected elevation level")
             })
             .collect())
        .collect();

    let mut dist = vec![vec![-1; grid[0].len()]; grid.len()];
    dist[start.0][start.1] = 0;

    let mut queue = VecDeque::from([start]);
    'outer_loop: while queue.len() > 0 {
        let (curr_row, curr_col) = queue.pop_front().unwrap();
        let deltas: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        for (dr, dc) in &deltas {
            let cand_row = curr_row as isize + dr;
            let cand_col = curr_col as isize + dc;

            if cand_row < 0 || cand_row >= grid.len() as isize || cand_col < 0 ||
                cand_col >= grid[0].len() as isize ||
                dist[cand_row as usize][cand_col as usize] != -1 ||
                grid[cand_row as usize][cand_col as usize] > grid[curr_row][curr_col] + 1 {
                continue;
            }

            let cand_row = cand_row as usize;
            let cand_col = cand_col as usize;

            dist[cand_row][cand_col] = dist[curr_row][curr_col] + 1;
            if (cand_row, cand_col) == end {
                break 'outer_loop;
            }
            queue.push_back((cand_row, cand_col));
        }
    }

    println!("{}", dist[end.0][end.1]);
}
