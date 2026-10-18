use std::io::{self, Read, Write};

fn dfs(
    x: usize,
    y: usize,
    sum: i64,
    // Borrowed parameters: &T reads shared data, &mut T permits changes, and [T] denotes a slice.
    grid: &[Vec<i64>],
    visited: &mut [Vec<bool>],
    answer: &mut i64,
) {
    *answer = (*answer).max(sum);
    for (dx, dy) in [(-2, -1), (-2, 1), (-1, -2), (-1, 2), (1, -2), (1, 2), (2, -1), (2, 1)] {
        // `as Type` explicitly casts a number; usize/isize are pointer-sized unsigned/signed integers.
        let nx = x as isize + dx;
        let ny = y as isize + dy;
        if nx < 0 || ny < 0 || nx >= grid.len() as isize || ny >= grid.len() as isize {
            continue;
        }
        // Tuple destructuring binds each component separately; mut applies only to the binding it precedes.
        let (nx, ny) = (nx as usize, ny as usize);
        if visited[nx][ny] {
            continue;
        }
        visited[nx][ny] = true;
        dfs(nx, ny, sum + grid[nx][ny], grid, visited, answer);
        visited[nx][ny] = false;
    }
}

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
    let r = next!(usize) - 1;
    let c = next!(usize) - 1;
    // `|_| ...` is a closure ignoring its argument; map transforms items and collect builds the target collection.
    let grid: Vec<Vec<i64>> = (0..n).map(|_| (0..n).map(|_| next!()).collect()).collect();
    // `vec![value; n]` creates n cloned values; `vec![a, b]` lists elements. The ! marks a macro call.
    let mut visited = vec![vec![false; n]; n];
    visited[r][c] = true;
    let mut answer = grid[r][c];
    dfs(r, c, grid[r][c], &grid, &mut visited, &mut answer);
    // The ! marks a formatting macro; {name} captures a variable and {} uses the next argument.
    writeln!(out, "{answer}").unwrap();
}
