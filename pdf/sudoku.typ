#let sudokus = json("sudoku.json")

#let write_cell_size(n, size) = {
  text(
    font: "Liberation Sans",
    size,
    weight: 550,
    if n == none {
      " "
    } else {
      str(n)
    },
  )
}

#let write_cell(n) = write_cell_size(n, 24pt)

#let write_solution_cell(n) = write_cell_size(n, 8pt)

#let margin = (top: 24mm, bottom: 6mm, left: 6mm, right: 6mm);

#let header = table(
  columns: (auto, 1fr, auto),
  rows: 32pt,
  align: horizon,
  text(
    font: "Liberation Sans",
    16pt,
    weight: 600,
    "Ime:",
  ),
  [],
  datetime.today().display()
)

#set page(margin: margin, header: header)

#let sudoku_stroke(x, y) = {
  let (t, b, l, r) = (1pt, 1pt, 1pt, 1pt)
  if calc.rem(x, 3) == 0 {
    l = 2pt
  }
  if calc.rem(x, 3) == 2 {
    r = 2pt
  }
  if calc.rem(y, 3) == 0 {
    t = 2pt
  }
  if calc.rem(y, 3) == 2 {
    b = 2pt
  }
  (top: t, bottom: b, left: l, right: r)
}

#for sudoku in sudokus.chunks(2) {
  let puzzle = sudoku
    .map(s => s.puzzle)
    .map(
      puzzle => block(
        breakable: false,
        table(
          columns: array.range(9).map(_ => 24pt),
          rows: array.range(9).map(_ => 24pt),
          align: center + horizon,
          stroke: sudoku_stroke,
          ..puzzle.flatten().map(write_cell)
        ),
      ),
    )
  let solutions = sudoku
    .map(s => s.solution)
    .map(
      puzzle => block(
        breakable: false,
        table(
          columns: array.range(9).map(_ => 10pt),
          rows: array.range(9).map(_ => 10pt),
          align: center + horizon,
          stroke: sudoku_stroke,
          ..puzzle.flatten().map(write_solution_cell)
        ),
      ),
    )
    .map(i => rotate(180deg, i))
  table(
    stroke: 0pt,
    inset: 6pt,
    align: center + horizon,
    rows: auto,
    columns: (auto, auto, auto),
    ..puzzle,
    for solution in solutions {
      solution
    },
  )
}
