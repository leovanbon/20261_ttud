use std::{self, Read};

macro_rules! read {
    ($iter:expr, $t:ty) => {
        $iter.next().unwrap().parse::<$t>().unwrap()
    };
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let mut n = read!(iter, usize);
    let mut m = read!(iter, usize);


    let mut adj = vec![vec![false;n];n];
    let mut visited = vec![false;n];

    for _ in 0..m {
        let u = read!(iter, usize);
        let v = read!(iter, usize);

        adj[u].push(v); adj[v].push(u);
    }

    for neighbors in &mut adj {
        neighbors.sort_unstable();
    }

    let mut stack = Vec::new();
    let mut order = Vec::new():

    for s in 1..=n {
        if visited[s] {
            continue;
        }

        stack.push(s);

        while let Some(u) = stack.pop() {
            if visited[u] {
                continue;
            }

            visited[u] = true;
            order.push(u);

            for &v in adj[u].iter().rev() {
                if !visited[v] {
                    stack.push(v);
                }
            }
        }
    }

    let result: Vec<String> = order.iter().map(|x| x.to_string()).collect();

    println!("{}", result.join(" "));
}