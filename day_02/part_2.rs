use std::io::BufRead;

fn choice_score(opponent: u64, round_outcome: u64) -> u64 {
    (opponent + (round_outcome + 2) % 3) % 3 + 1
}

fn main() {
    let mut total_score = 0;

    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let line = line.as_bytes();
        let opponent = (line[0] - 'A' as u8) as u64;
        let round_outcome = (line[2] - 'X' as u8) as u64;

        total_score += 3*round_outcome + choice_score(opponent, round_outcome);
    }

    println!("{}", total_score);
}
