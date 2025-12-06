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

fn valid_id_count(ids: &[u64], ranges: &[(u64, u64)]) -> u64 {
    // TODO: sort range and do some kind of binary search?
    // O(r*log r + i*log r) vs O(i*r)
    let mut count = 0;
    for id in ids {
        for range in ranges {
            if range.0 <= *id && *id <= range.1 {
                count += 1;
                break;
            }
        }
    }
    count
}

pub fn task1(input: &str) -> String {
    let (ranges, ids) = get_ranges_and_ids(input);
    valid_id_count(&ids, &ranges).to_string()
}

fn possible_valid_id_count(ranges: &mut [(u64, u64)]) -> u64 {
    ranges.sort_unstable_by_key(|range| range.0);
    let mut progress = 0;
    let mut count = 0;
    for range in ranges {
        if progress < range.0 {
            // add whole range
            count += range.1 - range.0 + 1;
            progress = range.1;
        } else if progress < range.1 {
            // add rest of range
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
