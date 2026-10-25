use std::io::{self, Read};

fn dfs(
    u: usize,
    cnt: usize,
    adj: &[Vec<bool>],
    visited: &mut [bool]
) -> bool {
    let n = adj.len();

    if cnt == n { return adj[u][0]; }

    for v in 0..n {
        if adj[u][v] && !visited[v]  {
            visited[v] = true;

            if dfs(v, cnt + 1, adj, visited) { return true; }

            visited[v] = false;
        }
    }

    false
}

fn solve<'a>(iter: &mut impl Iterator<Item = &'a str>) -> bool {
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    let mut adj = vec![vec![false;n]; n];
    let mut visited = vec![false; n];

    for _ in 0..m {
        let u = iter.next().unwrap().parse::<usize>().unwrap() - 1;
        let v = iter.next().unwrap().parse::<usize>().unwrap() - 1;
    
        adj[u][v] = true;
        adj[v][u] = true;
    }

    if n < 3 { return false; }

    visited[0] = true;

    dfs(0, 1, &adj, &mut visited)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut iter = input.split_whitespace();

    let t = iter.next().unwrap().parse::<usize>().unwrap();

    for _ in 0..t {
        println!("{}", if solve(&mut iter) {1} else {0});
    }
}