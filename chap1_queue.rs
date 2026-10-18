use std::collections::VecDeque;
use std::io::{self, Read, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    macro_rules! next {
        ($t:ty) => {
            it.next().unwrap().parse::<$t>().unwrap()
        };
        () => {
            it.next().unwrap().parse().unwrap()
        };
    }
    let mut out = io::BufWriter::new(io::stdout().lock());
    let mut queue: VecDeque<i32> = VecDeque::new();
    while let Some(cmd) = it.next() {
        match cmd {
            "#" => break,
            "PUSH" => queue.push_back(next!()),
            "POP" => match queue.pop_front() {
                Some(value) => writeln!(out, "{value}").unwrap(),
                None => writeln!(out, "NULL").unwrap(),
            },
            _ => {}
        }
    }
}
