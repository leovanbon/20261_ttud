use std::collections::BTreeMap;
use std::io::{self, BufRead, Write};

fn main() {
    let mut counts = BTreeMap::<String, usize>::new();
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }
        for word in line.split(|c: char| !c.is_ascii_alphanumeric()).filter(|s| !s.is_empty()) {
            *counts.entry(word.to_owned()).or_default() += 1;
        }
    }
    let mut out = io::BufWriter::new(io::stdout().lock());
    for (word, count) in counts {
        writeln!(out, "{word} {count}").unwrap();
    }
}
