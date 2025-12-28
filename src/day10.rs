use microlp::{ComparisonOp, OptimizationDirection, Problem, Variable};
use regex::Regex;
use std::{collections::HashSet, sync::LazyLock};

static LIGHTS_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\[([\.\#]+)\]").unwrap());
static BUTTONS_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\(((?:\d+,?)*)\)").unwrap());
static JOLTAGE_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\{((\d+,?)*)\}$").unwrap());

#[derive(Debug)]
struct Machine {
    indicator_lights: usize,
    buttons: Vec<usize>,
    joltage: Vec<u16>,
}

fn parse_machine(line: &str) -> Machine {
    let indicator_lights = LIGHTS_RE.captures(line).unwrap()[1]
        .bytes()
        .enumerate()
        .map(|(i, light)| match light {
            b'#' => 1 << i,
            b'.' => 0,
            _ => panic!(),
        })
        .sum();
    let joltage: Vec<u16> = JOLTAGE_RE.captures(line).unwrap()[1]
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect();
    let buttons = BUTTONS_RE
        .captures_iter(line)
        .map(|caps| {
            caps[1]
                .split(',')
                .map(|num| 1 << num.parse::<u8>().unwrap())
                .sum()
        })
        .collect();
    Machine {
        indicator_lights,
        buttons,
        joltage,
    }
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
    input
        .lines()
        .map(parse_machine)
        .map(|machine| activation_button_presses(&machine))
        .sum::<usize>()
        .to_string()
}

fn configureation_button_presses(machine: &Machine) -> usize {
    let mut problem = Problem::new(OptimizationDirection::Minimize);
    let max = *machine.joltage.iter().max().unwrap() as i32;
    let presses: Box<[Variable]> = machine
        .buttons
        .iter()
        .map(|_| problem.add_integer_var(1.0, (0, max)))
        .collect();
    for (i, joltage) in machine.joltage.iter().enumerate() {
        let joltage_presses =
            machine
                .buttons
                .iter()
                .zip(presses.iter())
                .filter_map(|(button, var)| {
                    if button & (1 << i) != 0 {
                        Some((*var, 1.0))
                    } else {
                        None
                    }
                });
        problem.add_constraint(joltage_presses, ComparisonOp::Eq, (*joltage).into());
    }
    problem.solve().unwrap().objective().round() as usize
}

pub fn task2(input: &str) -> String {
    input
        .lines()
        .map(parse_machine)
        .map(|machine| configureation_button_presses(&machine))
        .sum::<usize>()
        .to_string()
}
