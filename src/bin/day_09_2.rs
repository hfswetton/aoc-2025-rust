use std::cmp::{min, max};
use std::error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Lines};
use std::io::ErrorKind::InvalidInput;
use itertools::Itertools;

const OUTPUT_MESSAGE: &str = "Largest area";
const INPUT_FILE: &str = "input/day_09.txt";

type CoordValue = usize;

#[derive(Debug, PartialEq, PartialOrd)]
struct RedTile(CoordValue, CoordValue);

impl RedTile {
    fn from_string(input: String) -> Result<Self, Box<dyn error::Error>> {
        let parts: Vec<&str> = input.split(',').collect();
        if parts.len() != 2 { return Err(io::Error::new(InvalidInput, "invalid number of coordinates").into()); }
        Ok(RedTile(
            parts[0].parse::<CoordValue>()?,
            parts[1].parse::<CoordValue>()?,
        ))
    }
}

#[derive(Debug)]
struct Rectangle {
    left_x: CoordValue,
    right_x: CoordValue,
    bottom_y: CoordValue,
    top_y: CoordValue,
}

impl Rectangle {
    fn new(tile_1: &RedTile, tile_2: &RedTile) -> Self {
        Self {
            left_x: min(tile_1.0, tile_2.0),
            right_x: max(tile_1.0, tile_2.0),
            bottom_y: min(tile_1.1, tile_2.1),
            top_y: max(tile_1.1, tile_2.1),
        }
    }

    fn contains(&self, x: &usize, y: &usize) -> bool {
        *x >= self.left_x
            && *x <= self.right_x
            && *y >= self.bottom_y
            && *y <= self.top_y
    }

    fn contains_in_perimeter(&self, x: &usize, y: &usize) -> bool {
        ((*y == self.bottom_y || *y == self.top_y) && (*x >= self.left_x && *x <= self.right_x))
            || ((*x == self.left_x || *x == self.right_x) && (*y >= self.bottom_y && *y <= self.top_y))
    }

    fn contains_not_in_perimeter(&self, x: &usize, y: &usize) -> bool {
        *x > self.left_x
            && *x < self.right_x
            && *y > self.bottom_y
            && *y < self.top_y
    }

    /// Check the validity of a single rectangle
    ///
    /// **Idea:** A rectangle is invalid iff it contains a connection between
    /// two red tiles, except in its perimeter.
    fn is_valid(&self, red_tile_connections: &Vec<(usize, usize)>) -> bool {
        ! red_tile_connections.iter().any(|(x, y)| self.contains_not_in_perimeter(x, y))
    }

    fn area(&self) -> CoordValue {
        (self.right_x - self.left_x + 1) * (self.top_y - self.bottom_y + 1)
    }
}

fn parse_tiles(lines: Lines<BufReader<File>>) -> Vec<RedTile> {
    lines.map(|line| RedTile::from_string(line.expect("invalid line")).expect("invalid input")).collect()
}

fn find_red_tile_connections(tile_list: &Vec<RedTile>) -> Vec<(usize, usize)> {
    let mut red_tile_coords: Vec<(usize, usize)> = tile_list.iter().map(|tile| (tile.0, tile.1)).collect();
    let mut connection_coords = red_tile_coords.clone();
    let n_red_tiles = red_tile_coords.len();
    red_tile_coords.push(red_tile_coords[0]);
    (0..n_red_tiles).for_each(|i| {
        let (tile_1_x, tile_1_y) = red_tile_coords[i];
        let (tile_2_x, tile_2_y) = red_tile_coords[i + 1];
        if tile_1_x == tile_2_x {
            let range_start = min(tile_1_y, tile_2_y);
            let range_end = max(tile_1_y, tile_2_y);
            let y_range = (range_start + 1)..=(range_end - 1);
            y_range.for_each(|y| connection_coords.push((tile_1_x, y)));
        } else if tile_1_y == tile_2_y {
            let range_start = min(tile_1_x, tile_2_x);
            let range_end = max(tile_1_x, tile_2_x);
            let x_range = (range_start + 1)..=(range_end - 1);
            x_range.for_each(|x| connection_coords.push((x, tile_1_y)));
        } else {
            panic!("misaligned tiles");
        }
    });
    connection_coords
}

fn calculate_result(lines: Lines<BufReader<File>>) -> Result<CoordValue, ()> {
    let tile_list = parse_tiles(lines);
    println!("{} red tiles parsed", tile_list.len());
    let red_tile_connections = find_red_tile_connections(&tile_list);
    println!("{} connecting tiles found", red_tile_connections.len());
    let result = tile_list.iter().tuple_combinations().filter_map(|(a, b)| {
        let rect = Rectangle::new(a, b);
        if rect.is_valid(&red_tile_connections) {
            let area = rect.area();
            //println!("{a:?} - {b:?} -> {area:?}");
            Some(area)
        } else {
            //println!("{a:?} - {b:?} invalid");
            None
        }
    }).max();
    Ok(result.expect("no result returned"))
}

fn main() {
    let file = File::open(INPUT_FILE).expect("unable to open file");
    let reader = BufReader::new(file);
    let result = calculate_result(reader.lines()).expect("error calculating result");
    println!("{OUTPUT_MESSAGE}: {result}");
}
