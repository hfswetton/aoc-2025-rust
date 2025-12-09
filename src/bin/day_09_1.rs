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

    fn connected_area(&self, other: &Self) -> CoordValue {
        (self.0.abs_diff(other.0) + 1) * (self.1.abs_diff(other.1) + 1)
    }
}

fn parse_tiles(lines: Lines<BufReader<File>>) -> Vec<RedTile> {
    lines.map(|line| RedTile::from_string(line.expect("invalid line")).expect("invalid input")).collect()
}

fn calculate_result(lines: Lines<BufReader<File>>) -> Result<CoordValue, ()> {
    let tile_list = parse_tiles(lines);
    let result = tile_list.iter().tuple_combinations().map(|(a, b)| {
        let area = a.connected_area(b);
        //println!("{a:?} - {b:?} -> {area}");
        area
    }).max();
    Ok(result.expect("no result returned"))
}

fn main() {
    let file = File::open(INPUT_FILE).expect("unable to open file");
    let reader = BufReader::new(file);
    let result = calculate_result(reader.lines()).expect("error calculating result");
    println!("{OUTPUT_MESSAGE}: {result}");
}
