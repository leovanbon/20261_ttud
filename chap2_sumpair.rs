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
    let mut a: Vec<i64> = (0..n).map(|_| next!()).collect();
    a.sort_unstable();
    let (mut left, mut right, mut answer) = (0, n, 0);
    // Use an exclusive right endpoint so crossing pointers stop safely.
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
