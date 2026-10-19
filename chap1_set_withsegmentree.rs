use std::io::{self, Read, Write};

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
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let mut out = io::BufWriter::new(io::stdout().lock());

    const MAX: i32 = 100_007;
    let n: usize = it.next().unwrap().parse().unwrap();
    let mut tree = vec![0; 4 * MAX as usize + 5];

    for _ in 0..n {
        update(&mut tree, 1, 1, MAX, it.next().unwrap().parse().unwrap(), 1);
    }

    while let Some(cmd) = it.next() {
        if cmd == "#" {
            break;
        }
        let x: i32 = it.next().unwrap().parse().unwrap();
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
            writeln!(out, "NULL").unwrap();
        } else {
            writeln!(out, "{answer}").unwrap();
        }
    }
}
