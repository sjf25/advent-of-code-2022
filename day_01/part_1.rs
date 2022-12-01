use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let mut max_total = std::u64::MIN;
    let mut acc = 0;
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        match line.as_ref() {
            "" => {
                max_total = max_total.max(acc);
                acc = 0;
            },
            _ => acc += line.parse::<u64>().unwrap()
        }
    }
    max_total = max_total.max(acc);

    println!("{}", max_total);
}
