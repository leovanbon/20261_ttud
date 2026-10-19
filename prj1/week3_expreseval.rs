use std::io::{self, Read};
const MOD: i64 = 1_000_000_007;

struct Parser {
    s: Vec<u8>,
    pos: usize,
}

impl Parser {
    fn new(input: String) -> Self {
        let s = input.bytes().filter(|c| !c.is_ascii_whitespace()).collect();
        Parser { s, pos: 0 }
    }
    fn expression(&mut self) -> Option<i64> {
        let mut val = self.term()?;
        while self.pos < self.s.len() && self.s[self.pos] == b'+' {
            self.pos += 1;
            let x = self.term()?;
            val = (val + x) % MOD;
        }
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
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut parser = Parser::new(input);
    if parser.s.is_empty() {
        println!("NOT_CORRECT");
        return;
    }
    match parser.expression() {
        Some(ans) if parser.pos == parser.s.len() => {
            println!("{ans}")
        }
        _ => {
            println!("NOT_CORRECT")
        }
    }
}
