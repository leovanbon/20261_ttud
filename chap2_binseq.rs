use std::io::{self, Read, Write};

// `&T` borrows without taking ownership; `&mut T` permits changes. &[T] is a borrowed slice.
fn count(n: usize, pattern: &[u8], current: &mut Vec<u8>) -> i64 {
    if current.len() == n {
        return 1;
    }
    let mut answer = 0;
    // `b'0'` is a u8 byte literal; `'0'` is a Unicode char. Byte slices suit ASCII problems.
    for bit in [b'0', b'1'] {
        current.push(bit);
        if !current.ends_with(pattern) {
            answer += count(n, pattern, current);
        }
        current.pop();
    }
    answer
}

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
    let pattern = next!(String).into_bytes();
    // The ! marks a formatting macro; {name} captures a variable and {} uses the next argument.
    writeln!(out, "{}", count(n, &pattern, &mut Vec::new())).unwrap();
}
