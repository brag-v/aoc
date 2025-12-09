fn parse_nums(input: &str) -> Box<[(u64, u64)]> {
    input
        .lines()
        .map(|line| line.split_once(',').unwrap())
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .collect::<Box<[(u64, u64)]>>()
}

pub fn task1(input: &str) -> String {
    let nums = parse_nums(input);
    nums.iter()
        .enumerate()
        .flat_map(|(i, lhs)| nums[(i + 1)..].iter().map(move |rhs| (lhs, rhs)))
        .map(|(lhs, rhs)| (lhs.0.abs_diff(rhs.0) + 1) * (lhs.1.abs_diff(rhs.1) + 1))
        .max()
        .unwrap()
        .to_string()
}

pub fn task2(_input: &str) -> String {
    todo!("Day 9 task 2")
}
