use std::io::Read;

const N: usize = 14;

fn no_repeats(window: &[u8; N]) -> bool {
    for i in 0..N {
        for j in i+1..N {
            if window[i] == window[j] {
                return false;
            }
        }
    }
    true
}

fn main() {
    let mut window = [0; N];
    let mut position = 0;
    for i in std::io::stdin().bytes() {
        window[position % N] = i.unwrap();
        if position >= N-1 && no_repeats(&window) {
            println!("{}", position + 1);
            return;
        }
        position += 1;
    }
}
