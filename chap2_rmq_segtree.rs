use std::io::{self, Read, Write};

// `&T` borrows without taking ownership; `&mut T` permits changes. &[T] is a borrowed slice.
fn query(tree: &[i32], n: usize, mut l: usize, mut r: usize) -> i32 {
    let mut answer = i32::MAX;
    l += n;
    r += n;
    while l < r {
        if l & 1 == 1 {
            answer = answer.min(tree[l]);
            l += 1;
        }
        if r & 1 == 1 {
            r -= 1;
            answer = answer.min(tree[r]);
        }
        l /= 2;
        r /= 2;
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
    // `vec![value; n]` creates n cloned values; `vec![a, b]` lists elements. The ! marks a macro call.
    let mut tree = vec![i32::MAX; 2 * n];
    // `a..b` excludes b; `a..=b` includes b.
    for i in 0..n {
        tree[n + i] = next!();
    }
    for i in (1..n).rev() {
        tree[i] = tree[2 * i].min(tree[2 * i + 1]);
    }
    let q: usize = next!();
    let mut answer = 0i64;
    for _ in 0..q {
        let l: usize = next!();
        let r: usize = next!();
        answer += i64::from(query(&tree, n, l, r + 1));
    }
    // The ! marks a formatting macro; {name} captures a variable and {} uses the next argument.
    writeln!(out, "{answer}").unwrap();
}
