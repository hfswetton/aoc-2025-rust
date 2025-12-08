use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use itertools::Itertools;

const OUTPUT_MESSAGE: &str = "Total";
const INPUT_FILE: &str = "input/day_08.txt";

const N_JUNCTION_BOXES: usize = 1000;  // example: 20, real input: 1000
const N_CONNECTIONS: usize = 1000;  // example: 10, real input: 1000
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

fn build_connection_matrix(pairs_sorted: Vec<((usize, &JunctionBox), (usize, &JunctionBox))>) -> [[bool; N_JUNCTION_BOXES]; N_JUNCTION_BOXES] {
    let mut matrix = [[false; N_JUNCTION_BOXES]; N_JUNCTION_BOXES];
    pairs_sorted.iter().take(N_CONNECTIONS).for_each(|((i, _), (j, _))| {
        matrix[*i][*j] = true;
        matrix[*j][*i] = true;
    });
    matrix
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

fn calculate_result(lines: Lines<BufReader<File>>) -> Result<u64, ()> {
    let junction_boxes: Vec<JunctionBox> = lines.map(|l| JunctionBox::from_string(l.expect("invalid line")).expect("unable to parse junction box")).collect();
    let mut pairs: Vec<((usize, &JunctionBox), (usize, &JunctionBox))> = junction_boxes.iter().enumerate().tuple_combinations().collect();
    pairs.sort_by(|&((_ia1, &a1), (_ib1, &b1)), &((_ia2, &a2), (_ib2, &b2))| {
        let dist_1 = a1.dist(b1);
        let dist_2 = a2.dist(b2);
        dist_1.partial_cmp(&dist_2).expect("invalid_ordering")
    });
    let connection_matrix = build_connection_matrix(pairs);
    let circuit_sizes = extract_circuit_sizes(connection_matrix);
    println!("Circuit sizes: {circuit_sizes:?}");
    let total = circuit_sizes.iter().take(N_LARGEST_CIRCUITS).product();
    Ok(total)
}

fn main() {
    let file = File::open(INPUT_FILE).expect("unable to open file");
    let reader = BufReader::new(file);
    let result = calculate_result(reader.lines()).expect("error calculating result");
    println!("{OUTPUT_MESSAGE}: {result}");
}
