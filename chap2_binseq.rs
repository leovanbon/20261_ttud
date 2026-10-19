use std::io::{self, Read, Write};

fn count(n: usize, pattern: &[u8], current: &mut Vec<u8>) -> i64 {
    if current.len() == n {
        return 1;
    }
    let mut answer = 0;

    for bit in [b'0', b'1'] {
        current.push(bit);
        if !current.ends_with(pattern) {
            answer += count(n, pattern, current);
        }
        current.pop();
    }
    answer
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let mut out = io::BufWriter::new(io::stdout().lock());

    let n: usize = it.next().unwrap().parse().unwrap();
    let pattern = it.next().unwrap().parse::<String>().unwrap().into_bytes();
    writeln!(out, "{}", count(n, &pattern, &mut Vec::new())).unwrap();
}
