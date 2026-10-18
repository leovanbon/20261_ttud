use std::io::{self, Read};
 
// `&T` borrows without taking ownership; `&mut T` permits changes. &[T] is a borrowed slice.
fn troll(k:usize, n: usize, s: &[u8], a: &mut Vec<u8>, ans: &mut i64) {
    if k == n {
        *ans += 1;
        return;
    }

    // `b'0'` is a u8 byte literal; `'0'` is a Unicode char. Byte slices suit ASCII problems.
    for c in [b'0', b'1'] {
        a.push(c);
        let m = s.len();

        let mut ok = true;

        if a.len() >= m {
            // `a[start..end]` borrows a slice with an exclusive end; omitted bounds use the start/end of a.
            ok = &a[a.len() - m..] != s;
        }

        if ok {
            troll(k+1, n, s, a, ans);
        }

        a.pop();
    }
}

fn main() {
    // `let` bindings are immutable by default; `mut` allows changes. String owns its text.
    let mut input = String::new();
    // `&mut input` lends exclusive mutable access; unwrap() extracts Ok or panics on an I/O error.
    io::stdin().read_to_string(&mut input).unwrap();
    
    // The iterator yields borrowed &str tokens; input must stay alive while they are used.
    let mut it = input.split_whitespace();

    // `usize` is the unsigned type used for collection lengths and indices.
    let n: usize = it.next().unwrap().parse().unwrap();
    let s = it.next().unwrap().as_bytes();

    let mut a = Vec::new();
    let mut ans: i64 = 0;

    troll(0, n, s, &mut a, &mut ans);

    // The ! marks a formatting macro; {name} captures a variable and {} uses the next argument.
    println!("{}", ans);
}
