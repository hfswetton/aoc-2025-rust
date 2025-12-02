use std::fs::File;
use std::io::{BufRead, BufReader, Error, Lines};

const OUTPUT_MESSAGE: &str = "Total";
const INPUT_FILE: &str = "input/day_01.txt";

const MAX_DIAL_VALUE: u32 = 99;

enum TurnDirection {
    Left,
    Right,
}

/// Process one input line
fn parse_line(line_res: Result<String, Error>) -> (TurnDirection, u32) {
    let line = line_res.expect("error reading line");
    let (direction_str, clicks_str) = line.split_at(1);
    let direction = match direction_str {
        "L" => TurnDirection::Left,
        "R" => TurnDirection::Right,
        _ => panic!("invalid input"),
    };
    let clicks = clicks_str.parse::<u32>().expect("invalid # clicks");
    return (direction, clicks)
}

/// Turn the dial left a given nr. of clicks
fn turn_left(dial_value: &mut u32, clicks: u32) {
    let clicks_effective = clicks % (MAX_DIAL_VALUE + 1);
    if clicks_effective > *dial_value {
        *dial_value = MAX_DIAL_VALUE - (clicks_effective - *dial_value - 1)
    } else {
        *dial_value -= clicks_effective
    }
}

/// Turn the dial right a given nr. of clicks
fn turn_right(dial_value: &mut u32, clicks: u32) {
    *dial_value += clicks;
    while *dial_value > MAX_DIAL_VALUE {
        *dial_value -= MAX_DIAL_VALUE + 1
    }
}

fn calculate_result(lines: Lines<BufReader<File>>) -> Result<usize, ()> {
    let mut dial_value: u32 = 50;
    let mut n_zeros: u32 = 0;
    let instructions = lines.map(parse_line);
    instructions.for_each(|(direction, clicks)| {
        match direction {
            TurnDirection::Left => turn_left(&mut dial_value, clicks),
            TurnDirection::Right => turn_right(&mut dial_value, clicks),
        }
        if dial_value == 0 {
            n_zeros += 1;
        }
    });
    Ok(n_zeros.try_into().expect("result too large"))
}

fn main() {
    let file = File::open(INPUT_FILE).expect("unable to open file");
    let reader = BufReader::new(file);
    let result = calculate_result(reader.lines()).expect("error calculating result");
    println!("{OUTPUT_MESSAGE}: {result}");
}
