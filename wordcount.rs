use std::collections::BTreeMap;
use std::io::{self, Read};

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(mut s).unwrap();

    let mut cnt = BTreeMap::new();

    for word in s.split(|c: char| !c.is_ascii_alphanumeric()) {
        if !word.is_empty() {
            *cnt.entry(word).or_insert(0) += 1;
        }
    }

    for (word,count) in cnt {
        println("{} {}", word, count);
    }

    println!("Hello, world!");
}
