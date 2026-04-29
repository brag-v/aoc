use phf::phf_map;
use std::{env, error::Error, fs::read_to_string, time::Instant};

#[cfg(test)]
mod test;

mod ac3;
mod geometry;

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
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

type Solver = fn(&str) -> String;

static SOLVERS: phf::Map<&'static str, Solver> = phf_map! {
    "1.1" => day01::task1 as Solver,
    "1.2" => day01::task2 as Solver,
    "2.1" => day02::task1 as Solver,
    "2.2" => day02::task2 as Solver,
    "3.1" => day03::task1 as Solver,
    "3.2" => day03::task2 as Solver,
    "4.1" => day04::task1 as Solver,
    "4.2" => day04::task2 as Solver,
    "5.1" => day05::task1 as Solver,
    "5.2" => day05::task2 as Solver,
    "6.1" => day06::task1 as Solver,
    "6.2" => day06::task2 as Solver,
    "7.1" => day07::task1 as Solver,
    "7.2" => day07::task2 as Solver,
    "8.1" => day08::task1 as Solver,
    "8.2" => day08::task2 as Solver,
    "9.1" => day09::task1 as Solver,
    "9.2" => day09::task2 as Solver,
    "10.1" => day10::task1 as Solver,
    "10.2" => day10::task2 as Solver,
    "11.1" => day11::task1 as Solver,
    "11.2" => day11::task2 as Solver,
    "12.1" => day12::task1 as Solver,
    "12.2" => day12::task2 as Solver,
    "13.1" => day13::task1 as Solver,
    "13.2" => day13::task2 as Solver,
    "14.1" => day14::task1 as Solver,
    "14.2" => day14::task2 as Solver,
    "15.1" => day15::task1 as Solver,
    "15.2" => day15::task2 as Solver,
    "16.1" => day16::task1 as Solver,
    "16.2" => day16::task2 as Solver,
    "17.1" => day17::task1 as Solver,
    "17.2" => day17::task2 as Solver,
    "18.1" => day18::task1 as Solver,
    "18.2" => day18::task2 as Solver,
    "19.1" => day19::task1 as Solver,
    "19.2" => day19::task2 as Solver,
    "20.1" => day20::task1 as Solver,
    "20.2" => day20::task2 as Solver,
    "21.1" => day21::task1 as Solver,
    "21.2" => day21::task2 as Solver,
    "22.1" => day22::task1 as Solver,
    "22.2" => day22::task2 as Solver,
    "23.1" => day23::task1 as Solver,
    "23.2" => day23::task2 as Solver,
    "24.1" => day24::task1 as Solver,
    "24.2" => day24::task2 as Solver,
    "25.1" => day25::task1 as Solver,
    "25.2" => day25::task2 as Solver,
};

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
    let Some(solver) = SOLVERS.get(args[1].as_str()) else {
        return Err(format!("Day {day} task {task} doesn't exist").into());
    };

    let path = format!("./data/day{day:0>2}.txt");

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
