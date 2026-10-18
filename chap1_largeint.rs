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
    let a = next!(String).into_bytes();
    let b = next!(String).into_bytes();
    let (mut i, mut j, mut carry) = (a.len(), b.len(), 0u8);
    let mut answer = Vec::new();
    while i > 0 || j > 0 || carry > 0 {
        let mut sum = carry;
        if i > 0 {
            i -= 1;
            sum += a[i] - b'0';
        }
        if j > 0 {
            j -= 1;
            sum += b[j] - b'0';
        }
        answer.push(b'0' + sum % 10);
        carry = sum / 10;
    }
    answer.reverse();
    out.write_all(&answer).unwrap();
}
