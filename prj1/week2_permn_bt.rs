use std::io::{self, Read, Write};

// `&T` borrows without taking ownership; `&mut T` permits changes. &[T] is a borrowed slice.
// `impl Write` accepts any output type implementing the Write trait; &mut borrows it for writing.
fn generate(n: usize, a: &mut Vec<usize>, used: &mut [bool], out: &mut impl Write) {
    if a.len() == n {
        for value in a {
            // The ! marks a formatting macro; {name} captures a variable and {} uses the next argument.
            write!(out, "{value} ").unwrap();
        }
        writeln!(out).unwrap();
        return;
    }
    // `a..b` excludes b; `a..=b` includes b.
    for i in 0..n {
        if used[i] {
            continue;
        }
        used[i] = true;
        a.push(i + 1);
        generate(n, a, used, out);
        a.pop();
        used[i] = false;
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
    // `vec![value; n]` creates n cloned values; `vec![a, b]` lists elements. The ! marks a macro call.
    generate(n, &mut Vec::new(), &mut vec![false; n], &mut out);
}
