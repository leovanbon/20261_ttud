use std::io::{self, Read, Write};

fn dfs(
    x: usize,
    y: usize,
    sum: i64,
    grid: &[Vec<i64>],
    visited: &mut [Vec<bool>],
    answer: &mut i64,
) {
    *answer = (*answer).max(sum);
    for (dx, dy) in [(-2, -1), (-2, 1), (-1, -2), (-1, 2), (1, -2), (1, 2), (2, -1), (2, 1)] {
        let nx = x as isize + dx;
        let ny = y as isize + dy;
        if nx < 0 || ny < 0 || nx >= grid.len() as isize || ny >= grid.len() as isize {
            continue;
        }
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
    let n: usize = next!();
    let r = next!(usize) - 1;
    let c = next!(usize) - 1;
    let grid: Vec<Vec<i64>> = (0..n).map(|_| (0..n).map(|_| next!()).collect()).collect();
    let mut visited = vec![vec![false; n]; n];
    visited[r][c] = true;
    let mut answer = grid[r][c];
    dfs(r, c, grid[r][c], &grid, &mut visited, &mut answer);
    writeln!(out, "{answer}").unwrap();
}
