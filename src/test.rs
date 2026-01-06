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
"1721
979
366
299
675
1456",
        "514579"
    )
}


#[test]
fn test_day1_task2() {
    assert_task!(
        day01::task2,
"1721
979
366
299
675
1456",
        "241861950"
    )
}


