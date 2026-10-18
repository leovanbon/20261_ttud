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
    let mut path = Vec::new();
    // `vec![value; n]` creates n cloned values; `vec![a, b]` lists elements. The ! marks a macro call.
    let mut stack = vec![(0, 1)];
    // `while let Some(...)` keeps extracting values until the iterator/collection returns None.
    // last_mut lends the last tuple mutably; sum and next are &mut values, so * accesses their contents.
    while let Some((sum, next)) = stack.last_mut() {
        if *next > n - *sum {
            stack.pop();
            path.pop();
            continue;
        }
        let x = *next;
        *next += 1;
        let new_sum = *sum + x;
        path.push(x);
        if new_sum == n {
            // `for x in &collection` borrows its elements; iterating an owned Vec directly would consume it.
            for value in &path {
                // The ! marks a formatting macro; {name} captures a variable and {} uses the next argument.
                write!(out, "{value} ").unwrap();
            }
            writeln!(out).unwrap();
            path.pop();
        } else {
            stack.push((new_sum, x));
        }
    }
}
