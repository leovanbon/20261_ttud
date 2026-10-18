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
    // `vec![value; n]` creates n cloned values; `vec![a, b]` lists elements. The ! marks a macro call.
    let mut prefix = vec![0i64; n + 1];
    // `a..=b` includes b; `a..b` excludes b.
    for i in 1..=n {
        prefix[i] = prefix[i - 1] + next!(i64);
    }
    let q: usize = next!();
    // `a..b` excludes b; `a..=b` includes b.
    for _ in 0..q {
        let l: usize = next!();
        let r: usize = next!();
        // The ! marks a formatting macro; {name} captures a variable and {} uses the next argument.
        writeln!(out, "{}", prefix[r] - prefix[l - 1]).unwrap();
    }
}
