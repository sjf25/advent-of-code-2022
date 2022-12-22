use std::cmp::Ordering;
use std::io::{self, BufRead};
use std::iter::Peekable;

enum Packet {
    PacketInt(u64),
    PacketList(Vec<Packet>)
}

fn parse_packet<It>(input: &mut Peekable<It>) -> Packet
where
    It: Iterator<Item = char>
{
    match input.next().unwrap() {
        '[' => {
            let mut packet_list = Vec::new();
            if *input.peek().unwrap() != ']' {
                loop {
                    packet_list.push(parse_packet(input));
                    let next_sym = input.next().unwrap();
                    assert!(next_sym == ',' || next_sym == ']');
                    if next_sym == ']' {
                        break;
                    }
                }
            }
            else {
                input.next();
            }
            Packet::PacketList(packet_list)
        },
        first @ '0'..='9' => {
            let mut num_raw = String::from(first);
            while let Some(digit) = input.next_if(|c| ('0'..='9').contains(c)) {
                num_raw.push(digit);
            }
            Packet::PacketInt(num_raw.parse().unwrap())
        },
        sym @ _ => panic!("unexpected symbol: {}", sym)
    }
}

fn packet_ordering(p1: &Packet, p2: &Packet) -> Ordering {
    if let (Packet::PacketInt(n1), Packet::PacketInt(n2)) = (p1, p2) {
        return n1.cmp(n2);
    }
    else if let (Packet::PacketList(l1), Packet::PacketList(l2)) = (p1, p2) {
        for (e1, e2) in l1.iter().zip(l2.iter()) {
            let cmp = packet_ordering(e1, e2);
            if cmp != Ordering::Equal {
                return cmp;
            }
        }
        return l1.len().cmp(&l2.len());
    }
    else if let Packet::PacketInt(n1) = p1 {
        packet_ordering(&Packet::PacketList(Vec::from([Packet::PacketInt(*n1)])), p2)
    }
    else if let Packet::PacketInt(n2) = p2 {
        packet_ordering(p1, &Packet::PacketList(Vec::from([Packet::PacketInt(*n2)])))
    }
    else {
        panic!("unreachable");
    }
}

fn main() {
    let lines: Vec<_> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let pairs: Vec<_> = lines
        .chunks(3)
        .map(|chunk| (parse_packet(&mut chunk[0].chars().peekable()), parse_packet(&mut chunk[1].chars().peekable())))
        .collect();

    let mut idx_sum = 0;
    for (idx, (p1, p2)) in pairs.iter().enumerate() {
        if packet_ordering(p1, p2) == Ordering::Less {
            idx_sum += idx + 1;
        }
    }

    println!("{idx_sum}");
}
