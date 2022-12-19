use std::io::{self, BufRead};

fn update_signal_strength_sum(cycle_count: i64,
                              register_x: i64,
                              signal_strength_sum: &mut i64,
                              next_relevant_cycle: &mut i64) {
    if cycle_count >= *next_relevant_cycle {
        *signal_strength_sum += *next_relevant_cycle * register_x;
        *next_relevant_cycle += 40;
    }
}

fn main() {
    let mut register_x = 1;
    let mut cycle_count = 0;

    let mut next_relevant_cycle = 20;
    let mut signal_strength_sum = 0;

    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let tokens: Vec<_> = line.split_whitespace().collect();
        let instr = tokens[0];
        match instr {
            "addx" => {
                cycle_count += 2;
                update_signal_strength_sum(cycle_count,
                                           register_x,
                                           &mut signal_strength_sum,
                                           &mut next_relevant_cycle);
                register_x += tokens[1].parse::<i64>().unwrap();
            },
            "noop" => {
                cycle_count += 1;
                update_signal_strength_sum(cycle_count,
                                           register_x,
                                           &mut signal_strength_sum,
                                           &mut next_relevant_cycle);
            },
            _ => panic!("unexpected instruction")
        }
    }

    println!("{signal_strength_sum}");
}
