use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::time::Instant;
use aoc_2025_rust::coord_grid::Grid;

const OUTPUT_MESSAGE: &str = "Total";
const INPUT_FILE: &str = "input/day_12.txt";

const PRESENT_SIZE: usize = 3;
const N_PRESENT_TYPES: usize = 6;
const MAX_TREE_SIZE: usize = 50;

type Present = Grid<PRESENT_SIZE, PRESENT_SIZE, bool>;

/// Represents space below a tree as boolean values:
/// `false` = no present, `true` = present.
/// Uses a `Grid` to keep the space on the stack
/// (the maximum tree size is known in advance,
/// therefore the difference is padded with `true`).
#[derive(Debug, Clone)]
struct Tree {
    shape: (usize, usize),
    grid: Grid<MAX_TREE_SIZE, MAX_TREE_SIZE, bool>,
    presents: [usize; N_PRESENT_TYPES],
}

impl Tree {
    fn new(size_i: usize, size_j: usize) -> Self {
        let mut grid = Grid::create();
        grid.set_all(false);
        (size_i..MAX_TREE_SIZE).for_each(|i| {
            (0..MAX_TREE_SIZE).for_each(|j| {
                grid.set((i, j), true).unwrap();
            });
        });
        (0..MAX_TREE_SIZE).for_each(|i| {
            (size_j..MAX_TREE_SIZE).for_each(|j| {
                grid.set((i, j), true).unwrap();
            });
        });
        Self { shape: (size_i, size_j), grid, presents: [0; N_PRESENT_TYPES] }
    }

    #[allow(dead_code)]
    fn print(&self) {
        let sep_line: String = ['-'; MAX_TREE_SIZE + 2].into_iter().collect();
        println!("{sep_line}");
        self.grid.iter_rows().for_each(|row| {
            let line: String = row.iter().map(|v| if *v { '#' } else { ' ' }).collect();
            println!("|{line}|");
        });
        println!("{sep_line}");
    }

    /// Return a clone of `self` with the specified `Present`
    /// with the top-left corner at `coords` and rotated `n_rot` times,
    /// if the required space is free.
    fn with_present(&self, coords: (usize, usize), n_rot: usize, flipped: bool, present: &Present) -> Result<Self, ()> {
        let (i0, j0) = coords;
        let mut new_grid = self.grid.clone();
        if present.iter_values_with_coords().all(|((i, j), v)| {
            if v {
                let (i_new, j_new) = match (n_rot % 4, flipped) {
                    (0, false) => (i0 + i, j0 + j),
                    (1, false) => (i0 + j, j0 + PRESENT_SIZE - i - 1),
                    (2, false) => (i0 + PRESENT_SIZE - i - 1, j0 + PRESENT_SIZE - j - 1),
                    (3, false) => (i0 + PRESENT_SIZE - j - 1, j0 + i),
                    (0, true) => (i0 + PRESENT_SIZE - i - 1, j0 + j),
                    (1, true) => (i0 + PRESENT_SIZE - j - 1, j0 + PRESENT_SIZE - i - 1),
                    (2, true) => (i0 + i, j0 + PRESENT_SIZE - j - 1),
                    (3, true) => (i0 + j, j0 + i),
                    _ => unreachable!(),
                };
                match new_grid.get((i_new, j_new)) {
                    Err(_) => Err(()),      // out of bounds
                    Ok(true) => Err(()),    // present collision
                    Ok(false) => {          // correct placement
                        new_grid.set((i_new, j_new), true).unwrap();
                        Ok(())
                    },
                }
            } else {
                Ok(())  // no present -> grid content irrelevant
            }
        }.is_ok()) {
            let mut new_tree = self.clone();
            new_tree.grid = new_grid;
            Ok(new_tree)
        } else {
            Err(())
        }
    }

    /// Iterate over the coordinates with free space,
    /// with optional bottom-right padding.
    fn iter_free_coords(&self, padding_br: usize) -> impl Iterator<Item=(usize, usize)> {
        (0..(MAX_TREE_SIZE - padding_br)).flat_map(move |i| (0..(MAX_TREE_SIZE - padding_br)).filter_map(move |j| {
            if !self.grid.get((i, j)).unwrap() {
                Some((i.clone(), j))
            } else {
                None
            }
        }))
    }

