use std::io::{self, Read, Write};

fn solve(board: &mut [[u8; 9]; 9], empty: &[(usize, usize)]) -> i64 {
    let Some((&(x, y), remaining)) = empty.split_last() else {
        return 1;
    };
    let mut answer = 0;
    for value in 1..=9 {
        if board[x].contains(&value) || (0..9).any(|i| board[i][y] == value) {
            continue;
        }
        if (x / 3 * 3..x / 3 * 3 + 3)
            .any(|i| (y / 3 * 3..y / 3 * 3 + 3).any(|j| board[i][j] == value))
        {
            continue;
        }
        board[x][y] = value;
        answer += solve(board, remaining);
        board[x][y] = 0;
    }
    answer
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
    let mut board = [[0u8; 9]; 9];
    let mut empty = Vec::new();
    for (i, row) in board.iter_mut().enumerate() {
        for (j, cell) in row.iter_mut().enumerate() {
            *cell = next!();
            if *cell == 0 {
                empty.push((i, j));
            }
        }
    }
    writeln!(out, "{}", solve(&mut board, &empty)).unwrap();
}
