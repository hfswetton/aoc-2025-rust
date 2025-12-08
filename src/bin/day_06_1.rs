use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

const OUTPUT_MESSAGE: &str = "Total";
const INPUT_FILE: &str = "input/day_06.txt";

const N_PROBLEMS: usize = 1000;  // example: 4, real input: 1000
const N_NUMBER_LINES: usize = 4;  // example: 3, real input: 4

#[derive(Debug, Clone)]
enum Operation {
    Add,
    Multiply,
}

type NumberLine = [u64; N_PROBLEMS];
type OperationLine = [Operation; N_PROBLEMS];

fn parse_number_line(line: String) -> NumberLine {
    line
        .split_ascii_whitespace()
        .map(|n| n.parse::<u64>().expect("invalid number"))
        .collect::<Vec<u64>>()
        .try_into()
        .expect("incorrect length")
}

fn parse_operation_line(line: String) -> OperationLine {
    line
        .split_ascii_whitespace()
        .map(|n| match n {
            "+" => Operation::Add,
            "*" => Operation::Multiply,
            _ => panic!("invalid operation"),
        })
        .collect::<Vec<Operation>>()
        .try_into()
        .expect("incorrect length")
}

fn calculate_result(mut lines: Lines<BufReader<File>>) -> Result<u64, ()> {
    let number_lines: Vec<NumberLine> = lines.by_ref().take(N_NUMBER_LINES).map(|line| parse_number_line(line.expect("invalid line"))).collect();
    let operation_line: OperationLine = parse_operation_line(lines.next().expect("operation line missing").expect("invalid line"));
    let total = (0..N_PROBLEMS).map(|i| {
        let iter = number_lines.iter().map(|line| line[i]);
        let result: u64 = match operation_line[i] {
            Operation::Add => iter.sum(),
            Operation::Multiply => iter.product(),
        };
        result
    }).sum();
    Ok(total)
}

fn main() {
    let file = File::open(INPUT_FILE).expect("unable to open file");
    let reader = BufReader::new(file);
    let result = calculate_result(reader.lines()).expect("error calculating result");
    println!("{OUTPUT_MESSAGE}: {result}");
}
