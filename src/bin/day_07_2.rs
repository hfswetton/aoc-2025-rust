use std::cmp::PartialEq;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use aoc_2025_rust::coord_grid::Grid;

const OUTPUT_MESSAGE: &str = "Total";

const INPUT_FILE: &str = "input/day_07.txt";
const DIAGRAM_WIDTH: usize = 141;  // example: 15, real input: 141
const DIAGRAM_HEIGHT: usize = 142;  // example: 16, real input: 142

#[derive(Clone, Copy, Debug, Default, PartialEq)]
enum ManifoldLayoutCell {
    #[default]
    Empty,
    Source,
    Splitter,
}

type ManifoldLayoutDiagram = Grid<DIAGRAM_WIDTH, DIAGRAM_HEIGHT, ManifoldLayoutCell>;

fn parse_manifold_layout(lines: Vec<String>) -> (ManifoldLayoutDiagram, usize) {
    let mut layout: ManifoldLayoutDiagram = Grid::create();
    let mut source_j = 0;
    lines.into_iter().enumerate().for_each(|(i, line)| {
        line.chars().enumerate().for_each(|(j, char)| layout.set((i, j), match char {
            '.' => ManifoldLayoutCell::Empty,
            'S' => {
                source_j = j;
                ManifoldLayoutCell::Source
            },
            '^' => ManifoldLayoutCell::Splitter,
            _ => panic!("invalid character in map"),
        }).expect("tried to set invalid coords"));
    });
    (layout, source_j)
}

fn count_multiverses(manifold_layout: &ManifoldLayoutDiagram, source_col: &usize) -> u64 {
    let mut splitter_layout = [[false; DIAGRAM_WIDTH]; DIAGRAM_HEIGHT];
    manifold_layout.iter_coords().for_each(|(i, j)| if let Ok(ManifoldLayoutCell::Splitter) = manifold_layout.get((i, j)) { splitter_layout[i][j] = true });
    let mut cache: [[Option<u64>; DIAGRAM_WIDTH]; DIAGRAM_HEIGHT] = [[None; DIAGRAM_WIDTH]; DIAGRAM_HEIGHT];
    count_multiverses_inner(&splitter_layout, 0, *source_col, &mut cache)
}

/// Determine the number of multiverses from a given beam position,
/// using recursion if a splitter is encountered
/// and tracking previously visited positions using a cache
/// (the latter reduces the runtime from >minutes to <1s).
fn count_multiverses_inner(splitter_layout: &[[bool; DIAGRAM_WIDTH]; DIAGRAM_HEIGHT], start_row: usize, col: usize, cache: &mut [[Option<u64>; DIAGRAM_WIDTH]; DIAGRAM_HEIGHT]) -> u64 {
    let mut result = 1;
    let mut row = start_row;
    while row < DIAGRAM_HEIGHT {
        if let Some(value) = cache[row][col] {
            result = value;
            row += 1;  // needed for setting last cache row
            break;
        } else if splitter_layout[row][col] {
            result = count_multiverses_inner(splitter_layout, row, col - 1, cache)
                + count_multiverses_inner(splitter_layout, row, col + 1, cache);
            row += 1;  // needed for setting last cache row
            break;
        }
        row += 1;
    }
    (start_row..row).for_each(|r| cache[r][col] = Some(result));
    result
}

fn print_diagram(manifold_layout_diagram: &ManifoldLayoutDiagram) {
    let sep_line: String = ['-'; DIAGRAM_WIDTH + 2].into_iter().collect();
    println!("{sep_line}");
    manifold_layout_diagram.iter_rows().for_each(|row| {
        let line: String = row.iter().map(|v| match v {
            ManifoldLayoutCell::Empty => ' ',
            ManifoldLayoutCell::Splitter => '^',
            ManifoldLayoutCell::Source => '0',
        }).collect();
        println!("|{line}|");
    });
    println!("{sep_line}");
}

fn calculate_result(lines: Lines<BufReader<File>>) -> Result<u64, ()> {
    let (manifold_layout, source_col) = parse_manifold_layout(lines.map(|l| l.expect("invalid lines")).collect());
    print_diagram(&manifold_layout);
    let n_multiverses = count_multiverses(&manifold_layout, &source_col);
    Ok(n_multiverses)
}

fn main() {
    let file = File::open(INPUT_FILE).expect("unable to open file");
    let reader = BufReader::new(file);
    let result = calculate_result(reader.lines()).expect("error calculating result");
    println!("{OUTPUT_MESSAGE}: {result}");
}
