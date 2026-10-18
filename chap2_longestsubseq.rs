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
    let q: i64 = next!();
    let a: Vec<i64> = (0..n).map(|_| next!()).collect();
    let (mut sum, mut left, mut answer) = (0, 0, 0);
    for right in 0..n {
        sum += a[right];
        while left <= right && sum > q {
            sum -= a[left];
            left += 1;
        }
        answer = answer.max(right + 1 - left);
    }
    write!(out, "{answer}").unwrap();
}
