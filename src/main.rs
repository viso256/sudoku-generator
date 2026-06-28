use std::fmt::Display;

use rand::{RngExt, seq::IndexedRandom};

fn main() {
    println!("Hello, world!");

    let mut sudoku = Sudoku::default();

    let mut rng = rand::rng();

    // TODO: backtrack
    let mut tried = [[Seen::new(); 9]; 9];
    let mut index = 0;
    'cells: loop {
        let row_i = index / 9;
        let col_i = index % 9;
        loop {
            let seen = &mut tried[row_i][col_i];
            let Some(chosen) = seen.choose_new(&mut rng) else {
                seen.clear();
                sudoku.clear_cell(row_i, col_i);
                index = index.checked_sub(1).unwrap_or_default();
                continue 'cells;
            };
            seen.set(chosen);
            sudoku.set_cell(row_i, col_i, chosen);
            if sudoku.check() {
                break;
            }
        }
        index += 1;
        if index >= 9 * 9 {
            break 'cells;
        }
    }

    println!("{sudoku}");
}

#[derive(Debug, Default)]
pub struct Sudoku {
    cells: [[Option<Number>; 9]; 9], // [rows][cols]
}

impl Sudoku {
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

    pub fn set_cell(&mut self, row_i: usize, col_i: usize, n: Number) {
        self.cells[row_i][col_i] = Some(n);
    }

    pub fn clear_cell(&mut self, row_i: usize, col_i: usize) {
        self.cells[row_i][col_i] = None;
    }
}

impl Display for Sudoku {
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
                    write!(f, "-")?;
                }
            }
            writeln!(f, "│")?;
        }
        writeln!(f, "└────────┴────────┴────────┘")?;
        Ok(())
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

#[derive(Debug, Clone, Copy)]
pub struct Seen(u16);

impl Seen {
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
        for number in [N1, N2, N3, N4, N5, N6, N7, N8, N9] {
            if !self.get(number) {
                arr[i] = number;
                i += 1;
            }
        }
        arr[0..i].choose(rng).cloned()
    }
}
