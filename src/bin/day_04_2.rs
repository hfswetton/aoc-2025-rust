use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use strum::IntoEnumIterator;
use aoc_2025_rust::coord_grid::{Direction, file_lines_to_char_grid, Grid};

const OUTPUT_MESSAGE: &str = "Total";
const INPUT_FILE: &str = "input/day_04.txt";
const GRID_SIZE: usize = 140;  // example: 10; real input: 140
type PaperRollsGrid = Grid<GRID_SIZE, GRID_SIZE, bool>;

fn count_neighbouring_rolls(x: usize, y: usize, grid: &PaperRollsGrid) -> usize {
    Direction::iter().filter(|direction| {
        if let Ok((x2, y2)) = grid.move_coords((x, y), *direction) {
            if let Ok(b) = grid.get((x2, y2)) {
                if b { return true }
            }
        }
        false
    }).count()
}

fn remove_accessible_rolls(grid: &mut PaperRollsGrid) {
    let coords: Vec<(usize, usize)> = grid.iter_coords().collect();
    coords.iter().for_each(|(i, j)| {
        if let Ok(b) = grid.get((*i, *j)) {
            if b && count_neighbouring_rolls(*i, *j, &grid) < 4 {
                grid.set((*i, *j), false).expect("unable to remove roll");
            }
        }
    });

}

fn calculate_removed_rolls(grid_start: &PaperRollsGrid, grid_current: &PaperRollsGrid) -> usize {
    let arr_start = grid_start.raw();
    let arr_current = grid_current.raw();
    grid_current.iter_coords().filter(|(i, j)| arr_start[*i][*j] && ! arr_current[*i][*j]).count()
}

fn calculate_result(lines: Lines<BufReader<File>>) -> Result<usize, ()> {
    let grid_chars: Grid<GRID_SIZE, GRID_SIZE, char> = file_lines_to_char_grid(lines).expect("unable to parse grid");
    let mut grid: PaperRollsGrid = Grid::create();
    grid.set_all(false);
    grid_chars.iter_coords().for_each(|(i, j)| if grid_chars.raw()[i][j] == '@' { grid.set((i, j), true).expect("error setting value"); });
    let grid_start = grid.clone();
    let mut total: usize = 0;
    let mut total_last: usize = 1;
    while total != total_last {
        total_last = total;
        remove_accessible_rolls(&mut grid);
        total = calculate_removed_rolls(&grid_start, &grid);
        println!("{total} paper rolls removed so far")
    }
    Ok(total)
}

fn main() {
    let file = File::open(INPUT_FILE).expect("unable to open file");
    let reader = BufReader::new(file);
    let result = calculate_result(reader.lines()).expect("error calculating result");
    println!("{OUTPUT_MESSAGE}: {result}");
}
