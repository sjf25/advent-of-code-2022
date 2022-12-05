use std::io::BufRead;

fn main() {
    let mut priority_sum = 0;

    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let bytes = line.as_bytes();

        let mut seen = [false; 256];
        for i in bytes[..bytes.len()/2].iter() {
            seen[*i as usize] = true;
        }

        for i in bytes[bytes.len()/2..].iter() {
            if seen[*i as usize] {
                priority_sum += if *i > b'Z' { 1 + (*i - b'a') as u64 } else { 27 + (*i - b'A') as u64};
                break;
            }
        }
    }

    println!("{}", priority_sum);
}
