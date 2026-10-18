use std::collections::VecDeque;
use std::io::{self, Read, Write};

fn main() {
    // `let` bindings are immutable by default; `mut` allows changes. String owns its text.
    let mut input = String::new();
    // `&mut input` lends exclusive mutable access; unwrap() extracts Ok or panics on an I/O error.
    io::stdin().read_to_string(&mut input).unwrap();
    // The iterator yields borrowed &str tokens; input must stay alive while they are used.
    let mut it = input.split_whitespace();
    // `macro_rules!` defines a macro (called with !). `$t:ty` matches a type; parse::<T>() selects it explicitly.
    // unwrap() extracts Some/Ok or panics; next!() infers the type, while next!(usize) specifies it.
    macro_rules! next {
        ($t:ty) => {
            it.next().unwrap().parse::<$t>().unwrap()
        };
        () => {
            it.next().unwrap().parse().unwrap()
        };
    }
    let mut out = io::BufWriter::new(io::stdout().lock());
    // `usize` is the unsigned type used for collection lengths and indices.
    let n: usize = next!();
    let m: usize = next!();
    let r = next!(usize) - 1;
    let c = next!(usize) - 1;
    // `|_| ...` is a closure ignoring its argument; map transforms items and collect builds the target collection.
    let grid: Vec<Vec<u8>> = (0..n).map(|_| (0..m).map(|_| next!()).collect()).collect();
    // `vec![value; n]` creates n cloned values; `vec![a, b]` lists elements. The ! marks a macro call.
    let mut dist = vec![vec![-1i32; m]; n];
    let mut queue = VecDeque::from([(r, c)]);
    dist[r][c] = 0;
    // `while let Some(...)` keeps extracting values until the iterator/collection returns None.
    while let Some((x, y)) = queue.pop_front() {
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            // `as Type` explicitly casts a number; usize/isize are pointer-sized unsigned/signed integers.
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx < 0 || nx >= n as isize || ny < 0 || ny >= m as isize {
                // The ! marks a formatting macro; {name} captures a variable and {} uses the next argument.
                write!(out, "{}", dist[x][y] + 1).unwrap();
                return;
            }
            // Tuple destructuring binds each component separately; mut applies only to the binding it precedes.
            let (nx, ny) = (nx as usize, ny as usize);
            if grid[nx][ny] == 1 || dist[nx][ny] != -1 {
                continue;
            }
            dist[nx][ny] = dist[x][y] + 1;
            queue.push_back((nx, ny));
        }
    }
    write!(out, "-1").unwrap();
}
