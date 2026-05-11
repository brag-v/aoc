use std::collections::HashMap;

fn number_at_turn(starting_nums: &str, final_turn: usize) -> usize {
    let mut number_turns: HashMap<usize, usize> = HashMap::new();
    let mut turn = 1;
    let mut prev_num = 0;
    for starting_num in starting_nums.split(',').map(|num| num.parse().unwrap()) {
        number_turns.insert(starting_num, turn);
        turn += 1;
        prev_num = starting_num;
    }
    for turn in turn..=final_turn {
        let number_seen = number_turns.insert(prev_num, turn - 1);
        match number_seen {
            Some(prev_turn) => prev_num = turn - 1 - prev_turn,
            None => prev_num = 0,
        }
    }
    prev_num
}

pub fn task1(input: &str) -> String {
    number_at_turn(input, 2020).to_string()
}

pub fn task2(input: &str) -> String {
    // TODO: takes almost 2 seconds in --release
    // test takes almost a minute
    number_at_turn(input, 30_000_000).to_string()
}