    /// Check whether a tree's presents can be positioned under it.
    fn is_valid(&self, present_types: &[Present; N_PRESENT_TYPES]) -> bool {
        let mut presents: Vec<Present> = Vec::new();
        self.presents.iter().enumerate().for_each(|(i, n)| {
            (0..*n).for_each(|_| presents.push(present_types[i].clone()))
        });
        println!("Assigning {} presents to a tree of size {}x{}...", presents.len(), self.shape.0, self.shape.1);
        //self.print();
        let free_spaces: usize = self.grid.iter().map(|v| if !v { 1 } else { 0 }).sum();
        let required_spaces: usize = presents.iter().map(|present| present.iter().map(|v| if *v { 1 } else { 0 }).sum::<usize>()).sum();
        if free_spaces < required_spaces {
            println!("Assignment requires more free spaces than are available.");
            false
        } else {
            self.is_valid_inner(&presents, 0)
        }
    }

    /// Recursive function to find a valid present placement
    fn is_valid_inner(&self, presents: &Vec<Present>, present_idx: usize) -> bool {
        if present_idx >= presents.len() {
            // End recursion
            println!("Assigned presents successfully:");
            //self.print();
            return true
        }

        self.iter_free_coords(2).any(|coords| {
            (0..=3).any(|n_rot| {
                [true, false].iter().any(|flipped| {
                    match self.with_present(coords, n_rot, *flipped, &(presents[present_idx])) {
                        Ok(new_tree) => new_tree.is_valid_inner(presents, present_idx + 1),
                        Err(_) => false,
                    }
                })
            })
        })
    }
}

fn parse_input(lines_res: Lines<BufReader<File>>) -> ([Present; N_PRESENT_TYPES], Vec<Tree>) {
    let mut lines = lines_res.map(|line| line.expect("invalid line"));
    let mut presents: [Present; 6] = (0..N_PRESENT_TYPES).map(|_| Present::create()).collect::<Vec<Present>>().try_into().unwrap();
    let mut trees = Vec::new();

    let mut present_idx: usize = 0;
    while present_idx < N_PRESENT_TYPES {
        lines.next();
        lines.by_ref().take(3).enumerate().for_each(|(i, line)| {
            line.chars().enumerate().for_each(|(j, char)| {
                if char == '#' { presents[present_idx].set((i, j), true).expect("invalid line length"); }
            });
        });
        if lines.next().expect("input incomplete").trim() != "" {
            panic!("invalid input; lines missing between presents");
        }
        present_idx += 1;
    }

    lines.for_each(|line| {
        let line_parts: Vec<&str> = line.split(':').collect();
        let line_dims: [usize; 2] = line_parts[0].split('x').map(|n| n.parse::<usize>().expect("invalid dimension specifier")).collect::<Vec<usize>>().try_into().expect("incorrect number of dimensions");
        let present_nums: [usize; N_PRESENT_TYPES] = line_parts[1].trim().split(' ').map(|n| n.parse::<usize>().expect("invalid number specifier")).collect::<Vec<usize>>().try_into().expect("incorrect number of present specifiers");
        let mut tree = Tree::new(line_dims[0], line_dims[1]);
        tree.presents = present_nums;
        trees.push(tree);
    });

    (presents, trees)
}

fn calculate_result(lines: Lines<BufReader<File>>) -> Result<usize, ()> {
    let (presents, trees) = parse_input(lines);
    println!("Parsed {} present types and {} trees.", presents.len(), trees.len());
    let result = trees.iter().filter(|tree| tree.is_valid(&presents)).count();
    Ok(result)
}

fn main() {
    let file = File::open(INPUT_FILE).expect("unable to open file");
    let reader = BufReader::new(file);
    let now = Instant::now();
    let result = calculate_result(reader.lines()).expect("error calculating result");
    let elapsed_time = now.elapsed();
    println!("{OUTPUT_MESSAGE}: {result}");
    println!("Result calculated in {} ms", elapsed_time.as_millis());
}
