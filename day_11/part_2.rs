use std::io::{self, BufRead};
use std::cell::RefCell;

#[derive(Default)]
enum Operation {
    Add(u64),
    Multiply(u64),
    #[default]
    Square
}

#[derive(Default)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    divisible_test: u64,
    true_monkey: usize,
    false_monkey: usize,
    inspected_count: u64
}

fn last_word_as_num<T: std::str::FromStr>(x: &str) -> T 
where <T as std::str::FromStr>::Err: std::fmt::Debug
{
    x.split_whitespace().last().unwrap().parse().unwrap()
}

fn parse_monkey(monkey_input: &[String]) -> Monkey {
    assert!(monkey_input.len() == 6);
    let mut monkey = Monkey::default();

    for line in monkey_input.iter().skip(1) {
        let pieces: Vec<_> = line.splitn(2, ": ").collect();
        let field = pieces[0];
        let val = pieces[1];
        match field.trim() {
            "Starting items" => monkey.items = val.split(", ")
                .map(|x| x.parse().unwrap())
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect(),
            "Test" => monkey.divisible_test = last_word_as_num(val),
            "Operation" => monkey.operation = {
                if val.starts_with("new = old * old") {
                    Operation::Square
                }
                else if val.starts_with("new = old *") {
                    Operation::Multiply(last_word_as_num(val))
                }
                else {
                    Operation::Add(last_word_as_num(val))
                }
            },
            "If true" => monkey.true_monkey = last_word_as_num(val),
            "If false" => monkey.false_monkey = last_word_as_num(val),
            _ => panic!("unexpected row in monkey input")
        }
    }

    monkey
}

fn main() {
    let lines: Vec<_> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let monkeys: Vec<_> = lines.split(|line| line == "")
        .map(|monkey_input| RefCell::new(parse_monkey(&monkey_input)))
        .collect();

    let divisor_product: u64 = monkeys.iter().map(|m| m.borrow().divisible_test).product();

    for _iter in 0..10000 {
        for monkey in &monkeys {
            while monkey.borrow().items.len() > 0 {
                monkey.borrow_mut().inspected_count += 1;
                let mut item = monkey.borrow_mut().items.pop().unwrap();

                match monkey.borrow().operation {
                    Operation::Add(x) => item += x,
                    Operation::Multiply(x) => item *= x,
                    Operation::Square => item *= item
                }
                item %= divisor_product;

                let monkey_idx = {
                    let monkey = monkey.borrow();
                    if item % monkey.divisible_test == 0 {
                        monkey.true_monkey
                    }
                    else {
                        monkey.false_monkey
                    }
                };
                monkeys[monkey_idx].borrow_mut().items.push(item);
            }
        }
    }

    let mut counts: Vec<_> = monkeys.iter().map(|m| m.borrow().inspected_count).collect();
    counts.sort_by(|a, b| b.cmp(a));

    println!("{}", counts[0] * counts[1]);
}
