use std::collections::VecDeque;

fn is_sum_of_pair(num: &u64, previous_nums: &VecDeque<u64>) -> bool {
    for i in 0..previous_nums.len() {
        for j in (i + 1)..previous_nums.len() {
            if previous_nums[i] + previous_nums[j] == *num {
                return true;
            }
        }
    }
    false
}

fn first_invalid_number(input: &str, preamble_length: usize) -> u64 {
    let mut nums = input.lines().map(|num| num.parse().unwrap());
    let mut preamble: VecDeque<u64> = nums.by_ref().take(preamble_length).collect();
    nums.find(|num| {
        if !is_sum_of_pair(num, &preamble) {
            true
        } else {
            preamble.pop_front();
            preamble.push_back(*num);
            false
        }
    })
    .unwrap()
}

pub fn task1_custom_preamble(input: &str, preamble_length: usize) -> String {
    first_invalid_number(input, preamble_length).to_string()
}

pub fn task1(input: &str) -> String {
    task1_custom_preamble(input, 25)
}

pub fn task2_custom_preamble(input: &str, preamble_length: usize) -> String {
    let num = first_invalid_number(input, preamble_length);
    let sequence: Box<[u64]> = input.lines().map(|num| num.parse().unwrap()).collect();
    for i in 0..sequence.len() {
        let mut acc = sequence[i];
        for j in (i + 1)..sequence.len() {
            acc += sequence[j];
            if acc > num {
                break;
            } else if acc == num {
                return (sequence[i..=j].iter().min().unwrap()
                    + sequence[i..=j].iter().max().unwrap())
                .to_string();
            }
        }
    }
    panic!()
}

pub fn task2(input: &str) -> String {
    task2_custom_preamble(input, 25)
}
