use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

const OUTPUT_MESSAGE: &str = "Total";
const INPUT_FILE: &str = "input/day_05.txt";

/// Range of IDs as parsed from file - ends inclusive
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
struct IDRange(u64, u64);

impl IDRange {
    /// Try to combine two `IDRange`s to one object,
    /// returning the new `Ok<IDRange>` if successful and an `Err` otherwise.
    fn try_combine(&self, other: &Self) -> Result<Self, ()> {
        let (first_range, second_range) = if self.0 < other.0 { (self, other) } else { (other, self) };
        if first_range.1 >= second_range.0 {
            let new_right_bound = max(first_range.1, second_range.1);
            Ok(IDRange(first_range.0, new_right_bound))
        } else {
            Err(())
        }
    }

    /// Parse a `String` into an `IDRange`, returning a `Result`.
    fn from_string(spec: String) -> Result<Self, ()> {
        let bounds: Vec<&str> = spec.split("-").collect();
        if bounds.len() == 2 {
            Ok(IDRange(
                bounds[0].parse::<u64>().expect("invalid left bound"),
                bounds[1].parse::<u64>().expect("invalid right bound"),
            ))
        } else {
            Err(())
        }
    }

    fn len(&self) -> u64 {
        self.1 - self.0 + 1
    }
}

fn calculate_result(mut lines: Lines<BufReader<File>>) -> Result<usize, ()> {
    let ranges: Vec<IDRange> = lines
        .by_ref()
        .take_while(|l| l.as_ref().expect("invalid line").clone().trim() != "")
        .map(|l| IDRange::from_string(l.unwrap()).expect("invalid ID range spec"))
        .collect();
    println!("{ranges:?}");
    let mut ranges_deduplicated: Vec<IDRange> = ranges.clone();
    ranges_deduplicated.sort();
    let mut ranges_deduplicated_last: Vec<IDRange> = Vec::new();
    while ranges_deduplicated.len() != ranges_deduplicated_last.len() {
        ranges_deduplicated_last = ranges_deduplicated.clone();
        ranges_deduplicated = ranges_deduplicated.iter().fold(Vec::new(), |mut acc: Vec<IDRange>, range| {
            if let Some(last) = acc.pop() {
                if let Ok(new_range) = last.try_combine(range) {
                    acc.push(new_range);
                } else {
                    acc.push(last);
                    acc.push(*range);
                }
            } else {
                acc.push(*range);
            }
            acc
        });
        println!("Ranges after deduplicating: {ranges_deduplicated:?}")
    }
    let total: u64 = ranges_deduplicated.iter().map(|range| range.len()).sum();
    Ok(total.try_into().expect("result is invalid usize"))
}

fn main() {
    let file = File::open(INPUT_FILE).expect("unable to open file");
    let reader = BufReader::new(file);
    let result = calculate_result(reader.lines()).expect("error calculating result");
    println!("{OUTPUT_MESSAGE}: {result}");
}
