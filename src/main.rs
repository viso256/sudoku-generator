use std::{
    fmt::{Debug, Display},
    ops::Deref,
    time,
};

use rand::seq::IndexedRandom;
use serde::{Deserialize, Serialize};

fn main() {
    println!("Hello, world!");

    let start = time::Instant::now();

    let mut sudokus: Vec<Puzzle> = Vec::new();
    let repetitions = 12;
    for _ in 0..repetitions {
        let mut count = 0;
        let sudoku = loop {
            let mut sudoku = SudokuGenerator::default();

            let mut rng = rand::rng();

            let mut tried = [[Seen::new(); 9]; 9];
            let mut index = 0;
            'cells: loop {
                let row_i = index / 9;
                let col_i = index % 9;
                'value: loop {
                    let seen = &mut tried[row_i][col_i];
                    let Some(chosen) = seen.choose_new(&mut rng) else {
                        seen.clear();
                        sudoku.clear_cell(row_i, col_i);
                        index = index.checked_sub(1).unwrap_or_default();
                        continue 'cells;
                    };
                    seen.set(chosen);
                    match sudoku.try_set_cell(row_i, col_i, chosen) {
                        Ok(()) => break 'value,
                        Err(SudokuResult::InvalidValue) => continue 'value,
                        Err(e) => panic!("{:?}", e),
                    }
                }
                index += 1;
                if index >= 9 * 9 {
                    break 'cells;
                }
            }

            let solution: Sudoku = sudoku.clone().try_into().unwrap();

            let mut tried_rows = Seen::new();
            let mut tried_cols = [Seen::new(); 9];
            loop {
                let Some(row_i_number) = tried_rows.choose_new(&mut rng) else {
                    break;
                };
                let row_i = u8::from(row_i_number) as usize - 1;
                let Some(col_i_number) = tried_cols[row_i].choose_new(&mut rng) else {
                    tried_rows.set(row_i_number);
                    continue;
                };
                tried_cols[row_i].set(col_i_number);
                let col_i = u8::from(col_i_number) as usize - 1;
                let mut sudoku_clone = sudoku.clone();
                sudoku_clone.clear_cell(row_i, col_i);
                count += 1;
                if sudoku_clone.clone().has_one_solution() {
                    sudoku = sudoku_clone;
                } else {
                    continue;
                }
            }
            let count = sudoku.count_non_empty();
            println!("{}", count);
            // if sudoku.count_non_empty() < 22 {
            break Puzzle {
                puzzle: sudoku.try_into().unwrap(),
                solution,
            };
            // }
        };
        // println!("{}", sudoku.to_line());
        // println!("{sudoku}");
        // println!("{count}: {}", sudoku.count_non_empty());
        sudokus.push(sudoku);
    }
    let duration = time::Instant::now() - start;
    println!("{:?}", duration / repetitions);

    let writer = std::fs::File::create("out.json").unwrap();
    serde_json::to_writer_pretty(writer, &sudokus).unwrap();
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Puzzle {
    puzzle: Sudoku,
    solution: Sudoku,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sudoku([[Option<u8>; 9]; 9]);

impl TryFrom<SudokuGenerator> for Sudoku {
    type Error = SudokuResult;

    fn try_from(value: SudokuGenerator) -> Result<Self, Self::Error> {
        let mut sudoku = Sudoku([[None; 9]; 9]);
        for (row_i, row) in value.cells.iter().enumerate() {
            for (col_i, cell) in row.iter().enumerate() {
                sudoku.0[row_i][col_i] = cell.map(u8::from);
            }
        }
        Ok(sudoku)
    }
}

#[derive(Debug)]
pub enum SudokuResult {
    InvalidValue,
    InvalidCell,
}

#[derive(Debug, Default, Clone)]
pub struct SudokuGenerator {
    cells: [[Option<Number>; 9]; 9], // [rows][cols]
    rows: [Seen; 9],
    cols: [Seen; 9],
    boxes: [Seen; 9],
}

impl SudokuGenerator {
    pub fn check(&self) -> bool {
        let mut seen = Seen::new();
        for row in self.cells {
            seen.clear();
            for cell in row {
                if let Some(n) = cell {
                    if seen.get(n) {
                        return false;
                    } else {
                        seen.set(n);
                    }
                }
            }
        }
        for col_i in 0..9 {
            seen.clear();
            for cell_i in 0..9 {
                if let Some(n) = self.cells[cell_i][col_i] {
                    if seen.get(n) {
                        return false;
                    } else {
                        seen.set(n);
                    }
                }
            }
        }
        for group_row_i in 0..3 {
            for group_col_i in 0..3 {
                seen.clear();
                for row_i in 0..3 {
                    for col_i in 0..3 {
                        if let Some(n) =
                            self.cells[group_row_i * 3 + row_i][group_col_i * 3 + col_i]
                        {
                            if seen.get(n) {
                                return false;
                            } else {
                                seen.set(n);
                            }
                        }
                    }
                }
            }
        }
        true
    }
    pub fn try_set_cell(
        &mut self,
        row_i: usize,
        col_i: usize,
        n: Number,
    ) -> Result<(), SudokuResult> {
        if row_i >= 9 || col_i >= 9 {
            return Err(SudokuResult::InvalidCell);
        }
        if self.rows[row_i].get(n)
            || self.cols[col_i].get(n)
            || self.boxes[Self::get_box_i(row_i, col_i)].get(n)
        {
            return Err(SudokuResult::InvalidValue);
        }
        self.clear_cell(row_i, col_i);
        self.rows[row_i].set(n);
        self.cols[col_i].set(n);
        self.boxes[Self::get_box_i(row_i, col_i)].set(n);
        self.cells[row_i][col_i] = Some(n);
        Ok(())
    }

    fn get_box_i(row_i: usize, col_i: usize) -> usize {
        let box_row_i = row_i / 3;
        let box_col_i = col_i / 3;
        box_row_i * 3 + box_col_i
    }

    pub fn clear_cell(&mut self, row_i: usize, col_i: usize) {
        if let Some(n) = self.cells[row_i][col_i] {
            self.rows[row_i].unset(n);
            self.cols[col_i].unset(n);
            self.boxes[Self::get_box_i(row_i, col_i)].unset(n);
            self.cells[row_i][col_i] = None;
        }
    }

    pub fn get_cell(&self, row_i: usize, col_i: usize) -> Option<Number> {
        self.cells[row_i][col_i]
    }

    pub fn has_one_solution(mut self) -> bool {
        let mut empty_indices = Vec::new();
        for index in 0..9 * 9 {
            let row_i = index / 9;
            let col_i = index % 9;
            if self.get_cell(row_i, col_i).is_none() {
                empty_indices.push(index);
            }
        }
        let options = self.get_options();
        let mut tried = options;
        let mut index = 0;
        let mut solutions = 0;
        'cells: loop {
            let Some(cell_i) = empty_indices.get(index) else {
                solutions += 1;
                if solutions > 1 {
                    return false;
                }
                index -= 1;
                continue 'cells;
            };
            let row_i = cell_i / 9;
            let col_i = cell_i % 9;
            let Some(candidate) = tried[row_i][col_i].get_next() else {
                if index == 0 {
                    return solutions == 1;
                } else {
                    tried[row_i][col_i] = options[row_i][col_i];
                    self.clear_cell(row_i, col_i);
                    index -= 1;
                    continue 'cells;
                }
            };
            tried[row_i][col_i].set(candidate);
            match self.try_set_cell(row_i, col_i, candidate) {
                Ok(()) => {
                    if index < 9 * 9 {
                        index += 1;
                    }
                }
                Err(SudokuResult::InvalidValue) => {
                    self.clear_cell(row_i, col_i);
                }
                Err(e) => panic!("{:?}", e),
            }
        }
    }

    pub fn count_non_empty(&self) -> usize {
        self.cells.iter().flatten().filter(|c| c.is_some()).count()
    }

    pub fn get_options(&self) -> [[Seen; 9]; 9] {
        let mut tried = [[Seen::new(); 9]; 9];
        for (row_i, row) in self.cells.iter().enumerate() {
            for (col_i, cell) in row.iter().enumerate() {
                if let Some(number) = cell {
                    for i in 0..9 {
                        tried[row_i][i].set(*number);
                        tried[i][col_i].set(*number);
                        tried[(row_i / 3) * 3 + i / 3][(col_i / 3) * 3 + i % 3];
                    }
                }
            }
        }
        tried
    }
}

