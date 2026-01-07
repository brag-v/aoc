pub fn task1(input: &str) -> String {
    input
        .split("\n\n")
        .filter(|batch| {
            batch
                .split_ascii_whitespace()
                .map(|item| item.split_once(':').unwrap().0)
                .filter(|key| *key != "cid")
                .count()
                == 7
        })
        .count()
        .to_string()
}

fn validate_field(key: &str, value: &str) -> bool {
    match key {
        "byr" => {
            let year: u16 = value.parse().unwrap();
            (1920..=2002).contains(&year)
        }
        "iyr" => {
            let year: u16 = value.parse().unwrap();
            (2010..=2020).contains(&year)
        }
        "eyr" => {
            let year: u16 = value.parse().unwrap();
            (2020..=2030).contains(&year)
        }
        "hgt" => {
            let height: u8 = value.trim_end_matches(char::is_alphabetic).parse().unwrap();
            let unit = value.trim_start_matches(char::is_numeric);
            match unit {
                "cm" => (150..=193).contains(&height),
                "in" => (59..=76).contains(&height),
                _ => false,
            }
        }
        "hcl" => {
            value.starts_with('#')
                && value.len() == 7
                && value.chars().skip(1).all(|c| c.is_ascii_hexdigit())
        }
        "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value),
        "pid" => value.len() == 9 && value.chars().all(|c| c.is_numeric()),
        "cid" => true,
        _ => false,
    }
}

pub fn task2(input: &str) -> String {
    input
        .split("\n\n")
        .filter(|batch| {
            let mut count = 0;
            batch
                .split_ascii_whitespace()
                .map(|item| item.split_once(':').unwrap())
                .filter(|(key, _)| *key != "cid")
                .all(|(key, value)| {
                    count += 1;
                    validate_field(key, value)
                })
                && count == 7
        })
        .count()
        .to_string()
}
