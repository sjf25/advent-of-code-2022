use std::io::BufRead;

fn overlap(range1: &Vec<u64>, range2: &Vec<u64>) -> bool {
    range1[0] <= range2[1] && range2[0] <= range1[1]
}

fn main() {
    let mut full_overlap_count = 0;

    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let ranges: Vec<Vec<u64>> = line.split(",")
            .map(|raw_range| raw_range.split("-").map(|x| x.parse().unwrap()).collect())
            .collect();

        if overlap(&ranges[0], &ranges[1]) {
            full_overlap_count += 1;
        }

    }

    println!("{}", full_overlap_count);
}
