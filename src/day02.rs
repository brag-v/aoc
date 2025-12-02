fn get_ranges(input: &str) -> impl Iterator<Item = (u64, u64)> {
    input
        .split(',')
        .map(|range| range.split_once('-').unwrap())
        .map(|(start, end)| (start.parse().unwrap(), end.parse().unwrap()))
}

fn double_num_sum_single_magnitude(start: u64, end: u64) -> u64 {
    let log = start.ilog10();
    if log.is_multiple_of(2) {
        return 0;
    }

    let mid_magnitude = 10_u64.pow(log.div_ceil(2));

    let start_upper = start / mid_magnitude;
    let end_upper = end / mid_magnitude;
    let start_lower = start % mid_magnitude;
    let end_lower = end % mid_magnitude;

    let start_inclusive = start_lower <= start_upper;
    let end_inclusive = end_lower >= end_upper;

    if start_upper == end_upper {
        if start_inclusive && end_inclusive {
            start_upper + start_upper * mid_magnitude
        } else {
            0
        }
    } else {
        let mut total = 0;
        if start_inclusive {
            total += start_upper + start_upper * mid_magnitude;
        }
        for between_upper in (start_upper + 1)..end_upper {
            total += between_upper + between_upper * mid_magnitude;
        }
        if end_inclusive {
            total += end_upper + end_upper * mid_magnitude;
        }
        total
    }
}

fn double_num_sum(range: (u64, u64)) -> u64 {
    let log_low = range.0.ilog10();
    let log_high = range.1.ilog10();
    if log_low == log_high {
        return double_num_sum_single_magnitude(range.0, range.1);
    }

    let mut total = double_num_sum_single_magnitude(range.0, 10_u64.pow(log_low + 1) - 1)
        + double_num_sum_single_magnitude(10_u64.pow(log_high), range.1);
    for mid_log in (log_low + 1)..log_high {
        total += double_num_sum_single_magnitude(10_u64.pow(mid_log), 10_u64.pow(mid_log + 1) - 1);
    }
    total
}

pub fn task1(input: &str) -> String {
    get_ranges(input)
        .map(double_num_sum)
        .sum::<u64>()
        .to_string()
}

pub fn task2(_input: &str) -> String {
    todo!("Day 2 task 2")
}
