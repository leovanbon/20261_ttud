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
    let n: usize = next!();
    let mut prefix = vec![0i64; n + 1];
    for i in 1..=n {
        prefix[i] = prefix[i - 1] + next!(i64);
    }
    let q: usize = next!();
    for _ in 0..q {
        let l: usize = next!();
        let r: usize = next!();
        writeln!(out, "{}", prefix[r] - prefix[l - 1]).unwrap();
    }
}
