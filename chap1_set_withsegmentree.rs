use std::io::{self, Read, Write};

// `&T` borrows without taking ownership; `&mut T` permits changes. &[T] is a borrowed slice.
fn update(t: &mut [usize], id: usize, l: i32, r: i32, p: i32, value: usize) {
    if p < l || p > r {
        return;
    }
    if l == r {
        t[id] = value;
        return;
    }
    let m = (l + r) / 2;
    if p <= m {
        update(t, id * 2, l, m, p, value);
    } else {
        update(t, id * 2 + 1, m + 1, r, p, value);
    }
    t[id] = t[id * 2] + t[id * 2 + 1];
}
fn find(t: &[usize], id: usize, l: i32, r: i32, q: i32) -> i32 {
    if r < q || t[id] == 0 {
        return 0;
    }
    if l == r {
        return l;
    }
    let m = (l + r) / 2;
    let left = find(t, id * 2, l, m, q);
    if left != 0 {
        left
    } else {
        find(t, id * 2 + 1, m + 1, r, q)
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
    const MAX: i32 = 100_007;
    // `usize` is the unsigned type used for collection lengths and indices.
    let n: usize = next!();
    // `vec![value; n]` creates n cloned values; `vec![a, b]` lists elements. The ! marks a macro call.
    // `as Type` explicitly casts a number; usize/isize are pointer-sized unsigned/signed integers.
    let mut tree = vec![0; 4 * MAX as usize + 5];
    // `a..b` excludes b; `a..=b` includes b.
    for _ in 0..n {
        update(&mut tree, 1, 1, MAX, next!(), 1);
    }
    // `while let Some(...)` keeps extracting values until the iterator/collection returns None.
    while let Some(cmd) = it.next() {
        if cmd == "#" {
            break;
        }
        let x: i32 = next!();
        // `match` selects a pattern arm (`pattern => expression`); `_` matches any remaining value.
        let answer = match cmd {
            "insert" => {
                update(&mut tree, 1, 1, MAX, x, 1);
                continue;
            }
            "remove" => {
                update(&mut tree, 1, 1, MAX, x, 0);
                continue;
            }
            "min_greater_equal" => find(&tree, 1, 1, MAX, x),
            "min_greater" if x < MAX => find(&tree, 1, 1, MAX, x + 1),
            "min_greater" => 0,
            _ => continue,
        };
        if answer == 0 {
            // The ! marks a formatting macro; {name} captures a variable and {} uses the next argument.
            writeln!(out, "NULL").unwrap();
        } else {
            writeln!(out, "{answer}").unwrap();
        }
    }
}
