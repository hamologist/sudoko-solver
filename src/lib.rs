use std::collections::HashSet;
use std::sync::LazyLock;

static ALL_POSSIBLE_MOVES: LazyLock<HashSet<i32>> =
    LazyLock::new(|| HashSet::from([1, 2, 3, 4, 5, 6, 7, 8, 9]));

pub type Board = [[Option<i32>; 9]; 9];
pub type SolvedBoard = [[i32; 9]; 9];

#[derive(Debug)]
enum PossibleMovesCell {
    Solved(i32),
    Unknown(Vec<i32>),
}

type PossibleMoves = [[PossibleMovesCell; 9]; 9];

type CheckerCollection = [HashSet<i32>; 9];

struct PossibleMovesBuilder<'a> {
    board: &'a Board,
    row_checkers: CheckerCollection,
    col_checkers: CheckerCollection,
    grid_checkers: CheckerCollection,
}

impl<'a> PossibleMovesBuilder<'a> {
    fn from_board(board: &'a Board) -> Self {
        let mut row_checkers: CheckerCollection = core::array::from_fn(|_| HashSet::new());
        let mut col_checkers: CheckerCollection = core::array::from_fn(|_| HashSet::new());
        let mut grid_checkers: CheckerCollection = core::array::from_fn(|_| HashSet::new());
        for (row_index, row) in board.iter().enumerate() {
            for (col_index, cell) in row.iter().enumerate() {
                match cell {
                    Some(val) => {
                        row_checkers[row_index].insert(*val);
                        col_checkers[col_index].insert(*val);
                        grid_checkers[(3 * (row_index / 3)) + (col_index / 3)].insert(*val);
                    }
                    None => {
                        continue;
                    }
                }
            }
        }

        return PossibleMovesBuilder {
            board,
            row_checkers,
            col_checkers,
            grid_checkers,
        };
    }

    fn build(&self) -> PossibleMoves {
        core::array::from_fn(|row_index| {
            core::array::from_fn(|col_index| match self.board[row_index][col_index] {
                Some(val) => PossibleMovesCell::Solved(val),
                None => {
                    let valid_moves: Vec<i32> = ALL_POSSIBLE_MOVES
                        .difference(&self.row_checkers[row_index])
                        .map(|v| *v)
                        .collect::<HashSet<i32>>()
                        .difference(&self.col_checkers[col_index])
                        .map(|v| *v)
                        .collect::<HashSet<i32>>()
                        .difference(&self.grid_checkers[(3 * (row_index / 3)) + (col_index / 3)])
                        .map(|v| *v)
                        .collect();

                    if valid_moves.len() == 1 {
                        return PossibleMovesCell::Solved(valid_moves[0]);
                    }
                    return PossibleMovesCell::Unknown(valid_moves);
                }
            })
        })
    }
}

pub struct Solver {
    pub board: Board,
}

#[derive(Debug)]
pub enum SolverErrors {
    UnsolvableBoardError,
}

fn is_board_solved(solved_board: &SolvedBoard) -> Result<(), (usize, usize)> {
    let mut checker: HashSet<i32> = HashSet::new();

    for (row_index, row) in solved_board.iter().enumerate() {
        checker.clear();
        for (col_index, val) in row.iter().enumerate() {
            if checker.contains(val) {
                return Err((row_index, col_index));
            }
            checker.insert(*val);
        }
    }

    for col_index in 0..9 {
        checker.clear();
        for row_index in 0..9 {
            if checker.contains(&solved_board[row_index][col_index]) {
                return Err((row_index, col_index));
            }
            checker.insert(solved_board[row_index][col_index]);
        }
    }

    for block_row_index in 0..3 {
        for block_col_index in 0..3 {
            checker.clear();
            for cell_row_index in 0..3 {
                for cell_col_index in 0..3 {
                    let row_index = block_row_index * 3 + cell_row_index;
                    let col_index = block_col_index * 3 + cell_col_index;
                    if checker.contains(&solved_board[row_index][col_index]) {
                        return Err((row_index, col_index));
                    }
                    checker.insert(solved_board[row_index][col_index]);
                }
            }
        }
    }

    return Ok(());
}

enum InProgressErrors {
    Proceed,
    UnsolvableBoardError,
    CollisionError((usize, usize)),
}

impl Solver {
    fn _solve(&self, possible_moves: &PossibleMoves) -> Option<SolvedBoard> {
        struct Context<'a> {
            possible_moves: &'a PossibleMoves,
            result: &'a mut SolvedBoard,
        }

        fn inner(
            ctx: &mut Context,
            row_index: usize,
            col_index: usize,
        ) -> Result<SolvedBoard, InProgressErrors> {
            if row_index >= 9 {
                match is_board_solved(ctx.result) {
                    Ok(_) => return Ok(*ctx.result),
                    Err(indexes) => return Err(InProgressErrors::CollisionError(indexes)),
                };
            }
            if col_index >= 9 {
                return inner(ctx, row_index + 1, 0);
            }

            let cell_moves = match &ctx.possible_moves[row_index][col_index] {
                PossibleMovesCell::Unknown(cell_moves) => cell_moves,
                PossibleMovesCell::Solved(val) => {
                    ctx.result[row_index][col_index] = *val;
                    return inner(ctx, row_index, col_index + 1);
                }
            };
            if cell_moves.len() == 0 {
                return Err(InProgressErrors::UnsolvableBoardError);
            }

            for cell_move in cell_moves {
                ctx.result[row_index][col_index] = *cell_move;
                match inner(ctx, row_index, col_index + 1) {
                    Ok(solved) => return Ok(solved),
                    Err(err) => match err {
                        InProgressErrors::Proceed => {}
                        InProgressErrors::UnsolvableBoardError => {
                            return Err(InProgressErrors::UnsolvableBoardError);
                        }
                        InProgressErrors::CollisionError((
                            collision_row_index,
                            collision_col_index,
                        )) => {
                            if row_index > collision_row_index {
                                return Err(InProgressErrors::CollisionError((
                                    collision_row_index,
                                    collision_col_index,
                                )));
                            } else if col_index > collision_col_index {
                                return Err(InProgressErrors::CollisionError((
                                    collision_row_index,
                                    collision_col_index,
                                )));
                            }
                        }
                    },
                }
            }

            return Err(InProgressErrors::Proceed);
        }

        let mut result: SolvedBoard = [[0; 9]; 9];

        return match inner(
            &mut Context {
                possible_moves: &possible_moves,
                result: &mut result,
            },
            0,
            0,
        ) {
            Ok(val) => Some(val),
            Err(_) => None,
        };
    }

    pub fn solve(&self) -> Result<SolvedBoard, SolverErrors> {
        let possible_moves = PossibleMovesBuilder::from_board(&self.board).build();

        return match self._solve(&possible_moves) {
            Some(result) => Ok(result),
            None => Err(SolverErrors::UnsolvableBoardError),
        };
    }
}
