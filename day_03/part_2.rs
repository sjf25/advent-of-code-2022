use std::io::BufRead;

fn main() {
    let mut priority_sum = 0;

    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();
    while let Some(line1) = lines.next() {
        let line1 = line1.unwrap();
        let line2 = lines.next().unwrap().unwrap();
        let line3 = lines.next().unwrap().unwrap();
        
        let mut counts = [0; 256];

        for (line_num, line) in [line1, line2, line3].iter().enumerate() {
            for i in line.chars() {
                counts[i as usize] |= 1 << line_num;
            }
        }

        for (i, c) in counts.iter().enumerate() {
            let i = i as u8;
            if *c == 0b111 {
                priority_sum += if i > b'Z' { 1 + (i - b'a') as u64 } else { 27 + (i - b'A') as u64};
                break;
            }
        }
    }

    println!("{}", priority_sum);
}
