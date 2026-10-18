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
    // `usize` is the unsigned type used for collection lengths and indices.
    let n: usize = next!();
    // `vec![value; n]` creates n cloned values; `vec![a, b]` lists elements. The ! marks a macro call.
    let mut tree = vec![-1_000_000_000i32; 2 * n];
    // `a..b` excludes b; `a..=b` includes b.
    for i in 0..n {
        tree[n + i] = next!();
    }
    for i in (1..n).rev() {
        tree[i] = tree[2 * i].max(tree[2 * i + 1]);
    }
    let m: usize = next!();
    for _ in 0..m {
        let cmd = it.next().unwrap();
        let x: usize = next!();
        let y: i32 = next!();
        if cmd == "update" {
            let mut p = n + x - 1;
            tree[p] = y;
            while p > 1 {
                p /= 2;
                tree[p] = tree[p * 2].max(tree[p * 2 + 1]);
            }
        } else if cmd == "get-max" {
            // `as Type` explicitly casts a number; usize/isize are pointer-sized unsigned/signed integers.
            // Tuple destructuring binds each component separately; mut applies only to the binding it precedes.
            let (mut l, mut r) = (n + x - 1, n + y as usize);
            let mut answer = -1_000_000_000;
            while l < r {
                if l & 1 == 1 {
                    answer = answer.max(tree[l]);
                    l += 1;
                }
                if r & 1 == 1 {
                    r -= 1;
                    answer = answer.max(tree[r]);
                }
                l /= 2;
                r /= 2;
            }
            // The ! marks a formatting macro; {name} captures a variable and {} uses the next argument.
            writeln!(out, "{answer}").unwrap();
        }
    }
}
