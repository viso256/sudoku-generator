use crate::{
    bitset::{BitSet, Number},
    error::SudokuError,
};

pub trait IterCells {
    type Cell;
    fn iter_cells(&self) -> impl Iterator<Item = (&Self::Cell, usize, usize)>;
    fn iter_cells_mut(&mut self) -> impl Iterator<Item = (&mut Self::Cell, usize, usize)>;
}

impl<T, const M: usize, const N: usize> IterCells for [[T; M]; N] {
    type Cell = T;

    fn iter_cells(&self) -> impl Iterator<Item = (&Self::Cell, usize, usize)> {
        self.iter().enumerate().flat_map(|(row_i, row)| {
            row.iter()
                .enumerate()
                .map(move |(col_i, cell)| (cell, col_i, row_i))
        })
    }

    fn iter_cells_mut(&mut self) -> impl Iterator<Item = (&mut Self::Cell, usize, usize)> {
        self.iter_mut().enumerate().flat_map(|(row_i, row)| {
            row.iter_mut()
                .enumerate()
                .map(move |(col_i, cell)| (cell, col_i, row_i))
        })
    }
}

pub type SudokuCell = Option<Number>;
pub type SudokuCells = [[SudokuCell; 9]; 9];
pub const EMPTY_CELLS: SudokuCells = [[None; 9]; 9];

#[derive(Debug, Default, Clone)]
pub struct SudokuGenerator {
    pub(crate) cells: SudokuCells, // [rows][cols]
    rows: [BitSet; 9],
    cols: [BitSet; 9],
    boxes: [BitSet; 9],
}

impl SudokuGenerator {
    pub fn check(&self) -> bool {
        let mut seen = BitSet::new();
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
    ) -> Result<(), SudokuError> {
        if row_i >= 9 || col_i >= 9 {
            return Err(SudokuError::InvalidCell);
        }
        if (self.get_possible_for_cell(row_i, col_i)).get(n) {
            return Err(SudokuError::InvalidValue);
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

    pub fn cells_mut(&mut self) -> impl Iterator<Item = (&mut Option<Number>, usize, usize)> {
        self.cells.iter_cells_mut()
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
        let options = self.get_possible();
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
                Err(SudokuError::InvalidValue) => {
                    self.clear_cell(row_i, col_i);
                }
                Err(e) => panic!("{:?}", e),
            }
        }
    }

    pub fn count_non_empty(&self) -> usize {
        self.cells.iter().flatten().filter(|c| c.is_some()).count()
    }

    pub fn get_possible_for_cell(&self, row_i: usize, col_i: usize) -> BitSet {
        self.rows[row_i] | self.cols[col_i] | self.boxes[Self::get_box_i(row_i, col_i)]
    }

    pub fn get_possible(&self) -> [[BitSet; 9]; 9] {
        let mut possible = [[BitSet::new(); 9]; 9];
        for (row_i, row) in possible.iter_mut().enumerate() {
            for (col_i, cell) in row.iter_mut().enumerate() {
                *cell = self.get_possible_for_cell(row_i, col_i);
            }
        }
        possible
    }
}
