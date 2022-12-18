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

fn move_tail(head: &(i64, i64), tail: &mut (i64, i64)) {
    let dx = head.0 - tail.0;
    let dy = head.1 - tail.1;

    if dx.abs() <= 1 && dy.abs() <= 1 {
        return;
    }

    tail.0 += (dx > 0) as i64 - (dx < 0) as i64;
    tail.1 += (dy > 0) as i64 - (dy < 0) as i64;
}

fn main() {
    let mut head_pos = (0, 0);
    let mut tail_pos = (0, 0);
    let mut tail_history = HashSet::from([tail_pos]);

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let tokens: Vec<_> = line.split_whitespace().collect();
        let dir = tokens[0];
        let step_count = tokens[1].parse::<i64>().unwrap();

        for _i in 0..step_count {
            move_head(&mut head_pos, dir);
            move_tail(&head_pos, &mut tail_pos);

            assert!((head_pos.0 - tail_pos.0).abs() <= 1 &&
                    (head_pos.1 - tail_pos.1).abs() <= 1);

            tail_history.insert(tail_pos);
        }
    }

    println!("{}", tail_history.len());
}
