use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::time::Instant;

const OUTPUT_MESSAGE: &str = "Total";
const INPUT_FILE: &str = "input/day_10.txt";

const MAX_PRESSES: usize = 16;  // maximum number of buttons to try before giving up
const MAX_N_LIGHTS: usize = 10;  // maximum required array size

#[derive(Debug, Clone)]
struct Machine {
    target_light_states: [bool; MAX_N_LIGHTS],
    button_wiring: Vec<Vec<usize>>,
}

impl Machine {
    fn new(target_light_states_vec: Vec<bool>, button_wiring: Vec<Vec<usize>>) -> Self {
        if target_light_states_vec.len() > MAX_N_LIGHTS {
            panic!("attempted to create a Machine with more than MAX_N_LIGHTS lights");
        }
        let mut target_light_states = [false; MAX_N_LIGHTS];
        target_light_states_vec.into_iter().enumerate().for_each(|(i, v)| target_light_states[i] = v);
        Self {
            target_light_states,
            button_wiring,
        }
    }

    fn from_spec(spec: String) -> Self {
        let parts: Vec<&str> = spec.split(' ').collect();
        let spec_lights = &parts[0];
        let spec_buttons = &parts[1..(parts.len() - 1)];
        let _spec_joltage = &parts[parts.len() - 1];

        let target_light_states: Vec<bool> = spec_lights[1..(spec_lights.len() - 1)].chars().map(|c| c == '#').collect();
        let n_lights = target_light_states.len();
        let button_wiring: Vec<Vec<usize>> = (*spec_buttons).iter().map(|button_str| {
            button_str[1..(button_str.len() - 1)].split(',').map(|n| {
                let button_nr = n.parse::<usize>().expect("invalid button number");
                if button_nr >= n_lights { panic!("invalid button number"); }
                button_nr
            }).collect()
        }).collect();

        Self::new(target_light_states, button_wiring)
    }

    fn find_minimum_button_presses(&self) -> usize {
        println!("{self:?}");
        let mut n_presses = 1;
        while n_presses < MAX_PRESSES {
            let button_configs: Vec<Vec<usize>> = generate_button_configs(n_presses, self.button_wiring.len());
            println!("Testing {} button pressing options with length {}", button_configs.len(), n_presses);
            if let Some(buttons) = button_configs.into_iter().filter(|buttons| self.test_button_config(buttons)).next() {
                println!("Valid combination found: {buttons:?} ({} presses)", buttons.len());
                println!();
                return buttons.len();
            }
            n_presses += 1;
        }
        panic!("no valid button combination found");
    }

    fn test_button_config(&self, buttons: &Vec<usize>) -> bool {
        let mut light_states = [false; MAX_N_LIGHTS];
        buttons.iter().for_each(|&b| {
            self.button_wiring[b].iter().for_each(|&i| light_states[i] = !light_states[i]);
        });
        let result = light_states == self.target_light_states;
        result
    }
}

/// Generate all "sensible"* button press orders with length `n_buttons`.
///
/// *sensible = no "double-presses"
fn generate_button_configs(n_presses: usize, n_buttons: usize) -> Vec<Vec<usize>> {
    if n_presses < 1 {
        panic!("n_presses must be at least 1")
    } else if n_presses == 1 {
        (0..n_buttons).map(|n| vec!(n)).collect()
    } else {
        let previous_buttons = generate_button_configs(n_presses - 1, n_buttons);
        (0..n_buttons).flat_map(|n| {
            previous_buttons.clone().into_iter().filter_map(move |list| {
                if list[list.len() - 1] == n {  // skip "double-presses"
                    None
                } else {
                    let mut new_list = list.clone();
                    new_list.push(n);
                    Some(new_list)
                }
            })
        }).collect()
    }
}

fn calculate_result(lines: Lines<BufReader<File>>) -> Result<usize, ()> {
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
