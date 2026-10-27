use std::io::{self, Read, Write};

fn generate(n: usize, s: &mut Vec<u8>, out: &mut impl Write) {
    if s.len() == n {
        if s.windows(3).any(|w| w == b"111") {
            out.write_all(s).unwrap();
            writeln!(out).unwrap();
        }
        return;
    }

    for c in [b'0', b'1'] {
        s.push(c);
        generate(n, s, out);
        s.pop();
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let mut out = io::BufWriter::new(io::stdout().lock());

    let n: usize = it.next().unwrap().parse().unwrap();
    generate(n, &mut Vec::new(), &mut out);
}
