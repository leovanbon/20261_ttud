use std::io;
const MOD: i64 = 1_000_000_007;

fn evaluate(s: &[u8]) -> Option<i64> {
    if s.is_empty() {
        return None;
    }
    let (mut answer, mut term, mut i, mut op) = (0, 1, 0, b'*');
    while i < s.len() {
        if !s[i].is_ascii_digit() {
            return None;
        }
        let (mut value, mut positive) = (0, false);
        while i < s.len() && s[i].is_ascii_digit() {
            positive |= s[i] != b'0';
            value = (value * 10 + i64::from(s[i] - b'0')) % MOD;
            i += 1;
        }
        if !positive {
            return None;
        }
        if op == b'*' {
            term = term * value % MOD;
        } else {
            answer = (answer + term) % MOD;
            term = value;
        }
        if i == s.len() {
            break;
        }
        op = s[i];
        if op != b'+' && op != b'*' {
            return None;
        }
        i += 1;
        if i == s.len() {
            return None;
        }
    }
    Some((answer + term) % MOD)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let s: Vec<u8> = input.bytes().filter(|c| !c.is_ascii_whitespace()).collect();
    match evaluate(&s) {
        Some(answer) => print!("{answer}"),
        None => print!("NOT_CORRECT"),
    }
}
