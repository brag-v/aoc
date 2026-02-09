pub fn task1(input: &str) -> String {
    let sections: Box<[&str]> = input.split("\n\n").collect();
    let valid_ranges: Box<[(u32, u32)]> = sections[0]
        .lines()
        .flat_map(|line| {
            // cleaner with regex?
            let ranges = line.split_once(": ").unwrap().1;
            let words: Box<[&str]> = ranges.split(&[' ', '-']).collect();
            [
                (words[0].parse().unwrap(), words[1].parse().unwrap()),
                (words[3].parse().unwrap(), words[4].parse().unwrap()),
            ]
        })
        .collect();
    let nearby_tickets = sections[2];
    nearby_tickets
        .lines()
        .skip(1)
        .flat_map(|ticket| {
            ticket
                .split(',')
                .map(|value| value.parse().unwrap())
                .filter(|value| {
                    !valid_ranges.iter().any(|(lower_bound, upper_bound)| {
                        (lower_bound..=upper_bound).contains(&value)
                    })
                })
        })
        .sum::<u32>()
        .to_string()
}

pub fn task2(_input: &str) -> String {
    todo!("Day 1 task 2")
}
