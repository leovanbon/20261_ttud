use std::io::{self, Read, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let mut out = io::BufWriter::new(io::stdout().lock());

    let n: usize = it.next().unwrap().parse().unwrap();
    let target: i64 = it.next().unwrap().parse().unwrap();
    let mut a: Vec<i64> = (0..n).map(|_| it.next().unwrap().parse().unwrap()).collect();
    a.sort_unstable();
    let (mut left, mut right, mut answer) = (0, n, 0);

    while left + 1 < right {
        let sum = a[left] + a[right - 1];
        if sum == target {
            answer += 1;
            left += 1;
            right -= 1;
        } else if sum < target {
            left += 1;
        } else {
            right -= 1;
        }
    }

    write!(out, "{answer}").unwrap();
}
