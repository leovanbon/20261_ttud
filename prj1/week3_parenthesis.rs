use std::io::{self, Read, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let mut out = io::BufWriter::new(io::stdout().lock());
    let s = it.next().unwrap_or("");
    let mut stack = Vec::new();
    for c in s.bytes() {
        match c {
            b'(' | b'[' | b'{' => stack.push(c),
            _ => {
                let Some(top) = stack.pop() else {
                    write!(out, "0").unwrap();
                    return;
                };
                if (c == b')' && top != b'(')
                    || (c == b']' && top != b'[')
                    || (c == b'}' && top != b'{')
                {
                    write!(out, "0").unwrap();
                    return;
                }
            }
        }
    }
    write!(out, "{}", u8::from(stack.is_empty())).unwrap();
}
