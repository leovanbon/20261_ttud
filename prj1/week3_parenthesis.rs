use std::io::{self, Read, Write};

fn main() {
    // `let` bindings are immutable by default; `mut` allows changes. String owns its text.
    let mut input = String::new();
    // `&mut input` lends exclusive mutable access; unwrap() extracts Ok or panics on an I/O error.
    io::stdin().read_to_string(&mut input).unwrap();
    // The iterator yields borrowed &str tokens; input must stay alive while they are used.
    let mut it = input.split_whitespace();
    let mut out = io::BufWriter::new(io::stdout().lock());
    let s = it.next().unwrap_or("");
    let mut stack = Vec::new();
    for c in s.bytes() {
        // `match` selects a pattern arm (`pattern => expression`); `_` matches any remaining value.
        match c {
            // `|` between patterns means either pattern matches, unlike the logical OR operator ||.
            // `b'0'` is a u8 byte literal; `'0'` is a Unicode char. Byte slices suit ASCII problems.
            b'(' | b'[' | b'{' => stack.push(c),
            _ => {
                // `let ... else` extracts a matching value; the else branch must exit (return, break, etc.).
                let Some(top) = stack.pop() else {
                    // The ! marks a formatting macro; {name} captures a variable and {} uses the next argument.
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
