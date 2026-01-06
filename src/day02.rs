fn sled_is_valid(line: &str) -> bool {
    let (condition, password) = line.split_once(": ").unwrap();
    let (range, char) = condition.split_once(' ').unwrap();
    let char = char.as_bytes()[0];
    let (min, max) = range.split_once('-').unwrap();
    let min: usize = min.parse().unwrap();
    let max: usize = max.parse().unwrap();
    let count = password.as_bytes().iter().filter(|c| char == **c).count();
    min <= count && count <= max
}

pub fn task1(input: &str) -> String {
    input
        .lines()
        .filter(|password| sled_is_valid(password))
        .count()
        .to_string()
}

fn toboggan_is_valid(line: &str) -> bool {
    let (condition, password) = line.split_once(": ").unwrap();
    let password = password.as_bytes();
    let (positions, char) = condition.split_once(' ').unwrap();
    let char = char.as_bytes()[0];
    let (first, second) = positions.split_once('-').unwrap();
    let first: usize = first.parse().unwrap();
    let second: usize = second.parse().unwrap();
    (password[first - 1] == char) ^ (password[second - 1] == char)
}

pub fn task2(input: &str) -> String {
    input
        .lines()
        .filter(|password| toboggan_is_valid(password))
        .count()
        .to_string()
}
