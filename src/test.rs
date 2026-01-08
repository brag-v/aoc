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

#[test]
fn test_day2_task1() {
    assert_task!(
        day02::task1,
        "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc",
        "2"
    )
}

#[test]
fn test_day2_task2() {
    assert_task!(
        day02::task2,
        "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc",
        "1"
    )
}

#[test]
fn test_day3_task1() {
    assert_task!(
        day03::task1,
        "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#",
        "7"
    )
}


#[test]
fn test_day3_task2() {
    assert_task!(
        day03::task2,
        "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#",
        "336"
    )
}


#[test]
fn test_day4_task1() {
    assert_task!(
        day04::task1,
"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in",
        "2"
    )
}

#[test]
fn test_day4_task2() {
    assert_task!(
        day04::task2,
        "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007

pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:09315471",
        "3"
    )
}

#[test]
fn test_day5_task1() {
    assert_task!(
        day05::task1,
        "BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL",
        "820"
    )
}

#[test]
fn test_day6_task1() {
    assert_task!(
        day06::task1,
"abc

a
b
c

ab
ac

a
a
a
a

b",
        "11"
    )
}

#[test]
fn test_day6_task2() {
    assert_task!(
        day06::task2,
"abc

a
b
c

ab
ac

a
a
a
a

b",
        "6"
    )
}
