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
    let q: i64 = next!();
    // `|_| ...` is a closure ignoring its argument; map transforms items and collect builds the target collection.
    let a: Vec<i64> = (0..n).map(|_| next!()).collect();
    // Tuple destructuring binds each component separately; mut applies only to the binding it precedes.
    let (mut sum, mut left, mut answer) = (0, 0, 0);
    // `a..b` excludes b; `a..=b` includes b.
    for right in 0..n {
        sum += a[right];
        while left <= right && sum > q {
            sum -= a[left];
            left += 1;
        }
        answer = answer.max(right + 1 - left);
    }
    // The ! marks a formatting macro; {name} captures a variable and {} uses the next argument.
    write!(out, "{answer}").unwrap();
}
