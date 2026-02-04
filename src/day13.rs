pub fn task1(input: &str) -> String {
    let (start_time, busses) = input.split_once('\n').unwrap();
    let start_time: u32 = start_time.parse().unwrap();
    let first_id = busses
        .split(',')
        .filter_map(|freq| freq.parse().ok())
        .min_by_key(|freq| freq - start_time % freq)
        .unwrap();
    (first_id * (first_id - start_time % first_id)).to_string()
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    // euclids algorithm
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

pub fn task2(input: &str) -> String {
    let busses: Box<[(usize, usize)]> = input
        .split_once('\n')
        .unwrap()
        .1
        .split(',')
        .enumerate()
        .filter_map(|(offset, freq)| freq.parse().ok().map(|freq| (offset, freq)))
        .collect();

    let mut jump_size = 1;
    let mut time = 0;
    for (offset, freq) in &busses {
        // solve for one frequency at a time
        while (time + offset) % freq != 0 {
            time += jump_size;
        }

        // jump_size is lcm of all solved frequencies
        // this checks all valid times for those frequencies, and jumps over the rest
        jump_size = lcm(jump_size, *freq);
    }
    time.to_string()
}
