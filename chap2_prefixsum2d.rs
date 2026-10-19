use std::io::{self, Read, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let mut out = io::BufWriter::new(io::stdout().lock());

    let m: usize = it.next().unwrap().parse().unwrap();
    let n: usize = it.next().unwrap().parse().unwrap();
    let mut prefix = vec![vec![0i64; n + 1]; m + 1];

    for i in 1..=m {
        for j in 1..=n {
            prefix[i][j] = prefix[i - 1][j] + prefix[i][j - 1] - prefix[i - 1][j - 1]
                + it.next().unwrap().parse::<i64>().unwrap();
        }
    }
    let q: usize = it.next().unwrap().parse().unwrap();

    for _ in 0..q {
        let x1: usize = it.next().unwrap().parse().unwrap();
        let y1: usize = it.next().unwrap().parse().unwrap();
        let x2: usize = it.next().unwrap().parse().unwrap();
        let y2: usize = it.next().unwrap().parse().unwrap();
        writeln!(
            out,
            "{}",
            prefix[x2][y2] - prefix[x1 - 1][y2] - prefix[x2][y1 - 1] + prefix[x1 - 1][y1 - 1]
        )
        .unwrap();
    }
}
