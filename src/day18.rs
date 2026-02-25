#[derive(Debug)]
enum Operation {
    Add,
    Mul,
}
use Operation as Op;

fn evaluate_left_precedence(line: &str) -> u64 {
    let line = line.replace(")", " )").replace("(", "( "); // 🤮 
    let mut value_stack = vec![];
    let mut value = 0;
    let mut op = Op::Add;
    for symbol in line.split_whitespace() {
        match symbol {
            "+" => op = Op::Add,
            "*" => op = Op::Mul,
            "(" => {
                value_stack.push((op, value));
                value = 0;
                op = Op::Add
            }
            ")" => {
                let paren_value = value;
                (op, value) = value_stack.pop().unwrap();
                match op {
                    Op::Add => value += paren_value,
                    Op::Mul => value *= paren_value,
                }
            }
            // is numeric
            _ => match op {
                Op::Add => value += symbol.parse::<u64>().unwrap(),
                Op::Mul => value *= symbol.parse::<u64>().unwrap(),
            },
        }
    }
    debug_assert!(value_stack.is_empty());
    value
}

pub fn task1(input: &str) -> String {
    input
        .lines()
        .map(evaluate_left_precedence)
        .sum::<u64>()
        .to_string()
}

fn evaluate_add_precedence(line: &str) -> u64 {
    let line = line.replace(")", " )").replace("(", "( "); // 🤮 
    let mut paren_stack = vec![];
    let mut factor = 1;
    let mut product = 1;
    let mut op = Op::Mul;
    for symbol in line.split_whitespace() {
        match symbol {
            "+" => op = Op::Add,
            "*" => {
                op = Op::Mul;
                product *= factor;
                factor = 1;
            }
            "(" => {
                paren_stack.push((op, factor, product));
                factor = 1;
                product = 1;
                op = Op::Mul;
            }
            ")" => {
                let paren_value = product * factor;
                (op, factor, product) = paren_stack.pop().unwrap();
                match op {
                    Op::Add => factor += paren_value,
                    Op::Mul => factor *= paren_value,
                }
            }
            // is numeric
            _ => match op {
                Op::Add => factor += symbol.parse::<u64>().unwrap(),
                Op::Mul => factor *= symbol.parse::<u64>().unwrap(),
            },
        }
    }
    debug_assert!(paren_stack.is_empty());
    product *= factor;
    product
}

pub fn task2(input: &str) -> String {
    input
        .lines()
        .map(evaluate_add_precedence)
        .sum::<u64>()
        .to_string()
}
