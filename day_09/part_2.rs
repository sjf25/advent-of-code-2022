use std::collections::HashSet;
use std::io::{BufRead, self};

fn move_head(pos: &mut (i64, i64), dir: &str) {
    match dir {
        "R" => pos.0 += 1,
        "L" => pos.0 -= 1,
        "U" => pos.1 += 1,
        "D" => pos.1 -= 1,
        _ => panic!("unexpected direction")
    }
}

fn move_nonhead(head: &(i64, i64), tail: &mut (i64, i64)) {
    let dx = head.0 - tail.0;
    let dy = head.1 - tail.1;

    if dx.abs() <= 1 && dy.abs() <= 1 {
        return;
    }

    tail.0 += (dx > 0) as i64 - (dx < 0) as i64;
    tail.1 += (dy > 0) as i64 - (dy < 0) as i64;
}

fn main() {
    let mut knots = [(0, 0); 10];
    let mut tail_history = HashSet::from([(0, 0)]);

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let tokens: Vec<_> = line.split_whitespace().collect();
        let dir = tokens[0];
        let step_count = tokens[1].parse::<i64>().unwrap();

        for _i in 0..step_count {
            move_head(&mut knots[0], dir);
            for i in 1..knots.len() {
                let (left, right) = knots.split_at_mut(i);
                move_nonhead(left.last().unwrap(), &mut right[0]);

                assert!((knots[i-1].0 - knots[i].0).abs() <= 1 &&
                        (knots[i-1].1 - knots[i].1).abs() <= 1);
            }

            tail_history.insert(*knots.last().unwrap());
        }
    }

    println!("{}", tail_history.len());
}
