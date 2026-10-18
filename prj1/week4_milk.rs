use std::collections::VecDeque;
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
    let a: usize = next!();
    let b: usize = next!();
    let target: usize = next!();
    let mut x = a;
    let mut y = b;
    while y != 0 {
        (x, y) = (y, x % y);
    }
    if target == 0 {
        writeln!(out, "0").unwrap();
        return;
    }
    if target > a.max(b) || x == 0 || target % x != 0 {
        writeln!(out, "-1").unwrap();
        return;
    }
    let mut dist = vec![vec![-1i32; b + 1]; a + 1];
    let mut queue = VecDeque::from([(0, 0)]);
    dist[0][0] = 0;
    while let Some((x, y)) = queue.pop_front() {
        if x == target || y == target {
            writeln!(out, "{}", dist[x][y]).unwrap();
            return;
        }
        let pour_ab = x.min(b - y);
        let pour_ba = y.min(a - x);
        for (nx, ny) in
            [(a, y), (x, b), (0, y), (x, 0), (x - pour_ab, y + pour_ab), (x + pour_ba, y - pour_ba)]
        {
            if dist[nx][ny] == -1 {
                dist[nx][ny] = dist[x][y] + 1;
                queue.push_back((nx, ny));
            }
        }
    }
    writeln!(out, "-1").unwrap();
}
