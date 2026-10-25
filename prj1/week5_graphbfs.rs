
use std::collections::VecDeque;
use std::io::{self, Read};

macro_rules! read {
    ($iter:expr, $t:ty) => {
        $iter.next().unwrap().parse::<$t>().unwrap()
    };
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut iter = input.split_whitespace();

    let n = read!(iter, usize);
    let m = read!(iter, usize);

    let mut adj = vec![Vec::<usize>::new(); n + 1];
    let mut visited = vec![false; n + 1];

    for _ in 0..m {
        let u = read!(iter, usize);
        let v = read!(iter, usize);

        adj[u].push(v);
        adj[v].push(u);
    }

    for neighbors in &mut adj {
        neighbors.sort_unstable();
    }

    let mut q = VecDeque::new();
    let mut order = Vec::new();

    for s in 1..=n {
        if visited[s] {
            continue;
        }

        q.push_back(s);
        visited[s] = true;

        while let Some(u) = q.pop_front() {
            order.push(u);

            for &v in &adj[u] {
                if !visited[v] {
                    visited[v] = true;
                    q.push_back(v);
                }
            }
        }
    }

    let result: Vec<String> =
        order.iter().map(|x| x.to_string()).collect();

    println!("{}", result.join(" "));
}
