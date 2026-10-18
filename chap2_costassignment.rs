use std::io::{self, Read, Write};

fn dfs(
    task: usize,
    n: usize,
    capacity: usize,
    cost: i64,
    // Borrowed parameters: &T reads shared data, &mut T permits changes, and [T] denotes a slice.
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
    // `a..b` excludes b; `a..=b` includes b.
    for worker in 0..costs.len() {
        if counts[worker] < capacity {
            counts[worker] += 1;
            dfs(task + 1, n, capacity, cost + costs[worker][task], costs, counts, answer);
            counts[worker] -= 1;
        }
    }
}

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
    // `usize` is the unsigned type used for collection lengths and indices.
    let m: usize = next!();
    let n: usize = next!();
    let capacity: usize = next!();
    // `|_| ...` is a closure ignoring its argument; map transforms items and collect builds the target collection.
    let costs: Vec<Vec<i64>> = (0..m).map(|_| (0..n).map(|_| next!()).collect()).collect();
    let mut answer = i64::MAX;
    if m * capacity >= n {
        // `vec![value; n]` creates n cloned values; `vec![a, b]` lists elements. The ! marks a macro call.
        dfs(0, n, capacity, 0, &costs, &mut vec![0; m], &mut answer);
    }
    // The ! marks a formatting macro; {name} captures a variable and {} uses the next argument.
    // Rust if/else is an expression; both branches must produce the same type.
    write!(out, "{}", if answer == i64::MAX { -1 } else { answer }).unwrap();
}
