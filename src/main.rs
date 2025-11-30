use std::{env, error::Error, fs::read_to_string, time::Instant};

#[cfg(test)]
mod test;

mod grid;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    // Unhapppy paths for wrong arguments or IO errors
    if args.len() != 2 {
        return Err("Please provide a task to solve".into());
    }

    let Some((day, task)) = args[1].split_once('.') else {
        return Err("Please provide a task in the format <day number>.<task number>".into());
    };

    // Select solver for the provided task
    let solver = match (day, task) {
        ("1", "1") => day01::task1,
        ("1", "2") => day01::task2,
        ("2", "1") => day02::task1,
        ("2", "2") => day02::task2,
        ("3", "1") => day03::task1,
        ("3", "2") => day03::task2,
        ("4", "1") => day04::task1,
        ("4", "2") => day04::task2,
        ("5", "1") => day05::task1,
        ("5", "2") => day05::task2,
        ("6", "1") => day06::task1,
        ("6", "2") => day06::task2,
        ("7", "1") => day07::task1,
        ("7", "2") => day07::task2,
        ("8", "1") => day08::task1,
        ("8", "2") => day08::task2,
        ("9", "1") => day09::task1,
        ("9", "2") => day09::task2,
        ("10", "1") => day10::task1,
        ("10", "2") => day10::task2,
        ("11", "1") => day11::task1,
        ("11", "2") => day11::task2,
        ("12", "1") => day12::task1,
        ("12", "2") => day12::task2,
        _ => return Err(format!("Day {day} task {task} is doesn't exist").into()),
    };

    let path = format!("./data/day{day}.txt");

    // Solve task, and measure runtime
    let start_time = Instant::now();
    let input = read_to_string(path)?;
    let result = solver(input.trim_end());
    let runtime = start_time.elapsed();

    if result.contains('\n') {
        // if result is multiline, we might not want
        // the shift on the first line from "Result: "
        println!("Result:\n{result}");
    } else {
        println!("Result: {result}");
    }
    println!("Elapsed time: {:.3} ms", runtime.as_secs_f64() * 1000.);

    Ok(())
}
