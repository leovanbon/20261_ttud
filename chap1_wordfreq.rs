use std::collections::BTreeMap;
use std::io::{self, BufRead, Write};

fn main() {
    let mut counts = BTreeMap::<String, usize>::new();
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }
        // `|c: char| ...` is a closure with an explicit parameter type, used here as the split predicate.
        // `|c| ...` is a closure (like a C++ lambda); filter keeps items for which it returns true.
        for word in line.split(|c: char| !c.is_ascii_alphanumeric()).filter(|s| !s.is_empty()) {
            // entry(...).or_default() returns a mutable reference, inserting a default value if absent; * dereferences it.
            *counts.entry(word.to_owned()).or_default() += 1;
        }
    }
    let mut out = io::BufWriter::new(io::stdout().lock());
    for (word, count) in counts {
        // The ! marks a formatting macro; {name} captures a variable and {} uses the next argument.
        writeln!(out, "{word} {count}").unwrap();
    }
}
