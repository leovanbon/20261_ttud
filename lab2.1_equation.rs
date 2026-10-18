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
    let target: i64 = next!();
    // `|_| ...` is a closure ignoring its argument; map transforms items and collect builds the target collection.
    let a: Vec<usize> = (0..n).map(|_| next!()).collect();
    // `|&x|` destructures a reference argument, copying its value; map transforms each iterator item.
    // `as Type` explicitly casts a number; usize/isize are pointer-sized unsigned/signed integers.
    let remaining = target - a.iter().map(|&x| x as i64).sum::<i64>();
    if remaining < 0 {
        // The ! marks a formatting macro; {name} captures a variable and {} uses the next argument.
        write!(out, "0").unwrap();
        return;
    }
    let remaining = remaining as usize;
    // `vec![value; n]` creates n cloned values; `vec![a, b]` lists elements. The ! marks a macro call.
    let mut dp = vec![0i64; remaining + 1];
    dp[0] = 1;
    for value in a {
        // `a..=b` includes b; `a..b` excludes b.
        for sum in value..=remaining {
            dp[sum] += dp[sum - value];
        }
    }
    write!(out, "{}", dp[remaining]).unwrap();
}
