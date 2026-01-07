fn count_hits(forest: &str, x_vel: usize, y_vel: usize) -> u32 {
    let mut x = 0;
    let mut hits = 0;
    for line in forest.lines().step_by(y_vel) {
        let bytes = line.as_bytes();
        if bytes[x] == b'#' {
            hits += 1;
        }
        x = (x + x_vel) % bytes.len();
    }
    hits
}

pub fn task1(input: &str) -> String {
    count_hits(input, 3, 1).to_string()
}

pub fn task2(input: &str) -> String {
    let mut product = 1;
    for (x_vel, y_vel) in [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)] {
        product *= count_hits(input, x_vel, y_vel);
    }
    product.to_string()
}
