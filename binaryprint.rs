use std::io::{self, Read};
 
fn troll(k:usize, n: usize, s: &[u8], a: &mut Vec<u8>, ans: &mut i64) {
    if k == n {
        *ans += 1;
        return;
    }

    for c in [b'0', b'1'] {
        a.push(c);
        let m = s.len();

        let mut ok = true;

        if a.len() >= m {
            ok = &a[a.len() - m..] != s;
        }

        if ok {
            troll(k+1, n, s, a, ans);
        }

        a.pop();
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let s = it.next().unwrap().as_bytes();

    let mut a = Vec::new();
    let mut ans: i64 = 0;

    troll(0, n, s, &mut a, &mut ans);

    println!("{}", ans);
}
