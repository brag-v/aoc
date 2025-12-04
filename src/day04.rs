use crate::grid::{Map, Point2D};

fn count_accessable(map: &Map<bool>) -> usize {
    let mut count = 0;
    for y in 0..map.height() {
        for x in 0..map.width() {
            let pos = Point2D {
                x: x as isize,
                y: y as isize,
            };
            if map[pos]
                && pos
                    .adjecent_with_diagonals(map.width(), map.height())
                    .filter(|adj| map[*adj])
                    .count()
                    < 4
            {
                count += 1;
            }
        }
    }
    count
}

pub fn task1(input: &str) -> String {
    let map = Map::try_from_str(input, |c| match c {
        '.' => Some(false),
        '@' => Some(true),
        _ => None,
    })
    .unwrap();
    count_accessable(&map).to_string()
}

fn remove_and_count(map: &mut Map<bool>) -> usize {
    let mut count = 0;
    loop {
        let mut changed = false;
        for y in 0..map.height() {
            for x in 0..map.width() {
                let pos = Point2D {
                    x: x as isize,
                    y: y as isize,
                };
                if map[pos]
                    && pos
                        .adjecent_with_diagonals(map.width(), map.height())
                        .filter(|adj| map[*adj])
                        .count()
                        < 4
                {
                    map[pos] = false;
                    changed = true;
                    count += 1;
                }
            }
        }
        if !changed {
            return count;
        }
    }
}

pub fn task2(input: &str) -> String {
    let mut map = Map::try_from_str(input, |c| match c {
        '.' => Some(false),
        '@' => Some(true),
        _ => None,
    })
    .unwrap();
    remove_and_count(&mut map).to_string()
}
