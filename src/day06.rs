use std::collections::{HashMap, HashSet};

pub fn task1(input: &str) -> String {
    input
        .split("\n\n")
        .map(|group| {
            group
                .chars()
                .filter(char::is_ascii_alphabetic)
                .collect::<HashSet<char>>()
                .len()
        })
        .sum::<usize>()
        .to_string()
}

pub fn task2(input: &str) -> String {
    input
        .split("\n\n")
        .map(|group| {
            let mut answer_counts = HashMap::new();
            let n = group.lines().count();
            group
                .chars()
                .filter(char::is_ascii_alphabetic)
                .for_each(|answer| *answer_counts.entry(answer).or_insert(0) += 1);
            answer_counts.values().filter(|count| **count == n).count()
        })
        .sum::<usize>()
        .to_string()
}
