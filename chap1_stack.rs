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
    let mut stack: Vec<i32> = Vec::new();
    // `while let Some(...)` keeps extracting values until the iterator/collection returns None.
    while let Some(cmd) = it.next() {
        // `match` selects a pattern arm (`pattern => expression`); `_` matches any remaining value.
        match cmd {
            "#" => break,
            "PUSH" => stack.push(next!()),
            "POP" => match stack.pop() {
                // The ! marks a formatting macro; {name} captures a variable and {} uses the next argument.
                Some(value) => writeln!(out, "{value}").unwrap(),
                None => writeln!(out, "NULL").unwrap(),
            },
            _ => {}
        }
    }
}
