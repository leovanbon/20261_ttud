use std::io::{self, Read, Write};

// `&T` borrows without taking ownership; `&mut T` permits changes. &[T] is a borrowed slice.
fn solve(board: &mut [[u8; 9]; 9], empty: &[(usize, usize)]) -> i64 {
    // `let ... else` extracts a matching value; the else branch must exit (return, break, etc.).
    // split_last returns Option<(&T, &[T])>; &(x, y) copies the last tuple and remaining borrows the rest.
    let Some((&(x, y), remaining)) = empty.split_last() else {
        return 1;
    };
    let mut answer = 0;
    // `a..=b` includes b; `a..b` excludes b.
    for value in 1..=9 {
        // `|x| ...` defines a closure; any returns true when at least one iterator item satisfies it.
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
    let mut board = [[0u8; 9]; 9];
    let mut empty = Vec::new();
    // iter_mut yields mutable references; enumerate pairs each with its index. *cell writes through a reference.
    for (i, row) in board.iter_mut().enumerate() {
        for (j, cell) in row.iter_mut().enumerate() {
            *cell = next!();
            if *cell == 0 {
                empty.push((i, j));
            }
        }
    }
    // The ! marks a formatting macro; {name} captures a variable and {} uses the next argument.
    writeln!(out, "{}", solve(&mut board, &empty)).unwrap();
}
