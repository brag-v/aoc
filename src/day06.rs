enum Operator {
    Add,
    Multiply,
}

fn get_nums_and_operators(input: &str) -> (Vec<Vec<u64>>, Vec<Operator>) {
    let lines: Box<[&str]> = input.lines().collect();

    let nums: Vec<Vec<u64>> = lines[..(lines.len() - 1)]
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect();
    let ops: Vec<Operator> = lines[lines.len() - 1]
        .split_whitespace()
        .map(|op| match op {
            "+" => Operator::Add,
            "*" => Operator::Multiply,
            _ => panic!(),
        })
        .collect();
    debug_assert!(nums.iter().all(|line| line.len() == ops.len()));
    (nums, ops)
}

pub fn task1(input: &str) -> String {
    let (nums, ops) = get_nums_and_operators(input);
    ops.iter()
        .enumerate()
        .map(|(col, operator)| match operator {
            Operator::Add => {
                let mut subtotal = 0;
                for row in &nums {
                    subtotal += row[col];
                }
                subtotal
            }
            Operator::Multiply => {
                let mut subtotal = 1;
                for row in &nums {
                    subtotal *= row[col];
                }
                subtotal
            }
        })
        .sum::<u64>()
        .to_string()
}

pub fn task2(input: &str) -> String {
    let lines: Box<[&str]> = input.lines().collect();
    let num_lines = lines[..(lines.len() - 1)]
        .iter()
        .map(|line| line.as_bytes())
        .collect::<Box<_>>();
    let ops = lines[lines.len() - 1].as_bytes();

    let mut total: u64 = 0;

    // column state
    let mut subtotal = 0;
    let mut op = Operator::Add;

    for (i, op_char) in ops.iter().enumerate() {
        // check if start of new column
        // and add subtotal of previous column to total
        if *op_char == b'+' {
            op = Operator::Add;
            total += subtotal;
            subtotal = 0;
        } else if *op_char == b'*' {
            op = Operator::Multiply;
            total += subtotal;
            subtotal = 1;
        }
        // calculate column number
        let mut num = 0;
        for num_line in &num_lines {
            let digit = num_line[i];

            if digit.is_ascii_digit() {
                num = num * 10 + u64::from(digit - b'0');
            }
        }
        // add column to subtotal
        match op {
            Operator::Add => subtotal += num,
            Operator::Multiply => {
                if num > 0 {
                    subtotal *= num;
                }
            }
        }
    }
    total += subtotal;
    total.to_string()
}
