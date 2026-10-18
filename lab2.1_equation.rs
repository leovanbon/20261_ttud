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
    let target: i64 = next!();
    let a: Vec<usize> = (0..n).map(|_| next!()).collect();
    let remaining = target - a.iter().map(|&x| x as i64).sum::<i64>();
    if remaining < 0 {
        write!(out, "0").unwrap();
        return;
    }
    let remaining = remaining as usize;
    let mut dp = vec![0i64; remaining + 1];
    dp[0] = 1;
    for value in a {
        for sum in value..=remaining {
            dp[sum] += dp[sum - value];
        }
    }
    write!(out, "{}", dp[remaining]).unwrap();
}
