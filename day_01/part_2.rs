use std::io::BufRead;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    let stdin = std::io::stdin();
    let mut acc = 0;
    let mut heap: BinaryHeap<_> = vec![std::u64::MIN; 3].into_iter().map(Reverse).collect();

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        match line.as_ref() {
            "" => {
                heap.push(Reverse(acc));
                heap.pop();
                acc = 0;
            },
            _ => acc += line.parse::<u64>().unwrap()
        }
    }
    heap.push(Reverse(acc));
    heap.pop();

    let top_three_sum: u64 = heap.into_iter().map(|Reverse(n)| n).sum();
    println!("{}", top_three_sum);
}
