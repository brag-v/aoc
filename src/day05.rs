fn get_ranges_and_ids(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let (range_input, id_input) = input.split_once("\n\n").unwrap();
    let ranges = range_input
        .split('\n')
        .map(|line| line.split_once('-').unwrap())
        .map(|(start, end)| (start.parse().unwrap(), end.parse().unwrap()))
        .collect();
    let ids = id_input.split('\n').map(|id| id.parse().unwrap()).collect();
    (ranges, ids)
}

pub fn task1(input: &str) -> String {
    let (ranges, ids) = get_ranges_and_ids(input);
    let mut count = 0;
    for id in ids {
        for range in &ranges {
            if range.0 <= id && id <= range.1 {
                count += 1;
                break;
            }
        }
    }
    count.to_string()
}

fn possible_valid_id_count(ranges: &mut [(u64, u64)]) -> u64 {
    ranges.sort_unstable();
    let mut progress = 0;
    let mut count = 0;
    for range in ranges {
        if progress < range.0 {
            count += 1;
            progress = range.0;
        }
        if progress < range.1 {
            count += range.1 - progress;
            progress = range.1;
        }
    }
    count
}

pub fn task2(input: &str) -> String {
    let (mut ranges, _) = get_ranges_and_ids(input);
    possible_valid_id_count(&mut ranges).to_string()
}
