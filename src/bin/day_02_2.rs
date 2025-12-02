use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

const OUTPUT_MESSAGE: &str = "Total";
const INPUT_FILE: &str = "input/day_02.txt";

/// Check a single number for repeating patterns
fn check_number(number: u64) -> bool {
    let number_str = format!("{number}");
    for k in 1..(number_str.len() / 2 + 1) {
        if number_str.len() % k == 0 {
            let n_reps: usize = (number_str.len() / k).try_into().expect("n_reps can't be cast to usize");
            let first_part = &number_str[..k];
            let mut all_equal = true;
            for i in 1..n_reps {
                let compared_part = &number_str[(i * k)..((i + 1) * k)];
                if first_part.ne(compared_part) {
                    all_equal = false;
                    break
                }
            }
            if all_equal { return true; }
        }
    }
    false
}

/// Check a range of numbers and return the sum of all invalid IDs
fn check_range(range: (u64, u64)) -> u64 {
    let (a, b0) = range;
    let b = b0 + 1;
    let mut total = 0;
    (a..b).for_each(|n| {
        if check_number(n) { println!("Invalid ID found: {n}"); total += n; }
    });
    total
}

fn calculate_result(mut lines: Lines<BufReader<File>>) -> Result<usize, ()> {
    let ranges_str = lines.next().expect("no line found").expect("unable to read line");
    let ranges_separated = ranges_str.split(",");
    let ranges = ranges_separated.map(|r| {
        let mut r_iter = r.split("-");
        let a = r_iter.next().expect("1st bound missing").parse::<u64>().expect("invalid number");
        let b = r_iter.next().expect("2nd bound missing").parse::<u64>().expect("invalid number");
        (a, b)
    });
    let result: u64 = ranges.map(check_range).sum();
    Ok(result.try_into().expect("unable to convert result to usize"))
}

fn main() {
    let file = File::open(INPUT_FILE).expect("unable to open file");
    let reader = BufReader::new(file);
    let result = calculate_result(reader.lines()).expect("error calculating result");
    println!("{OUTPUT_MESSAGE}: {result}");
}
