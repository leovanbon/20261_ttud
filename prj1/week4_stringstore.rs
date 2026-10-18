use std::collections::HashSet;
use std::io::{self, Read, Write};

fn main() {
    // `let` bindings are immutable by default; `mut` allows changes. String owns its text.
    let mut input = String::new();
    // `&mut input` lends exclusive mutable access; unwrap() extracts Ok or panics on an I/O error.
    io::stdin().read_to_string(&mut input).unwrap();
    // The iterator yields borrowed &str tokens; input must stay alive while they are used.
    let mut it = input.split_whitespace();
    let mut out = io::BufWriter::new(io::stdout().lock());
    let mut strings = HashSet::new();
    // `while let Some(...)` keeps extracting values until the iterator/collection returns None.
    while let Some(s) = it.next() {
        if s == "*" {
            break;
        }
        strings.insert(s);
    }
    while let Some(cmd) = it.next() {
        if cmd == "***" {
            break;
        }
        let s = it.next().unwrap();
        // `match` selects a pattern arm (`pattern => expression`); `_` matches any remaining value.
        let answer = match cmd {
            "find" => strings.contains(s),
            "insert" => strings.insert(s),
            _ => continue,
        };
        // The ! marks a formatting macro; {name} captures a variable and {} uses the next argument.
        writeln!(out, "{}", u8::from(answer)).unwrap();
    }
}
