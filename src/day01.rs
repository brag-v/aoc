use itertools::Itertools;

pub fn task1(input: &str) -> String {
    let nums: Box<[u32]> = input.lines().map(|line| line.parse().unwrap()).collect();
    nums.iter()
        .combinations(2)
        .find(|pair| pair[0] + pair[1] == 2020)
        .map(|pair| pair[0] * pair[1])
        .unwrap()
        .to_string()
}

pub fn task2(input: &str) -> String {
    let nums: Box<[u32]> = input.lines().map(|line| line.parse().unwrap()).collect();
    let min = nums.iter().min().unwrap();
    let max = nums.iter().max().unwrap();
    for (i, a) in nums.iter().enumerate() {
        if a + 2 * max < 2020 || a + 2 * min > 2020 {
            continue;
        }
        for (j, b) in nums[(i + 1)..].iter().enumerate() {
            let subsum = a + b;
            if subsum + max < 2020 || subsum + min > 2020 {
                continue;
            }
            for c in nums[(j + 1)..].iter() {
                if subsum + c == 2020 {
                    return (a * b * c).to_string();
                }
            }
        }
    }
    "Not found".to_string()
}
