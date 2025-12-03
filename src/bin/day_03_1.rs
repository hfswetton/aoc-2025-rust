use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

const OUTPUT_MESSAGE: &str = "Total";
const INPUT_FILE: &str = "input/day_03.txt";

fn largest_digit_position_value(bank: &str) -> (usize, u32) {
    bank
        .chars()
        .enumerate()
        .fold(
            (0, 0),
            |(idx, val), (i, n_str)| {
                let n = n_str.to_digit(10).expect("invalid digit");
                if n > val { (i, n) } else { (idx, val) }
            }
        )
}

fn following_largest_digit(bank: &str, largest_pos: usize) -> u32 {
    let (_, v) = largest_digit_position_value(&bank[(largest_pos + 1)..]);
    v
}

fn calculate_result(lines: Lines<BufReader<File>>) -> Result<usize, ()> {
    let total: u32 = lines.map(|line_res| {
        let line = line_res.expect("invalid_line");
        let (i, a) = largest_digit_position_value(&line[..(line.len() - 1)]);
        let b = following_largest_digit(&line, i);
        let res = 10 * a + b;
        println!("{line} -> {res}");
        res
    }).sum();
    Ok(total.try_into().expect("invalid usize"))
}

fn main() {
    let file = File::open(INPUT_FILE).expect("unable to open file");
    let reader = BufReader::new(file);
    let result = calculate_result(reader.lines()).expect("error calculating result");
    println!("{OUTPUT_MESSAGE}: {result}");
}
