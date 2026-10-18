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
    // `|_| ...` is a closure ignoring its argument; map transforms items and collect builds the target collection.
    let mut a: Vec<i32> = (0..n).map(|_| next!()).collect();
    a.sort_unstable();
    // `while let Some(...)` keeps extracting values until the iterator/collection returns None.
    while let Some(cmd) = it.next() {
        if cmd == "#" {
            break;
        }
        let k: i32 = next!();
        // `|&value| ...` is a closure that copies the referenced integer into value.
        let index = a.partition_point(|&value| value <= k);
        // copied() turns Option<&T> into Option<T> for Copy types; unwrap_or supplies a value when None.
        // The ! marks a formatting macro; {name} captures a variable and {} uses the next argument.
        writeln!(out, "{}", a.get(index).copied().unwrap_or(-1)).unwrap();
    }
}
