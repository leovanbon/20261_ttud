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
    let mut a: Vec<usize> = (1..=n).collect();
    loop {
        // `for x in &collection` borrows its elements; iterating an owned Vec directly would consume it.
        for value in &a {
            // The ! marks a formatting macro; {name} captures a variable and {} uses the next argument.
            write!(out, "{value} ").unwrap();
        }
        writeln!(out).unwrap();
        // `let ... else` extracts a matching value; the else branch must exit (return, break, etc.).
        // `|&i|` copies an integer from a reference; find returns Some(first_match) or None.
        // saturating_sub stops at zero for unsigned values, avoiding subtraction underflow.
        let Some(i) = (0..n.saturating_sub(1)).rev().find(|&i| a[i] < a[i + 1]) else {
            break;
        };
        let j = (i + 1..n).rev().find(|&j| a[j] > a[i]).unwrap();
        a.swap(i, j);
        // `a[start..end]` borrows a slice with an exclusive end; omitted bounds use the start/end of a.
        a[i + 1..].reverse();
    }
}
