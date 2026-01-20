enum Operation {
    Nop,
    Acc,
    Jmp,
}

struct Instruction {
    operation: Operation,
    argument: i32,
}

fn to_instruction(line: &str) -> Instruction {
    let (op, arg) = line.split_once(' ').unwrap();
    Instruction {
        operation: match op {
            "nop" => Operation::Nop,
            "acc" => Operation::Acc,
            "jmp" => Operation::Jmp,
            _ => panic!("invalid instruction: {op}"),
        },
        argument: arg.parse().unwrap(),
    }
}

enum ProgramResult {
    Finishes(i32),
    Loops(i32),
    Invalid,
}

fn run_program(code: &[Instruction]) -> ProgramResult {
    let mut line = 0;
    let mut acc = 0;
    let mut visited = vec![false; code.len()];
    while line < code.len() && !visited[line] {
        visited[line] = true;
        let instruction = &code[line];
        match instruction.operation {
            Operation::Nop => line += 1,
            Operation::Acc => {
                acc += instruction.argument;
                line += 1
            }
            Operation::Jmp => {
                if instruction.argument.is_negative() {
                    line -= instruction.argument.unsigned_abs() as usize
                } else {
                    line += instruction.argument as usize
                }
            }
        }
    }

    if (0..code.len()).contains(&line) {
        ProgramResult::Loops(acc)
    } else if line == code.len() {
        ProgramResult::Finishes(acc)
    } else {
        ProgramResult::Invalid
    }
}

pub fn task1(input: &str) -> String {
    let code: Vec<Instruction> = input.lines().map(to_instruction).collect();
    if let ProgramResult::Loops(acc) = run_program(&code) {
        acc.to_string()
    } else {
        panic!()
    }
}

fn correct_program(code: &mut [Instruction]) -> i32 {
    for i in 0..code.len() {
        match code[i].operation {
            Operation::Nop => code[i].operation = Operation::Jmp,
            Operation::Jmp => code[i].operation = Operation::Nop,
            Operation::Acc => continue,
        }
        if let ProgramResult::Finishes(acc) = run_program(code) {
            return acc;
        }
        match code[i].operation {
            Operation::Nop => code[i].operation = Operation::Jmp,
            Operation::Jmp => code[i].operation = Operation::Nop,
            Operation::Acc => unreachable!(),
        }
    }
    panic!("Non recoverable program")
}

pub fn task2(input: &str) -> String {
    let mut code: Vec<Instruction> = input.lines().map(to_instruction).collect();
    correct_program(&mut code).to_string()
}
