use microlp::{ComparisonOp, OptimizationDirection, Problem, Variable};
use regex::Regex;
use std::collections::HashSet;

#[derive(Debug)]
struct Machine {
    indicator_lights: usize,
    buttons: Vec<usize>,
    joltage: Vec<u16>,
}

fn parse_machines(input: &str) -> Vec<Machine> {
    let machine_re = Regex::new(r"^\[(?<lights>[\.\#]+)\][^{]*\{(?<joltage>(\d+,?)*)\}$").unwrap();
    let buttons_re = Regex::new(r"\(((?:\d+,?)*)\)").unwrap();
    input
        .lines()
        .map(|line| {
            let caps = machine_re.captures(line).unwrap();
            let indicator_lights = caps["lights"]
                .bytes()
                .rev()
                .map(|light| match light {
                    b'#' => 1,
                    b'.' => 0,
                    _ => panic!(),
                })
                .fold(0, |acc, digit| acc * 2 + digit);
            let joltage: Vec<u16> = caps["joltage"]
                .split(',')
                .map(|num| num.parse().unwrap())
                .collect();
            let mut buttons = Vec::new();
            for caps in buttons_re.captures_iter(line) {
                buttons.push(
                    caps[1]
                        .split(',')
                        .map(|num| 1 << num.parse::<u8>().unwrap())
                        .sum(),
                );
            }
            Machine {
                indicator_lights,
                buttons,
                joltage,
            }
        })
        .collect()
}

fn activation_button_presses(machine: &Machine) -> usize {
    let mut visited: HashSet<usize> = HashSet::new();
    let mut current = &mut vec![0];
    let mut next = &mut vec![];
    for presses in 1.. {
        for lights in &mut *current {
            for button in &machine.buttons {
                let new_light = *lights ^ button;
                if visited.contains(&new_light) {
                    continue;
                }
                visited.insert(new_light);
                if new_light == machine.indicator_lights {
                    return presses;
                }
                next.push(new_light);
            }
        }
        (next, current) = (current, next);
        next.truncate(0);
    }
    panic!()
}

pub fn task1(input: &str) -> String {
    parse_machines(input)
        .iter()
        .map(activation_button_presses)
        .sum::<usize>()
        .to_string()
}

fn press(mut button: usize, joltage: &[u16]) -> Vec<u16> {
    let mut result = joltage.to_vec();
    for value in &mut result {
        if button % 2 == 1 {
            *value += 1;
        }
        button >>= 1;
    }
    result
}

// fn configureation_button_presses(machine: &Machine) -> usize {
//     let mut visited: HashSet<Vec<u16>> = HashSet::new();
//     let mut current = &mut vec![vec![0; machine.joltage.len()]];
//     let mut next = &mut vec![];
//     for presses in 1..100 {
//         for joltage in &mut *current {
//             for button in &machine.buttons {
//                 let new_joltage = press(*button, joltage);
//                 if visited.contains(&new_joltage) {
//                     continue;
//                 }
//                 if new_joltage.iter().zip(machine.joltage.iter()).any(|(current, target)| current > target) {
//                     continue;
//                 }
//                 if new_joltage == machine.joltage {
//                     return presses;
//                 }
//                 visited.insert(new_joltage.clone());
//                 next.push(new_joltage);
//             }
//         }
//         (next, current) = (current, next);
//         next.truncate(0);
//     }
//     0
// }

fn configureation_button_presses(machine: &Machine) -> usize {
    let mut problem = Problem::new(OptimizationDirection::Minimize);
    let max = *machine.joltage.iter().max().unwrap() as i32;
    let vars: Box<[Variable]> = machine
        .buttons
        .iter()
        .map(|_| problem.add_integer_var(1.0, (0, max)))
        .collect();
    for (i, spec) in machine.joltage.iter().enumerate() {
        let spec_vars = machine
            .buttons
            .iter()
            .zip(vars.iter())
            .filter_map(|(button, var)| {
                if button & 1 << i != 0 {
                    Some((*var, 1.0))
                } else {
                    None
                }
            });
        problem.add_constraint(spec_vars, ComparisonOp::Eq, (*spec).into());
    }
    let solution = problem.solve().unwrap();
    solution.objective() as usize
}

pub fn task2(input: &str) -> String {
    parse_machines(input)
        .iter()
        .map(configureation_button_presses)
        .inspect(|presses| println!("{presses}"))
        .sum::<usize>()
        .to_string()
}
