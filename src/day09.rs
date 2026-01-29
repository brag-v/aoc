use std::collections::VecDeque;

fn is_sum_of_pair(num: u64, previous_nums: &VecDeque<u64>) -> bool {
    for i in 0..previous_nums.len() {
        for j in (i + 1)..previous_nums.len() {
            if previous_nums[i] + previous_nums[j] == num {
                return true;
            }
        }
    }
    false
}

fn first_invalid_number(nums: &[u64], preamble_length: usize) -> u64 {
    let mut previous_nums = VecDeque::with_capacity(preamble_length);
    previous_nums.extend(nums[..preamble_length].iter());
    *nums[preamble_length..]
        .iter()
        .find(|num| {
            if is_sum_of_pair(**num, &previous_nums) {
                previous_nums.pop_front();
                previous_nums.push_back(**num);
                false
            } else {
                true
            }
        })
        .unwrap()
}

pub fn task1_custom_preamble(input: &str, preamble_length: usize) -> String {
    let nums: Box<[u64]> = input.lines().map(|num| num.parse().unwrap()).collect();
    first_invalid_number(&nums, preamble_length).to_string()
}

pub fn task1(input: &str) -> String {
    task1_custom_preamble(input, 25)
}

fn find_contigous_sum(nums: &[u64], target_sum: u64) -> Option<&[u64]> {
    for i in 0..nums.len() {
        let mut acc = nums[i];
        for j in (i + 1)..nums.len() {
            acc += nums[j];
            if acc > target_sum {
                break;
            } else if acc == target_sum {
                return Some(&nums[i..=j]);
            }
        }
    }
    None
}

pub fn task2_custom_preamble(input: &str, preamble_length: usize) -> String {
    let nums: Box<[u64]> = input.lines().map(|num| num.parse().unwrap()).collect();
    let invalid_num = first_invalid_number(&nums, preamble_length);
    match find_contigous_sum(&nums, invalid_num) {
        Some(range) => (range.iter().min().unwrap() + range.iter().max().unwrap()).to_string(),
        None => panic!(),
    }
}

pub fn task2(input: &str) -> String {
    task2_custom_preamble(input, 25)
}
