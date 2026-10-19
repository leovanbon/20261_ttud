use std::io::{self, Read, Write};

fn generate(n: usize, a: &mut Vec<usize>, used: &mut [bool], out: &mut impl Write) {
    if a.len() == n {
        for value in a {
            write!(out, "{value} ").unwrap();
        }
        writeln!(out).unwrap();
        return;
    }

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
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let mut out = io::BufWriter::new(io::stdout().lock());

    let n: usize = it.next().unwrap().parse().unwrap();
    generate(n, &mut Vec::new(), &mut vec![false; n], &mut out);
}
