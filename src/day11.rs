use crate::grid::{Map, Point2D};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Tile {
    Floor,
    Empty,
    Occupied,
}

fn new_seat(pos: Point2D, plane: &Map<Tile>) -> Tile {
    match plane[pos] {
        Tile::Floor => Tile::Floor,
        Tile::Empty => {
            if pos
                .adjecent_with_diagonals(plane.width(), plane.height())
                .any(|adj| matches!(plane[adj], Tile::Occupied))
            {
                Tile::Empty
            } else {
                Tile::Occupied
            }
        }
        Tile::Occupied => {
            if pos
                .adjecent_with_diagonals(plane.width(), plane.height())
                .filter(|adj| matches!(plane[*adj], Tile::Occupied))
                .count()
                >= 4
            {
                Tile::Empty
            } else {
                Tile::Occupied
            }
        }
    }
}

fn iterate_plane(prev_plane: &Map<Tile>, next_plane: &mut Map<Tile>) -> bool {
    let mut changed = false;
    for y in 0..prev_plane.height() {
        for x in 0..prev_plane.width() {
            let pos = Point2D {
                x: x as isize,
                y: y as isize,
            };
            let new_seat = new_seat(pos, prev_plane);
            if new_seat != prev_plane[pos] {
                changed = true;
            }
            next_plane[pos] = new_seat;
        }
    }
    changed
}

pub fn task1(input: &str) -> String {
    let mut plane = &mut Map::try_from_str(input, |tile| match tile {
        'L' => Some(Tile::Empty),
        '.' => Some(Tile::Floor),
        _ => None,
    })
    .unwrap();
    let mut next_plane = &mut Map::filled_with(Tile::Empty, plane.width(), plane.height());
    while iterate_plane(plane, next_plane) {
        (plane, next_plane) = (next_plane, plane);
    }
    plane
        .rows()
        .flat_map(|row| row.iter())
        .filter(|tile| matches!(tile, Tile::Occupied))
        .count()
        .to_string()
}

pub fn task2(_input: &str) -> String {
    todo!("Day 11 task 2")
}
