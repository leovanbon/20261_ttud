use std::collections::HashSet;
use std::io::{self, Read, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let mut out = io::BufWriter::new(io::stdout().lock());

    let n: usize = it.next().unwrap().parse().unwrap();
    let mut seen = HashSet::new();

    for _ in 0..n {
        let value: i32 = it.next().unwrap().parse().unwrap();
        writeln!(out, "{}", u8::from(!seen.insert(value))).unwrap();
    }
}
