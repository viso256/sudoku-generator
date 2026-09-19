use crate::generator::{EMPTY_CELLS, IterCells, SudokuCells, SudokuGenerator};

pub struct Solver;

impl Solve for Solver {
    fn solve(self, sudoku: &mut SudokuGenerator) -> i32 {
        todo!()
    }
}

pub trait Solve: Sized {
    /// Tries to solve the puzzle iteratively and returns the number of svalues inserted
    fn solve(self, sudoku: &mut SudokuGenerator) -> i32;
}

pub struct Singles;

impl Solve for Singles {
    fn solve(self, sudoku: &mut SudokuGenerator) -> i32 {
        let mut counter = 0;
        loop {
            let mut solutions: SudokuCells = EMPTY_CELLS;
            let possible = sudoku.get_possible();
            for (solution, possible) in solutions
                .iter_mut()
                .flatten()
                .zip(possible.iter().flatten())
            {
                if possible.get_count() == 1 {
                    *solution = possible.get_next();
                }
            }
            let mut found = 0;
            for (solution, row_i, col_i) in solutions.iter_cells() {
                if let Some(solution) = solution {
                    sudoku
                        .try_set_cell(row_i, col_i, *solution)
                        .expect("solution should be possible");
                    found += 1;
                }
            }
            if found > 0 {
                counter += found;
            } else {
                break;
            }
        }
        counter
    }
}
