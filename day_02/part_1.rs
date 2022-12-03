use std::io::BufRead;

fn outcome_score(opponent: u8, response: u8) -> u64 {
    if (response + 2) % 3 == opponent {
        return 6;
    }
    else if opponent == response {
        return 3;
    }
    0
}

fn main() {
    let mut total_score = 0;

    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let line = line.as_bytes();
        let opponent = line[0] - 'A' as u8;
        let response = line[2] - 'X' as u8;

        total_score += (response as u64 + 1) + outcome_score(opponent, response);
    }

    println!("{}", total_score);
}
