use std::io::{self, Read, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let mut out = io::BufWriter::new(io::stdout().lock());

    let n: usize = it.next().unwrap().parse().unwrap();
    let mut a: Vec<usize> = (1..=n).collect();

    loop {
        for value in &a {
            write!(out, "{value} ").unwrap();
        }
        writeln!(out).unwrap();
        let Some(i) = (0..n.saturating_sub(1)).rev().find(|&i| a[i] < a[i + 1]) else {
            break;
        };
        let j = (i + 1..n).rev().find(|&j| a[j] > a[i]).unwrap();
        a.swap(i, j);
        a[i + 1..].reverse();
    }
}
