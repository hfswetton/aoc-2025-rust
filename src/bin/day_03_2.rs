use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

const OUTPUT_MESSAGE: &str = "Total";
const INPUT_FILE: &str = "input/day_03.txt";

fn largest_digit_position_value(bank: &str) -> (usize, u64) {
    bank
        .chars()
        .enumerate()
        .fold(
            (0, 0),
            |(idx, val), (i, n_str)| {
                let n = n_str.to_digit(10).expect("invalid digit") as u64;
                if n > val { (i, n) } else { (idx, val) }
            }
        )
}

fn largest_12_digit_number(bank: &str) -> u64 {
    let mut idx: usize = 0;
    let mut digits: Vec<u64> = Vec::new();
    (0..12).for_each(|k| {
        let (i, v) = largest_digit_position_value(&bank[idx..(bank.len() - (11 - k))]);
        idx += i + 1;
        digits.push(v);
    });
    digits.reverse();
    digits.iter().enumerate().map(|(i, v)| v * 10_u64.pow(i as u32)).sum()
}

fn calculate_result(lines: Lines<BufReader<File>>) -> Result<usize, ()> {
    let total: u64 = lines.map(|line_res| {
        let line = line_res.expect("invalid_line");
        let res = largest_12_digit_number(&line);
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
