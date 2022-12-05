use std::io::BufRead;

fn is_subrange(subrange: &Vec<u64>, range: &Vec<u64>) -> bool {
    subrange[0] >= range[0] && subrange[1] <= range[1]
}

fn main() {
    let mut full_overlap_count = 0;

    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let ranges: Vec<Vec<u64>> = line.split(",")
            .map(|raw_range| raw_range.split("-").map(|x| x.parse().unwrap()).collect())
            .collect();

        if is_subrange(&ranges[0], &ranges[1]) || is_subrange(&ranges[1], &ranges[0]) {
            full_overlap_count += 1;
        }

    }

    println!("{}", full_overlap_count);
}
