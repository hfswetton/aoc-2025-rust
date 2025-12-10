use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::time::Instant;
use z3::{ast::Int, Solver};

const OUTPUT_MESSAGE: &str = "Total";
const INPUT_FILE: &str = "input/day_10.txt";

type CounterValue = u16;

/// Maximum number of solutions to check with z3 if many present
/// -> 128 too small, 2048 large enough
/// (slight time penalty: ~60s for 32, ~65s for 128, ~80s for 2048)
const MAX_N_SOLUTIONS: usize = 2048;

#[derive(Debug, Clone)]
struct Machine {
    target_counter_states: Vec<CounterValue>,
    button_wiring: Vec<Vec<usize>>,
}

impl Machine {
    fn new(target_counter_states: Vec<CounterValue>, button_wiring: Vec<Vec<usize>>) -> Self {
        Self {
            target_counter_states,
            button_wiring,
        }
    }

    fn from_spec(spec: String) -> Self {
        let parts: Vec<&str> = spec.split(' ').collect();
        let _spec_lights = &parts[0];
        let spec_buttons = &parts[1..(parts.len() - 1)];
        let spec_joltage = &parts[parts.len() - 1];

        let target_counter_states: Vec<CounterValue> = spec_joltage[1..(spec_joltage.len() - 1)].split(',').map(|n| {
            n.parse::<CounterValue>().expect("invalid joltage value")
        }).collect();
        let n_lights = target_counter_states.len();
        let button_wiring: Vec<Vec<usize>> = (*spec_buttons).iter().map(|button_str| {
            button_str[1..(button_str.len() - 1)].split(',').map(|n| {
                let button_nr = n.parse::<usize>().expect("invalid button number");
                if button_nr >= n_lights { panic!("invalid button number"); }
                button_nr
            }).collect()
        }).collect();

        Self::new(target_counter_states, button_wiring)
    }

    fn find_minimum_button_presses(&self) -> u64 {
        let n_buttons = self.button_wiring.len();

        let solver = Solver::new();
        let presses: Vec<Int> = (0..n_buttons).map(|n| Int::fresh_const(&format!("button_{n}_presses"))).collect();
        presses.iter().for_each(|value| solver.assert(value.ge(0)));
        self.target_counter_states.iter().enumerate().for_each(|(counter_idx, target_state)| {
            let buttons_affecting: Vec<usize> = self.button_wiring.iter().enumerate().filter_map(|(i, counters)| if counters.contains(&counter_idx) { Some(i) } else { None }).collect();
            if let Some(counter_constraint) = buttons_affecting.iter().map(|n| presses[*n].clone()).reduce(|acc, c| acc + &c) {
                solver.assert(counter_constraint.eq(*target_state));
            } else {
                if *target_state > 0 {
                    panic!("unreachable counter state required");
                }
            }
        });
        let solutions = solver.solutions(presses, false).take(MAX_N_SOLUTIONS);
        if let Some(best_solution) = solutions
            .map(|s| {
                s
                    .iter()
                    .map(|sol_int| sol_int.as_u64().expect("missing solution"))
                    .collect::<Vec<u64>>()
            })
            .min_by_key(|presses| presses.iter().sum::<u64>())
        {
            let n_presses = best_solution.iter().sum();
            println!("Best solution: {:?} ({} presses)", best_solution, n_presses);
            n_presses
        } else {
            panic!("no solutions found");
        }
    }
}

fn calculate_result(lines: Lines<BufReader<File>>) -> Result<u64, ()> {
    println!("Parsing machine specs:");
    let machines: Vec<Machine> = lines.map(|line| {
        let machine = Machine::from_spec(line.expect("invalid line"));
        println!("{machine:?}");
        machine
    }).collect();
    println!();
    let result = machines.iter().map(|machine| machine.find_minimum_button_presses()).sum();
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
