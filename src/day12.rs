use crate::grid::Map;

struct Gift {}

struct Region {
    map: Map<bool>,
    requirements: Vec<u8>,
}

fn parse_gifts_and_regions(input: &str) -> (Vec<Gift>, Vec<Region>) {
    let parts: Box<[&str]> = input.split("\n\n").collect();

    let gifts = parts[..(parts.len() - 1)]
        .iter()
        .map(|_part| Gift {})
        .collect();
    let regions = parts
        .last()
        .unwrap()
        .lines()
        .map(|line| line.split_once(": ").unwrap())
        .map(|(size, requirements)| {
            let (width, height) = size.split_once('x').unwrap();
            let requirements = requirements
                .split(' ')
                .map(|count| count.parse().unwrap())
                .collect();
            Region {
                map: Map::filled_with(false, width.parse().unwrap(), height.parse().unwrap()),
                requirements,
            }
        })
        .collect();
    (gifts, regions)
}

fn fits_gifts(region: &Region, _gifts: &[Gift]) -> bool {
    region.map.height() * region.map.width()
        <= region
            .requirements
            .iter()
            .map(|count| *count as usize)
            .sum::<usize>()
            * 9
}

pub fn task1(input: &str) -> String {
    let (gifts, mut regions) = parse_gifts_and_regions(input);
    regions
        .iter_mut()
        .filter(|region| fits_gifts(region, &gifts))
        .count()
        .to_string()
}

pub fn task2(_input: &str) -> String {
    todo!("Day 12 task 2")
}
