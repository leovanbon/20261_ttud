use std::io::{self, Read, Write};

fn query(tree: &[i32], n: usize, mut l: usize, mut r: usize) -> i32 {
    let mut answer = i32::MAX;
    l += n;
    r += n;

    while l < r {
        if l & 1 == 1 {
            answer = answer.min(tree[l]);
            l += 1;
        }
        if r & 1 == 1 {
            r -= 1;
            answer = answer.min(tree[r]);
        }
        l /= 2;
        r /= 2;
    }
    answer
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let mut out = io::BufWriter::new(io::stdout().lock());

    let n: usize = it.next().unwrap().parse().unwrap();
    let mut tree = vec![i32::MAX; 2 * n];

    for i in 0..n {
        tree[n + i] = it.next().unwrap().parse().unwrap();
    }

    for i in (1..n).rev() {
        tree[i] = tree[2 * i].min(tree[2 * i + 1]);
    }
    let answer = query(&tree, n, 2.min(n), 8.min(n)).min(1_000_000_000);
    writeln!(out, "{answer}").unwrap();
}
