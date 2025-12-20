use std::collections::HashSet;

use regex::Regex;

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
            let joltage = caps["joltage"]
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

pub fn task2(_input: &str) -> String {
    todo!("Day 10 task 2")
}
