use std::collections::HashSet;
use std::io::{self, Read, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let mut out = io::BufWriter::new(io::stdout().lock());
    let mut strings = HashSet::new();
    while let Some(s) = it.next() {
        if s == "*" {
            break;
        }
        strings.insert(s);
    }
    while let Some(cmd) = it.next() {
        if cmd == "***" {
            break;
        }
        let s = it.next().unwrap();
        let answer = match cmd {
            "find" => strings.contains(s),
            "insert" => strings.insert(s),
            _ => continue,
        };
        writeln!(out, "{}", u8::from(answer)).unwrap();
    }
}
