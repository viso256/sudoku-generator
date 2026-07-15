use std::time;

use sudoku::{
    bitset::BitSet,
    error::SudokuError,
    generator::SudokuGenerator,
    sudoku::{Puzzle, Sudoku},
};

fn main() {
    println!("Hello, world!");

    let start = time::Instant::now();

    let mut sudokus: Vec<Puzzle> = Vec::new();
    let repetitions = 12;
    for _ in 0..repetitions {
        let sudoku = loop {
            let mut sudoku = SudokuGenerator::default();

            let mut rng = rand::rng();

            let mut tried = [[BitSet::new(); 9]; 9];
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
                        Err(SudokuError::InvalidValue) => continue 'value,
                        Err(e) => panic!("{:?}", e),
                    }
                }
                index += 1;
                if index >= 9 * 9 {
                    break 'cells;
                }
            }

            let solution: Sudoku = sudoku.clone().into();

            let mut tried_rows = BitSet::new();
            let mut tried_cols = [BitSet::new(); 9];
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
                if sudoku_clone.clone().has_one_solution() {
                    sudoku = sudoku_clone;
                } else {
                    continue;
                }
            }
            let count = sudoku.count_non_empty();
            println!("{}", count);
            break Puzzle::new_from(sudoku.into(), solution);
        };
        sudokus.push(sudoku);
    }
    let duration = time::Instant::now() - start;
    println!("{:?}", duration / repetitions);

    let writer = std::fs::File::create("out.json").unwrap();
    serde_json::to_writer_pretty(writer, &sudokus).unwrap();
}
