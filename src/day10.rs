fn parse_adapters(input: &str) -> Vec<u32> {
    let mut nums: Vec<u32> = input.lines().map(|num| num.parse().unwrap()).collect();
    nums.push(0);
    nums.sort_unstable();
    nums.push(*nums.last().unwrap() + 3);
    nums
}

pub fn task1(input: &str) -> String {
    let nums = parse_adapters(input);
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

pub fn task2(input: &str) -> String {
    let adapters = parse_adapters(input);
    let n = adapters.len();
    let mut possible_arrangements: Vec<u64> = vec![0; n];
    // base case: last adapter (output) has 1 trivial arrangment (itself)
    *possible_arrangements.last_mut().unwrap() = 1;
    // buttom up calculation:
    for i in (0..(n - 1)).rev() {
        // possible arrangment from adapter i is the sum of all 
        // possible arrangments from adapters reachable from adapter i 
        // (within 3 joltage levels)
        for j in (i + 1)..n {
            if adapters[j] > adapters[i] + 3 {
                break;
            }
            possible_arrangements[i] += possible_arrangements[j]
        }
    }
    // return possible arrangmetns from first adapter (input)
    possible_arrangements[0].to_string()
}
