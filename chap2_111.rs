use std::io::{self, Read, Write};

// `&T` borrows without taking ownership; `&mut T` permits changes. &[T] is a borrowed slice.
// `impl Write` accepts any output type implementing the Write trait; &mut borrows it for writing.
fn generate(n: usize, s: &mut Vec<u8>, out: &mut impl Write) {
    if s.len() == n {
        // `|x| ...` defines a closure; any returns true when at least one iterator item satisfies it.
        // `b"111"` is a byte-string literal, suitable for comparison with a byte slice.
        if s.windows(3).any(|w| w == b"111") {
            out.write_all(s).unwrap();
            // The ! marks a formatting macro; {name} captures a variable and {} uses the next argument.
            writeln!(out).unwrap();
        }
        return;
    }
    // `b'0'` is a u8 byte literal; `'0'` is a Unicode char. Byte slices suit ASCII problems.
    for c in [b'0', b'1'] {
        s.push(c);
        generate(n, s, out);
        s.pop();
    }
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
    generate(n, &mut Vec::new(), &mut out);
}
