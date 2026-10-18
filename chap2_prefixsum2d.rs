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
    let m: usize = next!();
    let n: usize = next!();
    // `vec![value; n]` creates n cloned values; `vec![a, b]` lists elements. The ! marks a macro call.
    let mut prefix = vec![vec![0i64; n + 1]; m + 1];
    // `a..=b` includes b; `a..b` excludes b.
    for i in 1..=m {
        for j in 1..=n {
            prefix[i][j] = prefix[i - 1][j] + prefix[i][j - 1] - prefix[i - 1][j - 1] + next!(i64);
        }
    }
    let q: usize = next!();
    // `a..b` excludes b; `a..=b` includes b.
    for _ in 0..q {
        let x1: usize = next!();
        let y1: usize = next!();
        let x2: usize = next!();
        let y2: usize = next!();
        // The ! marks a formatting macro; {name} captures a variable and {} uses the next argument.
        writeln!(
            out,
            "{}",
            prefix[x2][y2] - prefix[x1 - 1][y2] - prefix[x2][y1 - 1] + prefix[x1 - 1][y1 - 1]
        )
        .unwrap();
    }
}
