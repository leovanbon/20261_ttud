use std::io::{self, Read, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let mut out = io::BufWriter::new(io::stdout().lock());

    let mut stack: Vec<i32> = Vec::new();

    while let Some(cmd) = it.next() {
        match cmd {
            "#" => break,
            "PUSH" => stack.push(it.next().unwrap().parse().unwrap()),
            "POP" => match stack.pop() {
                Some(value) => writeln!(out, "{value}").unwrap(),
                None => writeln!(out, "NULL").unwrap(),
            },
            _ => {}
        }
    }
}
