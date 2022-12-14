use std::io::BufRead;

macro_rules! visible_check {
    ($it1:ident in $range1:expr, $it2:ident in $range2:expr, $start_max: expr, $grid_val:expr, $can_see:expr) => {
        {
            for $it1 in $range1 {
                let mut curr_max = $start_max;
                for $it2 in $range2 {
                    if $grid_val > curr_max {
                        $can_see = true;
                        curr_max = $grid_val;
                    }
                }
            }
        }
    };
}

fn main() {
    let stdin = std::io::stdin();
    let grid: Vec<Vec<_>> = stdin.lock()
        .lines()
        .map(|line| line.unwrap().chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut can_see = vec![vec![false; grid[0].len()]; grid.len()];

    for i in 0..grid.len() {
        can_see[i][0] = true;
        can_see[i][grid[0].len()-1] = true;
    }
    for i in 0..grid[0].len() {
        can_see[0][i] = true;
        can_see[grid.len()-1][i] = true;
    }

    visible_check!(r in 1..grid.len(), c in 1..grid[0].len(), grid[r][0], grid[r][c], can_see[r][c]);
    visible_check!(c in 1..grid[0].len(), r in 1..grid.len(), grid[0][c], grid[r][c], can_see[r][c]);
    visible_check!(r in 1..grid.len(), c in (1..grid[0].len()).rev(), grid[r][grid[0].len()-1], grid[r][c], can_see[r][c]);
    visible_check!(c in 1..grid[0].len(), r in (1..grid.len()).rev(), grid[grid.len()-1][c], grid[r][c], can_see[r][c]);


    let mut count = 0;
    for row in &can_see {
        for is_visible in row {
            if *is_visible {
                count += 1;
            }
        }
    }

    println!("{count}");

}
