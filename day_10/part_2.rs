use std::io::{self, BufRead};

fn pixel_out(cycle_count: i64, register_x: i64) -> char {
    let row = cycle_count % 40;
    if register_x == row || register_x + 1 == row || register_x - 1 == row {
        '#'
    }
    else {
        '.'
    }
}

fn main() {
    let mut register_x = 1;
    let mut cycle_count = 0;

    let mut crt_out = Vec::new();

    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let tokens: Vec<_> = line.split_whitespace().collect();
        let instr = tokens[0];
        match instr {
            "addx" => {
                crt_out.push(pixel_out(cycle_count, register_x));
                crt_out.push(pixel_out(cycle_count + 1, register_x));
                cycle_count += 2;
                register_x += tokens[1].parse::<i64>().unwrap();
            },
            "noop" => {
                crt_out.push(pixel_out(cycle_count, register_x));
                cycle_count += 1;
            },
            _ => panic!("unexpected instruction")
        }
    }

    assert!(crt_out.len() == 40 * 6);

    for (idx, pixel) in crt_out.iter().enumerate() {
        if idx % 40 == 0 && idx != 0 {
            println!();
        }
        print!("{pixel}");
    }
    println!();
}
