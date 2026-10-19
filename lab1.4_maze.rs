use std::collections::VecDeque;
use std::io::{self, Read, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let mut out = io::BufWriter::new(io::stdout().lock());

    let n: usize = it.next().unwrap().parse().unwrap();
    let m: usize = it.next().unwrap().parse().unwrap();
    let r = it.next().unwrap().parse::<usize>().unwrap() - 1;
    let c = it.next().unwrap().parse::<usize>().unwrap() - 1;
    let mut grid = vec![vec![0u8; m]; n];
    for row in &mut grid {
        for value in row {
            *value = it.next().unwrap().parse().unwrap();
        }
    }

    let mut dist = vec![vec![-1i32; m]; n];
    let mut queue = VecDeque::from([(r, c)]);
    dist[r][c] = 0;

    while let Some((x, y)) = queue.pop_front() {
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx < 0 || nx >= n as isize || ny < 0 || ny >= m as isize {
                write!(out, "{}", dist[x][y] + 1).unwrap();
                return;
            }
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
