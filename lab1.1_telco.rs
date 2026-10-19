use std::collections::HashMap;
use std::io::{self, Read, Write};

fn phone_valid(s: &str) -> bool {
    s.len() == 10 && s.bytes().all(|c| c.is_ascii_digit())
}
fn seconds(s: &str) -> i64 {
    let parts: Vec<i64> = s.split(':').map(|p| p.parse().unwrap()).collect();
    parts[0] * 3600 + parts[1] * 60 + parts[2]
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let mut out = io::BufWriter::new(io::stdout().lock());

    let mut counts: HashMap<&str, i64> = HashMap::new();
    let mut durations: HashMap<&str, i64> = HashMap::new();
    let mut total = 0;
    let mut valid = true;

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
            *counts.entry(from).or_default() += 1;
            *durations.entry(from).or_default() += end - start;
            total += 1;
        }
    }

    while let Some(cmd) = it.next() {
        match cmd {
            "#" => break,
            "?check_phone_number" => writeln!(out, "{}", u8::from(valid)).unwrap(),
            "?number_total_calls" => writeln!(out, "{total}").unwrap(),
            "?number_calls_from" => {
                let phone = it.next().unwrap();
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
