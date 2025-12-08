use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use itertools::Itertools;

const OUTPUT_MESSAGE: &str = "Total";
const INPUT_FILE: &str = "input/day_08.txt";

const N_JUNCTION_BOXES: usize = 1000;  // example: 20, real input: 1000
const N_LARGEST_CIRCUITS: usize = 3;

type CoordElement = i64;
type Dist = f32;

#[derive(Clone, Copy, Debug)]
struct JunctionBox {
    x: CoordElement,
    y: CoordElement,
    z: CoordElement,
}

impl JunctionBox {
    /// Calculate Euclidean distance to another junction box
    fn dist(self, other: Self) -> Dist {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        let dist_sq: Dist = (dx.pow(2) + dy.pow(2) + dz.pow(2)) as Dist;
        dist_sq.sqrt()
    }

    /// Parse one line of input
    fn from_string(input: String) -> Result<Self, ()> {
        let parts: Vec<&str> = input.trim().split(",").collect();
        if parts.len() != 3 {
            Err(())
        } else {
            let x = parts[0].parse::<CoordElement>();
            let y = parts[1].parse::<CoordElement>();
            let z = parts[2].parse::<CoordElement>();
            if let (Ok(xx), Ok(yy), Ok(zz)) = (x, y, z) {
                Ok(Self{x: xx, y: yy, z: zz})
            } else {
                Err(())
            }
        }
    }
}

fn build_connection_matrix(pairs_sorted: &Vec<((usize, &JunctionBox), (usize, &JunctionBox))>, n_connections: usize) -> ([[bool; N_JUNCTION_BOXES]; N_JUNCTION_BOXES], (usize, usize)) {
    let mut matrix = [[false; N_JUNCTION_BOXES]; N_JUNCTION_BOXES];
    pairs_sorted.iter().take(n_connections).for_each(|((i, _), (j, _))| {
        matrix[*i][*j] = true;
        matrix[*j][*i] = true;
    });
    let ((last_connection_i, _), (last_connection_j, _)) = pairs_sorted[n_connections - 1];
    (matrix, (last_connection_i, last_connection_j))
}

fn extract_circuit_sizes(connection_matrix: [[bool; N_JUNCTION_BOXES]; N_JUNCTION_BOXES]) -> Vec<u64> {
    let mut checked = [false; N_JUNCTION_BOXES];
    let mut circuit_sizes: Vec<u64> = Vec::new();
    while checked.iter().any(|v| !v) {
        let next_idx = checked.iter().enumerate().filter_map(|(i, &v)| if v { None } else { Some(i) }).next().unwrap();
        let mut circuit_idxs: Vec<usize> = vec!(next_idx);
        let mut idx = 0;
        while idx < circuit_idxs.len() {
            let conn_vec = connection_matrix[circuit_idxs[idx]];
            let new_idxs: Vec<usize> = conn_vec.iter().enumerate().filter_map(|(i, &v)| if v { Some(i) } else { None }).collect();
            new_idxs.iter().for_each(|i| if !circuit_idxs.contains(&i) { circuit_idxs.push(i.clone()) });
            idx += 1;
        }
        circuit_idxs.iter().for_each(|i| checked[*i] = true);
        circuit_sizes.push(circuit_idxs.len() as u64);
    }
    circuit_sizes.sort_unstable();
    circuit_sizes.reverse();
    circuit_sizes
}

fn calculate_result(lines: Lines<BufReader<File>>) -> Result<i64, ()> {
    let junction_boxes: Vec<JunctionBox> = lines.map(|l| JunctionBox::from_string(l.expect("invalid line")).expect("unable to parse junction box")).collect();
    let mut pairs: Vec<((usize, &JunctionBox), (usize, &JunctionBox))> = junction_boxes.iter().enumerate().tuple_combinations().collect();
    pairs.sort_by(|&((_ia1, &a1), (_ib1, &b1)), &((_ia2, &a2), (_ib2, &b2))| {
        let dist_1 = a1.dist(b1);
        let dist_2 = a2.dist(b2);
        dist_1.partial_cmp(&dist_2).expect("invalid_ordering")
    });
    let mut circuit_sizes = vec!(0, 0, 0);  // dummy value
    let mut last_connection_i = 0;
    let mut last_connection_j = 0;
    let mut n_connections = 1;
    while circuit_sizes.len() != 2 {  // Start binary-search-ish to reduce the total search time
        if circuit_sizes.len() > 2 { n_connections *= 2; } else { n_connections -= n_connections / 10; }
        let (connection_matrix, _) = build_connection_matrix(&pairs, n_connections);
        circuit_sizes = extract_circuit_sizes(connection_matrix);
        println!("{n_connections} connections: {} circuits", circuit_sizes.len());
    }
    while circuit_sizes.len() > 1 {  // Find the final connection one step at a time
        n_connections += 1;
        let (connection_matrix, last_connection_idxs) = build_connection_matrix(&pairs, n_connections);
        (last_connection_i, last_connection_j) = last_connection_idxs;
        println!("Last connection: ({:?}, {:?})", junction_boxes[last_connection_i], junction_boxes[last_connection_j]);
        circuit_sizes = extract_circuit_sizes(connection_matrix);
        println!("{n_connections} connections: {} circuits", circuit_sizes.len());
    }
    let total = junction_boxes[last_connection_i].x * junction_boxes[last_connection_j].x;
    Ok(total)
}

fn main() {
    let file = File::open(INPUT_FILE).expect("unable to open file");
    let reader = BufReader::new(file);
    let result = calculate_result(reader.lines()).expect("error calculating result");
    println!("{OUTPUT_MESSAGE}: {result}");
}
