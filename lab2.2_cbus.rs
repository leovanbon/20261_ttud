use std::io::{self, Read, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let mut out = io::BufWriter::new(io::stdout().lock());

    const INF: i32 = 1_000_000_000;
    let n: usize = it.next().unwrap().parse().unwrap();
    let capacity: u32 = it.next().unwrap().parse().unwrap();
    let points = 2 * n;
    let mut costs = vec![vec![0i32; points + 1]; points + 1];
    for row in &mut costs {
        for value in row {
            *value = it.next().unwrap().parse().unwrap();
        }
    }

    let states = 1usize << points;
    let mut dp = vec![vec![INF; points + 1]; states];
    dp[0][0] = 0;

    for mask in 0..states {
        let pickups = mask & ((1 << n) - 1);
        let deliveries = mask >> n;
        if deliveries & !pickups != 0 {
            continue;
        }
        let onboard = pickups.count_ones() - deliveries.count_ones();
        for last in 0..=points {
            let cost = dp[mask][last];
            if cost == INF {
                continue;
            }
            for i in 0..n {
                let pickup = 1 << i;
                let delivery = 1 << (i + n);
                if onboard < capacity && mask & pickup == 0 {
                    let next = mask | pickup;
                    dp[next][i + 1] = dp[next][i + 1].min(cost + costs[last][i + 1]);
                }
                if mask & pickup != 0 && mask & delivery == 0 {
                    let next = mask | delivery;
                    dp[next][i + n + 1] = dp[next][i + n + 1].min(cost + costs[last][i + n + 1]);
                }
            }
        }
    }
    let mut answer = INF;

    for last in 1..=points {
        answer = answer.min(dp[states - 1][last] + costs[last][0]);
    }

    writeln!(out, "{answer}").unwrap();
}
