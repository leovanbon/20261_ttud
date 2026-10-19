use std::io::{self, Read, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let mut out = io::BufWriter::new(io::stdout().lock());

    let n: usize = it.next().unwrap().parse().unwrap();
    let q: i64 = it.next().unwrap().parse().unwrap();
    let a: Vec<i64> = (0..n).map(|_| it.next().unwrap().parse().unwrap()).collect();
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
