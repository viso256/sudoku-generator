use std::time;

use sudoku::{
    bitset::BitSet,
    error::SudokuError,
    generate_puzzle,
    generator::SudokuGenerator,
    solver::Solve,
    sudoku::{Puzzle, Sudoku},
};

fn main() {
    println!("Hello, world!");

    let start = time::Instant::now();

    let mut sudokus: Vec<Puzzle> = Vec::new();
    let repetitions = 12;
    for _ in 0..repetitions {
        let sudoku = generate_puzzle();
        sudokus.push(sudoku);
    }

    let duration = time::Instant::now() - start;
    println!("{:?}", duration / repetitions);

    let writer = std::fs::File::create("out.json").unwrap();
    serde_json::to_writer_pretty(writer, &sudokus).unwrap();
}
