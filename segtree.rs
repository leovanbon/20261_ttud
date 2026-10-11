use std::fs::File;
use std::io::{Read, Write};

const M: usize = 100000;

fn update(t: &mut Vec<i32>, id: usize, l: usize, r: usize, p: usize, v: i32) {
    if l == r {
        t[id] = v;
        return;
    }

    let m = (l + r) / 2;

    if p <= m {
        update(t, id * 2, l, m, p, v);
    } else {
        update(t, id * 2 + 1, m + 1, r, p, v);
    }

    t[id] = t[id * 2] + t[id * 2 + 1];
}

fn get(t: &Vec<i32>, id: usize, l: usize, r: usize, q: usize) -> usize {
    if r < q || t[id] == 0 {
        return 0;
    }

    if l == r {
        return l;
    }

    let m = (l + r) / 2;
    let x = get(t, id * 2, l, m, q);

    if x != 0 {
        x
    } else {
        get(t, id * 2 + 1, m + 1, r, q)
    }
}

fn main() {
    let mut s = String::new();
    File::open("input.txt")
        .unwrap()
        .read_to_string(&mut s)
        .unwrap();

    let mut it = s.split_whitespace();
    let mut out = File::create("output.txt").unwrap();

    let n: usize = it.next().unwrap().parse().unwrap();
    let mut t = vec![0; 4 * M + 5];

    for _ in 0..n {
        let x = it.next().unwrap().parse().unwrap();
        update(&mut t, 1, 1, M, x, 1);
    }

    while let Some(op) = it.next() {
        if op == "#" {
            break;
        }

        let x: usize = it.next().unwrap().parse().unwrap();

        match op {
            "insert" => update(&mut t, 1, 1, M, x, 1),

            "remove" => update(&mut t, 1, 1, M, x, 0),

            "min_greater_equal" => {
                let ans = if x <= M { get(&t, 1, 1, M, x) } else { 0 };

                if ans == 0 {
                    writeln!(out, "NULL").unwrap();
                } else {
                    writeln!(out, "{ans}").unwrap();
                }
            }

            "min_greater" => {
                let ans = if x < M { get(&t, 1, 1, M, x + 1) } else { 0 };

                if ans == 0 {
                    writeln!(out, "NULL").unwrap();
                } else {
                    writeln!(out, "{ans}").unwrap();
                }
            }

            _ => {}
        }
    }
}
