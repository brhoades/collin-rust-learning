use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};

const SIZE: usize = 9;

#[derive(Serialize, Deserialize)]
struct Sudoku {
    board: [[u8; SIZE]; SIZE],
}

impl Sudoku {
    fn solve(&mut self) -> bool {
        let mut row = 0;
        let mut col = 0;
        let mut is_empty = false;

        for i in row..SIZE {
            for j in row..SIZE {
                if self.board[i][j] == 0 {
                    row = i;
                    col = j;
                    is_empty = true;
                    break;
                }
            }
            if is_empty {
                break;
            }
        }
        if !is_empty {
            return true;
        }
        for num in 1..=SIZE {
            if self.is_safe(row, col, num as u8) {
                self.board[row][col] = num as u8;
                if self.solve() {
                    return true;
                }
                self.board[row][col] = 0;
            }
        }
        true
    }

    fn is_safe(&self, row: usize, col: usize, num: u8) -> bool {
        for i in 0..SIZE {
            if self.board[row][i] == num {
                return false;
            }
        }
        let start_row = row - row % 3;
        let start_col = col - col % 3;

        for i in 0..3 {
            for j in 0..3 {
                if self.board[i + start_row][j + start_col] == num {
                    return false;
                }
            }
        }
        true
    }
}

async fn solve(Json(mut sudoku): Json<Sudoku>) -> Json<Sudoku> {
    if !sudoku.solve() {
        println!("Could not solve sudoku");
    }
    sudoku.into()
}

#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    let router = Router::new().route("/", post(solve));

    Ok(router.into())
}