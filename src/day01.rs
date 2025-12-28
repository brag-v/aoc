pub fn task1(input: &str) -> String {
    let mut dial: i32 = 50;
    let mut zero_count = 0;
    for line in input.lines() {
        let direction = line.as_bytes()[0];
        let distance: i32 = line[1..].parse().unwrap();
        match direction {
            b'L' => dial = (dial - distance).rem_euclid(100),
            b'R' => dial = (dial + distance).rem_euclid(100),
            _ => panic!(),
        }
        if dial == 0 {
            zero_count += 1;
        }
    }
    zero_count.to_string()
}

pub fn task2(input: &str) -> String {
    let mut dial: i32 = 50;
    let mut zero_count = 0;
    for line in input.lines() {
        let direction = line.bytes().next().unwrap();
        let distance: i32 = line[1..].parse().unwrap();
        let was_zero = dial == 0;
        match direction {
            b'L' => dial -= distance,
            b'R' => dial += distance,
            _ => panic!(),
        }
        if dial <= 0 {
            zero_count += -dial / 100 + 1;
            if was_zero {
                zero_count -= 1;
            }
        } else if dial > 99 {
            zero_count += dial / 100;
        }
        dial = dial.rem_euclid(100);
    }
    zero_count.to_string()
}
