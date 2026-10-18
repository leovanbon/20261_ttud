use std::io::{self};
const MOD: i64 = 1_000_000_007;

struct Parser {
    s: Vec<u8>,
    pos: usize,
}

// `impl Type` defines associated functions and methods; &mut self is a mutable borrow of the receiver.
impl Parser {
    // `Self` names the enclosing type; this associated function is called as Parser::new(...).
    fn new(input: String) -> Self {
        // `|c| ...` is a closure (like a C++ lambda); filter keeps items for which it returns true.
        let s = input.bytes().filter(|c| !c.is_ascii_whitespace()).collect();
        // Field shorthand: `s` means `s: s`; this final expression is returned without a semicolon.
        Parser { s, pos: 0 }
    }

    // `&T` borrows without taking ownership; `&mut T` permits changes. &[T] is a borrowed slice.
    // `Option<T>` is either Some(value) or None; absence is represented explicitly, without a null value.
    fn expression(&mut self) -> Option<i64> {
        // `?` unwraps Some(value), or immediately returns None from this Option-returning function.
        let mut val = self.term()?;

        // `b'0'` is a u8 byte literal; `'0'` is a Unicode char. Byte slices suit ASCII problems.
        while self.pos < self.s.len() && self.s[self.pos] == b'+' {
            self.pos += 1;

            let x = self.term()?;
            val = (val + x) % MOD;
        }

        // No trailing semicolon: this final expression is the function's return value.
        Some(val)
    }

    fn term(&mut self) -> Option<i64> {
        let mut val = self.factor()?;

        while self.pos < self.s.len() && self.s[self.pos] == b'*' {
            self.pos += 1;
            let x = self.factor()?;
            val = val * x % MOD;
        }

        Some(val)
    }

    fn factor(&mut self) -> Option<i64> {
        if self.pos >= self.s.len() {
            return None;
        }

        if self.s[self.pos] == b'(' {
            self.pos += 1;
            let val = self.expression()?;

            if self.pos >= self.s.len() || self.s[self.pos] != b')' {
                return None;
            }

            self.pos += 1;
            return Some(val);
        }

        if !self.s[self.pos].is_ascii_digit() {
            return None;
        }

        let mut val = 0i64;
        let mut positive = false;

        while self.pos < self.s.len() && self.s[self.pos].is_ascii_digit() {
            // `as Type` explicitly casts a number; usize/isize are pointer-sized unsigned/signed integers.
            let digit = (self.s[self.pos] - b'0') as i64;

            if digit != 0 {
                positive = true;
            }

            val = (val * 10 + digit) % MOD;
            self.pos += 1;
        }

        if !positive {
            return None;
        }

        Some(val)
    }
}

fn main() {
    // `let` bindings are immutable by default; `mut` allows changes. String owns its text.
    let mut input = String::new();
    // `&mut input` lends exclusive mutable access; unwrap() extracts Ok or panics on an I/O error.
    io::stdin().read_line(&mut input).unwrap();

    // Passing String by value moves ownership into new; input cannot be used afterward.
    let mut parser = Parser::new(input);

    if parser.s.is_empty() {
        // The ! marks a formatting macro; {name} captures a variable and {} uses the next argument.
        println!("NOT_CORRECT");
        return;
    }

    // `match` selects a pattern arm (`pattern => expression`); `_` matches any remaining value.
    match parser.expression() {
        // `if` after a pattern is a match guard: this arm runs only when the condition also holds.
        Some(ans) if parser.pos == parser.s.len() => {
            println!("{ans}")
        }
        _ => {
            println!("NOT_CORRECT")
        }
    }
}
