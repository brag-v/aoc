fn max_joltage(line: &str) -> u32 {
    let bytes = line.as_bytes();
    let mut first = bytes[0];
    let mut first_i = 0;
    for (i, c) in bytes[..(bytes.len() - 1)].iter().enumerate().skip(1) {
        if *c > first {
            first = *c;
            first_i = i;
        }
    }
    let mut second = bytes[first_i + 1];
    for c in bytes[(first_i + 2)..].iter() {
        if *c > second {
            second = *c;
        }
    }
    ((first - b'0') * 10 + second - b'0') as u32
}

pub fn task1(input: &str) -> String {
    input
        .lines()
        .map(max_joltage)
        .sum::<u32>()
        .to_string()
}

pub fn task2(_input: &str) -> String {
    todo!("Day 3 task 2")
}
