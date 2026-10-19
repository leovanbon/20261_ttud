use std::io::{self, Read, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let mut out = io::BufWriter::new(io::stdout().lock());

    let n: usize = it.next().unwrap().parse().unwrap();
    let mut prefix = vec![0i64; n + 1];

    for i in 1..=n {
        prefix[i] = prefix[i - 1] + it.next().unwrap().parse::<i64>().unwrap();
    }
    let q: usize = it.next().unwrap().parse().unwrap();

    for _ in 0..q {
        let l: usize = it.next().unwrap().parse().unwrap();
        let r: usize = it.next().unwrap().parse().unwrap();
        writeln!(out, "{}", prefix[r] - prefix[l - 1]).unwrap();
    }
}
