use std::collections::HashMap;
use std::io::{self, Read, Write};

// `&T` borrows without taking ownership; `&mut T` permits changes. &[T] is a borrowed slice.
fn phone_valid(s: &str) -> bool {
    // `|x| ...` defines a closure; all returns true when every iterator item satisfies it.
    s.len() == 10 && s.bytes().all(|c| c.is_ascii_digit())
}
fn seconds(s: &str) -> i64 {
    let parts: Vec<i64> = s.split(':').map(|p| p.parse().unwrap()).collect();
    parts[0] * 3600 + parts[1] * 60 + parts[2]
}

fn main() {
    // `let` bindings are immutable by default; `mut` allows changes. String owns its text.
    let mut input = String::new();
    // `&mut input` lends exclusive mutable access; unwrap() extracts Ok or panics on an I/O error.
    io::stdin().read_to_string(&mut input).unwrap();
    // The iterator yields borrowed &str tokens; input must stay alive while they are used.
    let mut it = input.split_whitespace();
    let mut out = io::BufWriter::new(io::stdout().lock());
    let mut counts: HashMap<&str, i64> = HashMap::new();
    let mut durations: HashMap<&str, i64> = HashMap::new();
    let mut total = 0;
    let mut valid = true;
    // `while let Some(...)` keeps extracting values until the iterator/collection returns None.
    while let Some(cmd) = it.next() {
        if cmd == "#" {
            break;
        }
        if cmd == "call" {
            let from = it.next().unwrap();
            let to = it.next().unwrap();
            let _date = it.next().unwrap();
            let start = seconds(it.next().unwrap());
            let end = seconds(it.next().unwrap());
            valid &= phone_valid(from) && phone_valid(to);
            // entry(...).or_default() returns a mutable reference, inserting a default value if absent; * dereferences it.
            *counts.entry(from).or_default() += 1;
            *durations.entry(from).or_default() += end - start;
            total += 1;
        }
    }
    while let Some(cmd) = it.next() {
        // `match` selects a pattern arm (`pattern => expression`); `_` matches any remaining value.
        match cmd {
            "#" => break,
            // The ! marks a formatting macro; {name} captures a variable and {} uses the next argument.
            "?check_phone_number" => writeln!(out, "{}", u8::from(valid)).unwrap(),
            "?number_total_calls" => writeln!(out, "{total}").unwrap(),
            "?number_calls_from" => {
                let phone = it.next().unwrap();
                // get returns Option<&T>; unwrap_or(&0) supplies a borrowed zero when the key is absent.
                writeln!(out, "{}", counts.get(phone).unwrap_or(&0)).unwrap();
            }
            "?count_time_calls_from" => {
                let phone = it.next().unwrap();
                writeln!(out, "{}", durations.get(phone).unwrap_or(&0)).unwrap();
            }
            _ => {}
        }
    }
}
