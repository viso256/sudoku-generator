use std::fmt::{Debug, Display};

use serde::{Deserialize, Serialize};

use crate::generator::SudokuGenerator;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Puzzle {
    puzzle: Sudoku,
    solution: Sudoku,
}

impl Puzzle {
    pub fn new_from(puzzle: Sudoku, solution: Sudoku) -> Self {
        Self { puzzle, solution }
    }

    #[cfg(feature = "serde_json")]
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("failed to generate json")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sudoku([[Option<u8>; 9]; 9]);

impl From<SudokuGenerator> for Sudoku {
    fn from(value: SudokuGenerator) -> Self {
        let mut sudoku = Sudoku([[None; 9]; 9]);
        for (row_i, row) in value.cells.iter().enumerate() {
            for (col_i, cell) in row.iter().enumerate() {
                sudoku.0[row_i][col_i] = cell.map(u8::from);
            }
        }
        sudoku
    }
}

impl Display for Sudoku {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "┌────────┬────────┬────────┐")?;
        for (row_i, row) in self.0.iter().enumerate() {
            if row_i < 9 && row_i > 0 && row_i % 3 == 0 {
                writeln!(f, "├────────┼────────┼────────┤")?;
            }
            for (col_i, cell) in row.iter().enumerate() {
                if col_i % 3 == 0 {
                    write!(f, "│")?;
                } else {
                    write!(f, " ")?;
                }
                if let Some(c) = cell {
                    write!(f, " {}", u8::from(*c))?;
                } else {
                    write!(f, "  ")?;
                }
            }
            writeln!(f, "│")?;
        }
        writeln!(f, "└────────┴────────┴────────┘")?;
        Ok(())
    }
}

impl Sudoku {
    pub fn to_line(&self) -> String {
        let mut string = String::with_capacity(9 * 9);
        for row in self.0 {
            for cell in row {
                string.push(
                    cell.map(|c| char::from_digit(u8::from(c) as u32, 10).unwrap())
                        .unwrap_or('0'),
                );
            }
        }
        string
    }
}
