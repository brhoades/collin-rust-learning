use std::{fmt::Display, process};

fn main() {
    let n = 4;
    let mut board = Board::new(n);
    if board.solve() {
        println!("Solved for n={n}\n{}", board);
    } else {
        eprintln!("Could not solve for {n}");
        process::exit(1);
    }
}

#[derive(Debug, Clone)]
struct Board {
    n: usize,
    vals: Vec<bool>,
}

impl Board {
    fn new(n: usize) -> Self {
        Self {
            n: n,
            vals: vec![false; n * n],
        }
    }
    // attempt to solve the board. we must place N pieces on the board.
    fn solve(&mut self) -> bool {
        self.solve_n(self.n)
    }
    // solve for remain queens
    fn solve_n(&mut self, remain: usize) -> bool {
        if remain == 0 {
            return true;
        }
        for col in 0..self.n {
            for row in 0..self.n {
                let occupied = self.at(row, col);
                if occupied {
                    continue;
                }
                // the the piece
                self.set(row, col, true);

                // check if valid
                let valid = self.valid();

                // if not valid, unset and continue
                if !valid {
                    self.set(row, col, false);
                    continue;
                }

                // if valid, recurse
                let success = self.solve_n(remain - 1);
                if success {
                    return true;
                }

                // didn't work, unset and continue
                self.set(row, col, false);
            }
        }
        // if we got to the end, we failed
        false
    }
    // ensures that all placed pieces are in a valid position
    fn valid(&self) -> bool {
        for col in 0..self.n {
            for row in 0..self.n {
                if self.at(row, col) {
                    if !self.check(row, col) {
                        return false;
                    }
                }
            }
        }
        true
    }
    // checks that the piece at (row,col) cannot reach any other pieces
    fn check(&self, row: usize, col: usize) -> bool {
        // sanity check
        assert_eq!(true, self.at(row, col));

        for delta_x in -1..=1 {
            for delta_y in -1..=1 {
                if delta_x == 0 && delta_y == 0 {
                    continue;
                }
                if !self.check_deltas(row, col, delta_x, delta_y) {
                    return false;
                }
            }
        }
        true
    }
    fn check_deltas(&self, row: usize, col: usize, row_delta: i32, col_delta: i32) -> bool {
        let mut row = row as i32;
        let mut col = col as i32;
        loop {
            row += row_delta;
            col += col_delta;
            if self.oob(row) || self.oob(col) {
                break;
            }
            if self.at(row as usize, col as usize) {
                return false;
            }
        }
        true
    }
    fn oob(&self, dim: i32) -> bool {
        dim < 0 || dim >= self.n.try_into().unwrap()
    }
    fn set(&mut self, row: usize, col: usize, val: bool) {
        self.vals[row * self.n + col] = val
    }
    fn at(&self, row: usize, col: usize) -> bool {
        self.vals[row * self.n + col]
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for i in 0..self.n {
            for j in 0..self.n {
                let idx = i * self.n + j;
                if self.vals[idx] {
                    s.push_str("Q ");
                } else {
                    s.push_str("_ ");
                }
            }
            s.push_str("\n");
        }
        write!(f, "{}", s)
    }
}