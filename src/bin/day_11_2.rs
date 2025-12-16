use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::time::Instant;

const OUTPUT_MESSAGE: &str = "Total";
const INPUT_FILE: &str = "input/day_11.txt";

#[derive(Debug, Clone)]
struct Device<'a> {
    connections: Vec<&'a str>,
}

impl<'a> Device<'a> {
    fn new(connections: Vec<&'a str>) -> Self {
        Self { connections }
    }
}

fn parse_connections<'a>(lines: Lines<BufReader<File>>) -> HashMap<String, Vec<String>> {
    let mut devices: HashMap<String, Vec<String>> = HashMap::new();
    lines.for_each(|line_res| {
        let line = line_res.expect("invalid line");
        let device_label = line[..3].to_string();
        let connection_labels_single: Vec<String> = line[5..].split(' ').map(|s| s.to_string()).collect();
        devices.insert(device_label, connection_labels_single);
    });
    devices.insert("out".to_string(), Vec::new());
    println!("{devices:?}");
    devices
}

fn count_paths(start_label: &str, end_label: &str, forbidden_labels: HashSet<&str>, devices: &HashMap<&str, Device>) -> usize {
    println!("Counting paths from {start_label} to {end_label}, excluding the following: {forbidden_labels:?}");
    let mut cache: HashMap<String, usize> = HashMap::new();
    let result = count_paths_inner(start_label, end_label, &forbidden_labels, devices, &mut cache);
    println!("{result} paths found.");
    result
}

fn count_paths_inner(start_label: &str, end_label: &str, forbidden_labels: &HashSet<&str>, devices: &HashMap<&str, Device>, cache: &mut HashMap<String, usize>) -> usize {
    if let Some(value) = cache.get(start_label) {
        //println!("Found value for {start_label} in cache: {value}");
        return *value;
    }
    let starting_device = devices.get(start_label).expect("starting device not found");
    //println!("Finding paths from {starting_device:?}");
    let next_device_labels = starting_device.connections.iter();
    // Note from checking data: if a device is connected to "out", it has no other connections
    let n_paths = next_device_labels.map(|device_label| {
        if *device_label == end_label {
            1
        } else if forbidden_labels.contains(device_label) {
            0
        } else {
            count_paths_inner(device_label, end_label, forbidden_labels, devices, cache)
        }
    }).sum::<usize>();
    cache.insert(start_label.to_string(), n_paths);
    n_paths
}

fn calculate_result(lines: Lines<BufReader<File>>) -> Result<usize, ()> {
    let connections = parse_connections(lines);
    let devices: HashMap<&str, Device> = connections.iter().map(|(label, connections)| {
        (&label[..], Device::new(connections.iter().map(|s| &s[..]).collect()))
    }).collect();
    println!("Devices: {devices:?}");

    let valid_paths_1 = count_paths("svr", "fft", ["dac", "out"].into(), &devices)
        * count_paths("fft", "dac", ["svr", "out"].into(), &devices)
        * count_paths("dac", "out", ["svr", "fft"].into(), &devices);

    let valid_paths_2 = count_paths("svr", "dac", ["fft", "out"].into(), &devices)
        * count_paths("dac", "fft", ["svr", "out"].into(), &devices)
        * count_paths("fft", "out", ["svr", "dac"].into(), &devices);

    let result = valid_paths_1 + valid_paths_2;
    Ok(result)
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
