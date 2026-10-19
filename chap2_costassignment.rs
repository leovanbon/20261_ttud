use std::io::{self, Read, Write};

fn dfs(
    task: usize,
    n: usize,
    capacity: usize,
    cost: i64,
    costs: &[Vec<i64>],
    counts: &mut [usize],
    answer: &mut i64,
) {
    if cost >= *answer {
        return;
    }
    if task == n {
        *answer = cost;
        return;
    }

    for worker in 0..costs.len() {
        if counts[worker] < capacity {
            counts[worker] += 1;
            dfs(task + 1, n, capacity, cost + costs[worker][task], costs, counts, answer);
            counts[worker] -= 1;
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let mut out = io::BufWriter::new(io::stdout().lock());

    let m: usize = it.next().unwrap().parse().unwrap();
    let n: usize = it.next().unwrap().parse().unwrap();
    let capacity: usize = it.next().unwrap().parse().unwrap();
    let mut costs = vec![vec![0i64; n]; m];
    for row in &mut costs {
        for value in row {
            *value = it.next().unwrap().parse().unwrap();
        }
    }

    let mut answer = i64::MAX;
    if m * capacity >= n {
        dfs(0, n, capacity, 0, &costs, &mut vec![0; m], &mut answer);
    }

    write!(out, "{}", if answer == i64::MAX { -1 } else { answer }).unwrap();
}
