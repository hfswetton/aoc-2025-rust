use std::cmp::PartialEq;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use aoc_2025_rust::coord_grid::Grid;

const OUTPUT_MESSAGE: &str = "Total";
const INPUT_FILE: &str = "input/day_07.txt";

const DIAGRAM_WIDTH: usize = 141;  // example: 15, real input: 141
const DIAGRAM_HEIGHT: usize = 142;  // example: 16, real input: 142

type ManifoldDiagram<T> = Grid<DIAGRAM_WIDTH, DIAGRAM_HEIGHT, T>;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
enum ManifoldLayoutCell {
    #[default]
    Empty,
    Source,
    Splitter,
}

type ManifoldLayoutDiagram = ManifoldDiagram<ManifoldLayoutCell>;
type BeamPathDiagram = ManifoldDiagram<bool>;

// Concept:
// 1. Parse lines into map
// 2. Find source
// 3. Propagate beam through map
// 4. Count splitters hit by beam (splitter with beam directly above)
//      -> equals number of splits

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

fn push_if_not_present<T: PartialEq>(vec: &mut Vec<T>, elem: T) {
    if !vec.contains(&elem) { vec.push(elem); }
}

fn propagate_beam(manifold_layout: &ManifoldLayoutDiagram, source_col: &usize) -> BeamPathDiagram {
    let mut beam_paths: BeamPathDiagram = Grid::create();
    let mut beam_ends: Vec<usize> = vec!(*source_col);
    for row in 0..DIAGRAM_HEIGHT {
        let mut new_beam_ends = Vec::new();
        beam_ends.iter().for_each(|&j| {
            beam_paths.set((row, j), true).expect("tried to set invalid coords");
            if let Ok(ManifoldLayoutCell::Splitter) = manifold_layout.get((row, j)) {
                push_if_not_present(&mut new_beam_ends, j - 1);
                push_if_not_present(&mut new_beam_ends, j + 1);
                beam_paths.set((row, j - 1), true).expect("tried to set invalid coords");
                beam_paths.set((row, j + 1), true).expect("tried to set invalid coords");
            } else {
                new_beam_ends.push(j);
            }
        });
        beam_ends = new_beam_ends;
    }
    beam_paths
}

fn count_splits(manifold_layout: &ManifoldLayoutDiagram, beam_paths: &BeamPathDiagram) -> usize {
    manifold_layout.iter_coords().filter(|(i, j)| {
        if let Ok(ManifoldLayoutCell::Splitter) = manifold_layout.get((*i, *j)) {
            if let Ok(beam_value) = beam_paths.get((i - 1, *j)) { beam_value } else { false }
        } else {
            false
        }
    }).count()
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

fn print_beam_paths(beam_paths: &BeamPathDiagram) {
    let sep_line: String = ['-'; DIAGRAM_WIDTH + 2].into_iter().collect();
    println!("{sep_line}");
    beam_paths.iter_rows().for_each(|row| {
        let line: String = row.iter().map(|v| if *v { '\'' } else { ' ' }).collect();
        println!("|{line}|");
    });
    println!("{sep_line}");
}

fn calculate_result(lines: Lines<BufReader<File>>) -> Result<usize, ()> {
    let (manifold_layout, source_col) = parse_manifold_layout(lines.map(|l| l.expect("invalid lines")).collect());
    print_diagram(&manifold_layout);
    let beam_paths = propagate_beam(&manifold_layout, &source_col);
    print_beam_paths(&beam_paths);
    let n_splits = count_splits(&manifold_layout, &beam_paths);
    Ok(n_splits)
}

fn main() {
    let file = File::open(INPUT_FILE).expect("unable to open file");
    let reader = BufReader::new(file);
    let result = calculate_result(reader.lines()).expect("error calculating result");
    println!("{OUTPUT_MESSAGE}: {result}");
}
