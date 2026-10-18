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
    let mut tree = vec![-1_000_000_000i32; 2 * n];
    for i in 0..n {
        tree[n + i] = next!();
    }
    for i in (1..n).rev() {
        tree[i] = tree[2 * i].max(tree[2 * i + 1]);
    }
    let m: usize = next!();
    for _ in 0..m {
        let cmd = it.next().unwrap();
        let x: usize = next!();
        let y: i32 = next!();
        if cmd == "update" {
            let mut p = n + x - 1;
            tree[p] = y;
            while p > 1 {
                p /= 2;
                tree[p] = tree[p * 2].max(tree[p * 2 + 1]);
            }
        } else if cmd == "get-max" {
            let (mut l, mut r) = (n + x - 1, n + y as usize);
            let mut answer = -1_000_000_000;
            while l < r {
                if l & 1 == 1 {
                    answer = answer.max(tree[l]);
                    l += 1;
                }
                if r & 1 == 1 {
                    r -= 1;
                    answer = answer.max(tree[r]);
                }
                l /= 2;
                r /= 2;
            }
            writeln!(out, "{answer}").unwrap();
        }
    }
}
