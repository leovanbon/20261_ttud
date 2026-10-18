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
    // `usize` is the unsigned type used for collection lengths and indices.
    let n: usize = next!();
    let target: i64 = next!();
    // `|_| ...` is a closure ignoring its argument; map transforms items and collect builds the target collection.
    let mut a: Vec<i64> = (0..n).map(|_| next!()).collect();
    a.sort_unstable();
    // Tuple destructuring binds each component separately; mut applies only to the binding it precedes.
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
    // The ! marks a formatting macro; {name} captures a variable and {} uses the next argument.
    write!(out, "{answer}").unwrap();
}
