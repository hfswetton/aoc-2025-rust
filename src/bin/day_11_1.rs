use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::time::Instant;

const OUTPUT_MESSAGE: &str = "Total";
const INPUT_FILE: &str = "input/day_11.txt";

type Label = String;

#[derive(Debug, Clone)]
struct Device {
    connections: Vec<Label>,
    is_reactor: bool,
}

impl Device {
    fn new(connections: Vec<Label>) -> Self {
        Self { connections, is_reactor: false }
    }

    fn new_reactor() -> Self {
        Self { connections: Vec::new(), is_reactor: true }
    }
}

fn parse_devices(lines: Lines<BufReader<File>>) -> HashMap<Label, Device> {
    let mut devices: HashMap<Label, Device> = HashMap::new();
    lines.for_each(|line_res| {
        let line = line_res.expect("invalid line");
        let device_label = line[..3].to_string();
        let connection_labels_single: Vec<String> = line[5..].split(' ').map(|s| s.to_string()).collect();
        devices.insert(device_label.clone(), Device::new(connection_labels_single));
    });
    devices.insert("out".to_string(), Device::new_reactor());
    println!("{devices:?}");
    devices
}

fn find_paths_inner(starting_device_label: &Label, devices: &HashMap<Label, Device>, cache: &mut HashMap<Label, usize>) -> usize {
    if let Some(value) = cache.get(&starting_device_label[..]) {
        println!("Found value for {starting_device_label} in cache: {value}");
        return *value;
    }
    let starting_device = devices.get(&starting_device_label[..]).expect("starting device not found");
    println!("Finding paths from {starting_device:?}");
    let next_device_labels = starting_device.connections.iter();
    // Note from checking data: if a device is connected to "out", it has no other connections
    let n_paths = next_device_labels.map(|device_label| {
        let device = devices.get(&device_label[..]).expect("connected device not found");
        if device.is_reactor {
            println!("{starting_device_label} connected to reactor");
            1
        } else {
            find_paths_inner(device_label, devices, cache)
        }
    }).sum::<usize>();
    if n_paths == 0 { panic!("no paths found"); }
    cache.insert(starting_device_label.to_string(), n_paths);
    n_paths
}

fn calculate_result(lines: Lines<BufReader<File>>) -> Result<usize, ()> {
    let devices = parse_devices(lines);
    println!("Devices: {devices:?}");
    let mut cache: HashMap<Label, usize> = HashMap::new();
    let result = find_paths_inner(&"you".to_string(), &devices, &mut cache);
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
