use wasm_bindgen::prelude::*;
use sudoko_solver::{Board, Solver, SolverErrors};

#[wasm_bindgen]
pub enum RollError {
    InvalidBoardLength = "INVALID_BOARD_LENGTH",
    UnsolvableBoard = "UNSOLVABLE_BOARD",
}

#[wasm_bindgen]
pub fn solve(board: Vec<i32>) -> Result<Vec<i32>, RollError> {
    if board.len() != 81 {
        return Err(RollError::InvalidBoardLength)
    }

    let mut solver_board: Board = core::array::from_fn(|_| {
        return core::array::from_fn(|_| None)
    });
    for (pos, item) in board.iter().enumerate() {
        if *item != 0 {
            solver_board[pos / 9][pos % 9] = Some(*item);
        }
    }

    let solver = Solver { board: solver_board };
    let solver_result = match solver.solve() {
        Ok(val) => val,
        Err(e) => match e {
            SolverErrors::UnsolvableBoardError => {
                return Err(RollError::UnsolvableBoard);
            }
        }
    };

    let mut result = Vec::<i32>::new();
    for row in solver_result {
        for col in row {
            result.push(col);
        }
    }

    return Ok(result);
}
