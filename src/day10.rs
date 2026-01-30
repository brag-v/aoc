pub fn task1(input: &str) -> String {
    let mut nums: Vec<u32> = input.lines().map(|num| num.parse().unwrap()).collect();
    nums.push(0);
    nums.sort_unstable();
    nums.push(*nums.last().unwrap() + 3);
    let (ones, threes) =
        nums.windows(2)
            .map(|pair| pair[1] - pair[0])
            .fold((0, 0), |(ones, threes), difference| {
                if difference == 1 {
                    (ones + 1, threes)
                } else if difference == 3 {
                    (ones, threes + 1)
                } else {
                    (ones, threes)
                }
            });
    (ones * threes).to_string()
}

pub fn task2(_input: &str) -> String {
    todo!("Day 10 task 2")
}
