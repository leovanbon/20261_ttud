use std::io::{self, Read, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let mut out = io::BufWriter::new(io::stdout().lock());

    let n: usize = it.next().unwrap().parse().unwrap();
    let mut a: Vec<i32> = (0..n).map(|_| it.next().unwrap().parse().unwrap()).collect();
    a.sort_unstable();

    while let Some(cmd) = it.next() {
        if cmd == "#" {
            break;
        }
        let k: i32 = it.next().unwrap().parse().unwrap();
        let index = a.partition_point(|&value| value <= k);
        writeln!(out, "{}", a.get(index).copied().unwrap_or(-1)).unwrap();
    }
}
