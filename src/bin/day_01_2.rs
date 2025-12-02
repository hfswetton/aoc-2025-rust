use std::fs::File;
use std::io::{BufRead, BufReader, Error, Lines};

const OUTPUT_MESSAGE: &str = "Total";
const INPUT_FILE: &str = "input/day_01.txt";

const MAX_DIAL_VALUE: i32 = 99;

#[derive(Debug)]
enum TurnDirection {
    Left,
    Right,
}

/// Process one input line
fn parse_line(line_res: Result<String, Error>) -> (TurnDirection, i32) {
    let line = line_res.expect("error reading line");
    let (direction_str, clicks_str) = line.split_at(1);
    let direction = match direction_str {
        "L" => TurnDirection::Left,
        "R" => TurnDirection::Right,
        _ => panic!("invalid input"),
    };
    let clicks = clicks_str.parse::<i32>().expect("invalid # clicks");
    return (direction, clicks)
}

/// Turn the dial a given nr. of clicks, counting zeros
/// (ugly algorithm that increments in steps of 1)
fn turn(dial_value: &mut i32, direction: TurnDirection, clicks: i32, zero_counter: &mut u32) {
    print!("{dial_value} -> {clicks} {direction:?} ({zero_counter} zeros -> ");
    match direction {
        TurnDirection::Left => {
            (0..clicks).for_each(|_| {
                if *dial_value == 0 {
                    *dial_value = MAX_DIAL_VALUE;
                } else {
                    *dial_value -= 1;
                    if *dial_value == 0 { *zero_counter += 1; }
                }
            });
        },
        TurnDirection::Right => {
            (0..clicks).for_each(|_| {
                if *dial_value == MAX_DIAL_VALUE {
                    *dial_value = 0;
                    *zero_counter += 1;
                } else {
                    *dial_value += 1;
                }
            });
        },
    };
    println!("{zero_counter} zeros) -> {dial_value}");
}

fn calculate_result(lines: Lines<BufReader<File>>) -> Result<usize, ()> {
    let mut dial_value: i32 = 50;
    let mut n_zeros: u32 = 0;
    lines.map(parse_line).for_each(|(direction, clicks)| {
        turn(&mut dial_value, direction, clicks, &mut n_zeros);
    });
    Ok(n_zeros.try_into().expect("result too large"))
}

fn main() {
    let file = File::open(INPUT_FILE).expect("unable to open file");
    let reader = BufReader::new(file);
    let result = calculate_result(reader.lines()).expect("error calculating result");
    println!("{OUTPUT_MESSAGE}: {result}");
}
