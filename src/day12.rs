use crate::grid::{Direction, Point2D};

pub fn task1(input: &str) -> String {
    let mut direction = Direction::East;
    let mut ship = Point2D::ZERO;
    for line in input.lines() {
        let (dir, count) = line.split_at(1);
        let count = count.parse().unwrap();
        match dir {
            "N" => ship += Direction::North.as_offset() * count,
            "S" => ship += Direction::South.as_offset() * count,
            "E" => ship += Direction::East.as_offset() * count,
            "W" => ship += Direction::West.as_offset() * count,
            "F" => ship += direction.as_offset() * count,
            "L" => {
                for _ in 0..(count / 90) {
                    direction = direction.left();
                }
            }
            "R" => {
                for _ in 0..(count / 90) {
                    direction = direction.right();
                }
            }
            _ => panic!("Invalid direction: {dir}"),
        }
    }
    ship.manhattan_distance(&Point2D::ZERO).to_string()
}

pub fn task2(input: &str) -> String {
    let mut ship = Point2D::ZERO;
    let mut waypoint = Point2D { x: 10, y: -1 };
    for line in input.lines() {
        let (dir, count) = line.split_at(1);
        let count = count.parse().unwrap();
        match dir {
            "N" => waypoint += Direction::North.as_offset() * count,
            "S" => waypoint += Direction::South.as_offset() * count,
            "E" => waypoint += Direction::East.as_offset() * count,
            "W" => waypoint += Direction::West.as_offset() * count,
            "F" => ship += waypoint * count,
            "L" => {
                for _ in 0..(count / 90) {
                    (waypoint.x, waypoint.y) = (waypoint.y, -waypoint.x);
                }
            }
            "R" => {
                for _ in 0..(count / 90) {
                    (waypoint.x, waypoint.y) = (-waypoint.y, waypoint.x);
                }
            }
            _ => panic!("Invalid direction: {dir}"),
        }
    }
    ship.manhattan_distance(&Point2D::ZERO).to_string()
}
