use regex::Regex;

struct Machine {
    indicator_lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    _joltage: Vec<u8>,
}

fn parse_machines(input: &str) -> Vec<Machine> {
    let machine_re = Regex::new(
        r"(?x)
    \[(?<lights>[\.\#]+)\]\ 
    (?<buttons>(\((\d+,?)*\)\ )*)
    \{(?<joltage>(\d+,?)*)\}
    ",
    )
    .unwrap();
    let re = Regex::new(r"\((\d*)(,)?\)").unwrap();
    let a: Vec<()> = input
        .lines()
        .map(|line| {
            let caps = machine_re.captures(line).unwrap();
            println!("{caps:?}");
            // let (_, buttons) = re.captures(line).unwrap().extract();
            // println!("{buttons:?}");
        })
        .collect();
    todo!()
}

pub fn task1(input: &str) -> String {
    parse_machines(input);
    todo!("Day 10 task 1")
}

pub fn task2(_input: &str) -> String {
    todo!("Day 10 task 2")
}
