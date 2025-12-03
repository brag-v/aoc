fn max_subseqence(digits: &str, digit_count: u32) -> u64 {
    let digits = digits.as_bytes();
    let mut total = 0;
    let mut progress = 0;
    for digit in (0..digit_count).rev() {
        let mut max = 0;
        for (i, digit) in digits[..(digits.len() - digit as usize)]
            .iter()
            .enumerate()
            .skip(progress)
        {
            if *digit > max {
                max = *digit;
                progress = i + 1;
            }
        }
        total += 10_u64.pow(digit) * u64::from(max - b'0');
    }
    total
}

fn solve_tasks(input: &str, battery_count: u32) -> String {
    input
        .lines()
        .map(|digits| max_subseqence(digits, battery_count))
        .sum::<u64>()
        .to_string()
}

pub fn task1(input: &str) -> String {
    solve_tasks(input, 2)
}

pub fn task2(input: &str) -> String {
    solve_tasks(input, 12)
}
