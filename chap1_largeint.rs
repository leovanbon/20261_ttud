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
    let a = next!(String).into_bytes();
    let b = next!(String).into_bytes();
    // Tuple destructuring binds each component separately; mut applies only to the binding it precedes.
    let (mut i, mut j, mut carry) = (a.len(), b.len(), 0u8);
    let mut answer = Vec::new();
    while i > 0 || j > 0 || carry > 0 {
        let mut sum = carry;
        if i > 0 {
            i -= 1;
            // `b'0'` is a u8 byte literal; `'0'` is a Unicode char. Byte slices suit ASCII problems.
            sum += a[i] - b'0';
        }
        if j > 0 {
            j -= 1;
            sum += b[j] - b'0';
        }
        answer.push(b'0' + sum % 10);
        carry = sum / 10;
    }
    answer.reverse();
    out.write_all(&answer).unwrap();
}
