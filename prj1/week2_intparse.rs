use std::io::{self, Read, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    macro_rules! next {
        ($t:ty) => {
            it.next().unwrap().parse::<$t>().unwrap()
        };
        () => {
            it.next().unwrap().parse().unwrap()
        };
    }
    let mut out = io::BufWriter::new(io::stdout().lock());
    let n: usize = next!();
    let mut path = Vec::new();
    let mut stack = vec![(0, 1)];
    while let Some((sum, next)) = stack.last_mut() {
        if *next > n - *sum {
            stack.pop();
            path.pop();
            continue;
        }
        let x = *next;
        *next += 1;
        let new_sum = *sum + x;
        path.push(x);
        if new_sum == n {
            for value in &path {
                write!(out, "{value} ").unwrap();
            }
            writeln!(out).unwrap();
            path.pop();
        } else {
            stack.push((new_sum, x));
        }
    }
}
