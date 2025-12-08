use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

const OUTPUT_MESSAGE: &str = "Total";
const INPUT_FILE: &str = "input/day_06.txt";

const N_PROBLEMS: usize = 1000;  // example: 4, real input: 1000
const N_NUMBER_LINES: usize = 4;  // example: 3, real input: 4
const N_NUMBER_CHARS: usize = 3766;  // line width - example: 15, real input: 3766

const SPACE_BYTE: u8 = 0x20;

#[derive(Debug, Clone)]
enum Operation {
    Add,
    Multiply,
}

type ProblemNumbers = Vec<u64>;
type OperationLine = [Operation; N_PROBLEMS];

fn parse_number_lines(lines: Vec<String>) -> Vec<ProblemNumbers> {
    // "transpose" input lines
    let mut chars: [[u8; N_NUMBER_CHARS]; N_NUMBER_LINES] = [[SPACE_BYTE; N_NUMBER_CHARS]; N_NUMBER_LINES];
    lines.into_iter().enumerate().for_each(|(i, line)| {
        line.into_bytes().iter().enumerate().for_each(|(j, c)| chars[i][j] = *c);
    });
    let lines_transp: Vec<String> = (0..N_NUMBER_CHARS).map(|j| {
        let bytes: Vec<u8> = (0..N_NUMBER_LINES).map(|i| chars[i][j]).collect();
        String::from_utf8(bytes).unwrap()
    }).collect();

    // extract numbers
    let mut problem_numbers: Vec<ProblemNumbers> = Vec::new();
    problem_numbers.push(Vec::new());
    lines_transp.into_iter().for_each(|line| {
        if line.trim().len() == 0 {
            problem_numbers.push(Vec::new());
        } else {
            let idx = problem_numbers.len() - 1;
            problem_numbers[idx].push(line.trim().parse().expect("invalid number"));
        }
    });
    problem_numbers
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
    let number_lines: Vec<String> = lines.by_ref().take(N_NUMBER_LINES).map(|line| line.expect("invalid line")).collect();
    let numbers: Vec<ProblemNumbers> = parse_number_lines(number_lines);
    let operation_line: OperationLine = parse_operation_line(lines.next().expect("operation line missing").expect("invalid line"));
    let total = (0..N_PROBLEMS).map(|i| {
        let iter = numbers[i].iter();
        let result: u64 = match operation_line[i] {
            Operation::Add => iter.sum(),
            Operation::Multiply => iter.product(),
        };
        println!("{:?} {:?} -> {}", operation_line[i], numbers[i], result);
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
