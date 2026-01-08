fn seat_id(seat: &str) -> usize {
    seat.chars()
        .map(|partition| match partition {
            'F' | 'L' => 0,
            'B' | 'R' => 1,
            _ => panic!(),
        })
        .fold(0, |acc, partition| acc * 2 + partition)
}

pub fn task1(input: &str) -> String {
    input.lines().map(seat_id).max().unwrap().to_string()
}

pub fn task2(input: &str) -> String {
    let mut taken_seats = vec![false; 128 * 8];
    input
        .lines()
        .map(seat_id)
        .for_each(|id| taken_seats[id] = true);
    taken_seats
        .iter()
        .enumerate()
        .skip_while(|(_, taken)| !**taken)
        .find(|(_, taken)| !**taken)
        .unwrap()
        .0
        .to_string()
}
