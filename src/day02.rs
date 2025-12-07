use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};

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
        // neither test nor actual input ever reaches here
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

fn contains_repeats(num: u64, possible_lengths: &[usize]) -> bool {
    let binding = num.to_string();
    let sequence = binding.as_bytes();
    possible_lengths.iter().any(|pattern_len| {
        (*pattern_len..sequence.len()).all(|i| sequence[i] == sequence[i - pattern_len])
    })
}

fn possible_repeat_lengths(start: u64, end: u64) -> Vec<usize> {
    let start_length = start.ilog10() + 1;
    let end_length = end.ilog10() + 1;
    let mut possible_lengths = Vec::new();
    for length in (1..=(end_length / 2)).rev() {
        if (start_length..=end_length).any(|base_length| base_length.is_multiple_of(length)) {
            possible_lengths.push(length as usize);
        }
    }
    possible_lengths
}

pub fn task2(input: &str) -> String {
    // looks very inefficient, but takes 6 ms on real input using optimized build
    get_ranges(input)
        .par_bridge()
        .map(|(start, end)| {
            // TODO: split magnitudes?
            let possible_lengths = possible_repeat_lengths(start, end);
            (start..=end)
                .into_par_iter()
                .filter(|num| contains_repeats(*num, &possible_lengths))
                .sum::<u64>()
        })
        .sum::<u64>()
        .to_string()
}
