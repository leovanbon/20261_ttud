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
    let m: usize = next!();
    let n: usize = next!();
    let mut prefix = vec![vec![0i64; n + 1]; m + 1];
    for i in 1..=m {
        for j in 1..=n {
            prefix[i][j] = prefix[i - 1][j] + prefix[i][j - 1] - prefix[i - 1][j - 1] + next!(i64);
        }
    }
    let q: usize = next!();
    for _ in 0..q {
        let x1: usize = next!();
        let y1: usize = next!();
        let x2: usize = next!();
        let y2: usize = next!();
        writeln!(
            out,
            "{}",
            prefix[x2][y2] - prefix[x1 - 1][y2] - prefix[x2][y1 - 1] + prefix[x1 - 1][y1 - 1]
        )
        .unwrap();
    }
}
