use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let mut init_stacking: Vec<_> = stdin.lock()
        .lines()
        .map(|x| x.unwrap())
        .take_while(|x| x != "")
        .collect();
    init_stacking.pop();

    let moves: Vec<_> = stdin.lock().lines().map(|x| x.unwrap()).collect();

    let num_stacks = (init_stacking[0].len() + 1) / 4;
    let mut stacks = vec![Vec::<char>::new(); num_stacks];
    for line in init_stacking.iter().rev() {
        for i in 0..num_stacks {
            let sym = line.as_bytes()[i*4 + 1] as char;
            if sym != ' ' {
                stacks[i].push(sym);
            }
        }
    }

    for m in moves {
        let tokens: Vec<_> = m.split_whitespace().collect();
        let quantity: usize = tokens[1].parse().unwrap();
        let from: usize = tokens[3].parse().unwrap();
        let to: usize = tokens[5].parse().unwrap();

        for _i in 0..quantity {
            let val = stacks[from-1].pop().unwrap();
            stacks[to-1].push(val);
        }
    }

    for s in stacks {
        print!("{}", s.last().unwrap());
    }
    println!();

}
