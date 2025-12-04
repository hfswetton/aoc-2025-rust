use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use strum::IntoEnumIterator;
use aoc_2025_rust::coord_grid::{Direction, file_lines_to_char_grid, Grid};

const OUTPUT_MESSAGE: &str = "Total";
const INPUT_FILE: &str = "input/day_04.txt";
const GRID_SIZE: usize = 140;  // example: 10; real input: 140

fn count_neighbouring_rolls(x: usize, y: usize, grid: &Grid<GRID_SIZE, GRID_SIZE, char>) -> usize {
    Direction::iter().filter(|direction| {
        if let Ok((x2, y2)) = grid.move_coords((x, y), *direction) {
            if let Ok(c) = grid.get((x2, y2)) {
                if c == '@' { return true }
            }
        }
        false
    }).count()
}

fn calculate_result(lines: Lines<BufReader<File>>) -> Result<usize, ()> {
    let grid: Grid<GRID_SIZE, GRID_SIZE, char> = file_lines_to_char_grid(lines).expect("unable to parse grid");
    let mut accessible_grid: Grid<GRID_SIZE, GRID_SIZE, bool> = Grid::create();
    accessible_grid.set_all(false);
    let total: usize = grid.iter_coords().filter(|(x, y)| {
        if let Ok(c) = grid.get((*x, *y)) {
            c == '@' && count_neighbouring_rolls(*x, *y, &grid) < 4
        } else { false }
    }).count();
    // grid.iter_coords().for_each(|(x, y)| if count_neighbouring_rolls(x, y, &grid) < 4 { accessible_grid.set((x, y), true); });
    // println!("{accessible_grid:?}");
    Ok(total)
}

fn main() {
    let file = File::open(INPUT_FILE).expect("unable to open file");
    let reader = BufReader::new(file);
    let result = calculate_result(reader.lines()).expect("error calculating result");
    println!("{OUTPUT_MESSAGE}: {result}");
}
