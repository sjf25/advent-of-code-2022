use std::io::BufRead;

fn tree_score(r: usize, c: usize, grid: &Vec<Vec<u32>>) -> i32 {
    let mut score = 1;

    // up
    let mut count = 0;
    for i in (0..r).rev() {
        count += 1;
        if grid[i][c] >= grid[r][c] {
            break;
        }
    }
    score *= count;

    // down
    let mut count = 0;
    for i in r+1..grid.len() {
        count += 1;
        if grid[i][c] >= grid[r][c] {
            break;
        }
    }
    score *= count;

    // left
    let mut count = 0;
    for i in (0..c).rev() {
        count += 1;
        if grid[r][i] >= grid[r][c] {
            break;
        }
    }
    score *= count;
    
    // right
    let mut count = 0;
    for i in c+1..grid[0].len() {
        count += 1;
        if grid[r][i] >= grid[r][c] {
            break;
        }
    }
    score *= count;

    return score;
}

fn main() {
    let stdin = std::io::stdin();
    let grid: Vec<Vec<_>> = stdin.lock()
        .lines()
        .map(|line| line.unwrap().chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut best_score = 0;
    for r in 1..grid.len()-1 {
        for c in 1..grid[0].len()-1 {
            best_score = best_score.max(tree_score(r, c, &grid));
        }
    }

    println!("{best_score}");
}
