use crate::grid::{ADJACENT_WITH_DIAGONAL_OFFSETS, Map, Point2D};

// TODO: calculate seat neighbors during construction,
// which us used to iterate the plane, rather than passing the update rule

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Tile {
    Floor,
    Empty,
    Occupied,
}

fn new_seat_immediate_neighbors(pos: Point2D, plane: &Map<Tile>) -> Tile {
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

fn iterate_plane(
    prev_plane: &Map<Tile>,
    next_plane: &mut Map<Tile>,
    next_seat_rule: &impl Fn(Point2D, &Map<Tile>) -> Tile,
) -> bool {
    let mut changed = false;
    for y in 0..prev_plane.height() {
        for x in 0..prev_plane.width() {
            let pos = Point2D {
                x: x as isize,
                y: y as isize,
            };
            let new_seat = next_seat_rule(pos, prev_plane);
            if new_seat != prev_plane[pos] {
                changed = true;
            }
            next_plane[pos] = new_seat;
        }
    }
    changed
}

fn stable_occupancy(input: &str, next_seat_rule: &impl Fn(Point2D, &Map<Tile>) -> Tile) -> String {
    let mut plane = &mut Map::try_from_str(input, |tile| match tile {
        'L' => Some(Tile::Empty),
        '.' => Some(Tile::Floor),
        _ => None,
    })
    .unwrap();
    let mut next_plane = &mut Map::filled_with(Tile::Empty, plane.width(), plane.height());
    while iterate_plane(plane, next_plane, next_seat_rule) {
        (plane, next_plane) = (next_plane, plane);
    }
    plane
        .rows()
        .flat_map(|row| row.iter())
        .filter(|tile| matches!(tile, Tile::Occupied))
        .count()
        .to_string()
}

pub fn task1(input: &str) -> String {
    stable_occupancy(input, &new_seat_immediate_neighbors)
}

fn sees_occupied(start: &Point2D, direction: &Point2D, plane: &Map<Tile>) -> bool {
    let mut pos = *start + *direction;
    while plane.contains(&pos) {
        match plane[pos] {
            Tile::Floor => (),
            Tile::Empty => return false,
            Tile::Occupied => return true,
        }
        pos = pos + *direction;
    }
    false
}

fn new_seat_visible_neighbors(pos: Point2D, plane: &Map<Tile>) -> Tile {
    match plane[pos] {
        Tile::Floor => Tile::Floor,
        Tile::Empty => {
            if ADJACENT_WITH_DIAGONAL_OFFSETS
                .iter()
                .any(|direction| sees_occupied(&pos, direction, plane))
            {
                Tile::Empty
            } else {
                Tile::Occupied
            }
        }
        Tile::Occupied => {
            if ADJACENT_WITH_DIAGONAL_OFFSETS
                .iter()
                .filter(|direction| sees_occupied(&pos, direction, plane))
                .count()
                >= 5
            {
                Tile::Empty
            } else {
                Tile::Occupied
            }
        }
    }
}

pub fn task2(input: &str) -> String {
    stable_occupancy(input, &new_seat_visible_neighbors)
}
