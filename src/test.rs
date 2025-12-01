use super::*;

macro_rules! assert_task {
    ($task:path, $input:literal, $expected:literal) => {{
        let solution = $task($input);
        assert_eq!(solution, $expected.to_string());
    }};
}

#[test]
fn test_day1_task1() {
    assert_task!(
        day01::task1,
        "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82",
        "3"
    )
}

#[test]
fn test_day1_task2() {
    assert_task!(
        day01::task2,
        "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82",
        "6"
    )
}
