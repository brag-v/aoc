fn split_count(input: &str) -> (u64, u64) {
    let mut lines = input.lines().map(str::as_bytes);

    let start_line = lines.next().unwrap();
    let width = start_line.len();

    let mut current_beams = &mut vec![0; width];
    let mut next_beams = &mut vec![0; width];

    let mut start = start_line
        .iter()
        .enumerate()
        .find_map(|(i, c)| if *c == b'S' { Some(i) } else { None })
        .unwrap();
    let mut end = start;

    current_beams[start] = 1;

    let mut split_count = 0;
    for line in lines {
        // we're updating the bounds for the next iteration
        #[allow(clippy::mut_range_bound)]
        for i in start..=end {
            if current_beams[i] == 0 {
                continue;
            }
            if line[i] == b'^' {
                split_count += 1;
                if i > 0 {
                    next_beams[i - 1] += current_beams[i];
                    start = start.min(i - 1);
                }
                if i < width - 1 {
                    next_beams[i + 1] += current_beams[i];
                    end = end.max(i + 1);
                }
            } else {
                next_beams[i] += current_beams[i];
            }
            current_beams[i] = 0;
        }
        (next_beams, current_beams) = (current_beams, next_beams);
    }
    (split_count, current_beams.iter().sum())
}

pub fn task1(input: &str) -> String {
    split_count(input).0.to_string()
}

pub fn task2(input: &str) -> String {
    split_count(input).1.to_string()
}