impl Display for SudokuGenerator {
    // ─ │ ┌ ┐ └ ┘ ├ ┤ ┬ ┴ ┼
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "┌────────┬────────┬────────┐")?;
        for (row_i, row) in self.cells.iter().enumerate() {
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

impl SudokuGenerator {
    pub fn to_line(&self) -> String {
        let mut string = String::with_capacity(9 * 9);
        for row in self.cells {
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

#[derive(Debug, Clone, Copy)]
pub enum Number {
    N1 = 1,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
}

impl From<Number> for u8 {
    fn from(value: Number) -> Self {
        value as u8
    }
}

impl Number {
    pub fn to_bits(self) -> u16 {
        1 << u8::from(self)
    }
}

#[derive(Clone, Copy, Default)]
pub struct Seen(u16);

impl Debug for Seen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Seen")
            .field(&format!("{:b}", self.0))
            .finish()
    }
}

impl Seen {
    const ALL_NUMBERS: [Number; 9] = {
        use Number::*;
        [N1, N2, N3, N4, N5, N6, N7, N8, N9]
    };

    pub fn new() -> Self {
        Self(0)
    }

    pub fn clear(&mut self) {
        self.0 = 0;
    }

    pub fn get(&self, n: Number) -> bool {
        self.0 & n.to_bits() > 0
    }

    pub fn set(&mut self, n: Number) {
        self.0 |= n.to_bits();
    }

    pub fn unset(&mut self, n: Number) {
        self.0 &= !n.to_bits();
    }

    pub fn all_seen(&self) -> bool {
        use Number::*;
        self.0
            == (N1.to_bits()
                | N2.to_bits()
                | N3.to_bits()
                | N4.to_bits()
                | N5.to_bits()
                | N6.to_bits()
                | N7.to_bits()
                | N8.to_bits()
                | N9.to_bits())
    }

    pub fn choose_new<R>(&self, rng: &mut R) -> Option<Number>
    where
        R: rand::Rng + ?Sized,
    {
        use Number::*;
        let mut arr = [N1; 9];
        let mut i = 0;
        for number in Self::ALL_NUMBERS {
            if !self.get(number) {
                arr[i] = number;
                i += 1;
            }
        }
        arr[0..i].choose(rng).cloned()
    }

    pub fn get_next(&self) -> Option<Number> {
        for n in Self::ALL_NUMBERS {
            if !self.get(n) {
                return Some(n);
            }
        }
        None
    }
}
