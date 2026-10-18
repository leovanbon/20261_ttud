use std::io::{self, Read, Write};

fn main() {
    // `let` bindings are immutable by default; `mut` allows changes. String owns its text.
    let mut input = String::new();
    // `&mut input` lends exclusive mutable access; unwrap() extracts Ok or panics on an I/O error.
    io::stdin().read_to_string(&mut input).unwrap();
    // The iterator yields borrowed &str tokens; input must stay alive while they are used.
    let mut it = input.split_whitespace();
    // `macro_rules!` defines a macro (called with !). `$t:ty` matches a type; parse::<T>() selects it explicitly.
    // unwrap() extracts Some/Ok or panics; next!() infers the type, while next!(usize) specifies it.
    macro_rules! next {
        ($t:ty) => {
            it.next().unwrap().parse::<$t>().unwrap()
        };
        () => {
            it.next().unwrap().parse().unwrap()
        };
    }
    let mut out = io::BufWriter::new(io::stdout().lock());
    const INF: i32 = 1_000_000_000;
    // `usize` is the unsigned type used for collection lengths and indices.
    let n: usize = next!();
    let capacity: u32 = next!();
    let points = 2 * n;
    let costs: Vec<Vec<i32>> =
        // `|_| ...` is a closure ignoring its argument; map transforms items and collect builds the target collection.
        (0..=points).map(|_| (0..=points).map(|_| next!()).collect()).collect();
    let states = 1usize << points;
    // `vec![value; n]` creates n cloned values; `vec![a, b]` lists elements. The ! marks a macro call.
    let mut dp = vec![vec![INF; points + 1]; states];
    dp[0][0] = 0;
    // `a..b` excludes b; `a..=b` includes b.
    for mask in 0..states {
        let pickups = mask & ((1 << n) - 1);
        let deliveries = mask >> n;
        // Only reachable masks have every delivery preceded by its pickup.
        if deliveries & !pickups != 0 {
            continue;
        }
        let onboard = pickups.count_ones() - deliveries.count_ones();
        // `a..=b` includes b; `a..b` excludes b.
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
    // The ! marks a formatting macro; {name} captures a variable and {} uses the next argument.
    writeln!(out, "{answer}").unwrap();
}
