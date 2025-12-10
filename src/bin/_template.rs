use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::time::Instant;

const OUTPUT_MESSAGE: &str = "Total";
const INPUT_FILE: &str = "input/day_XX.txt";

fn calculate_result(lines: Lines<BufReader<File>>) -> Result<usize, ()> {
    Ok(0)
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
