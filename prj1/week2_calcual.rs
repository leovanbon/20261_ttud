use std::io;
const MOD: i64 = 1_000_000_007;

// `&T` borrows without taking ownership; `&mut T` permits changes. &[T] is a borrowed slice.
// `Option<T>` is either Some(value) or None; absence is represented explicitly, without a null value.
fn evaluate(s: &[u8]) -> Option<i64> {
    if s.is_empty() {
        return None;
    }
    // `b'0'` is a u8 byte literal; `'0'` is a Unicode char. Byte slices suit ASCII problems.
    // Tuple destructuring binds each component separately; mut applies only to the binding it precedes.
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
    // `let` bindings are immutable by default; `mut` allows changes. String owns its text.
    let mut input = String::new();
    // `&mut input` lends exclusive mutable access; unwrap() extracts Ok or panics on an I/O error.
    io::stdin().read_line(&mut input).unwrap();
    // `|c| ...` is a closure (like a C++ lambda); filter keeps items for which it returns true.
    let s: Vec<u8> = input.bytes().filter(|c| !c.is_ascii_whitespace()).collect();
    // `match` selects a pattern arm (`pattern => expression`); `_` matches any remaining value.
    match evaluate(&s) {
        // The ! marks a formatting macro; {name} captures a variable and {} uses the next argument.
        // `Some(answer)` extracts the successful result; None selects the failure arm.
        Some(answer) => print!("{answer}"),
        None => print!("NOT_CORRECT"),
    }
}
