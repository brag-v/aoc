use std::collections::HashSet;

use crate::geometry::Point2D;

#[derive(Debug)]
enum Move {
    NW,
    NE,
    W,
    E,
    SW,
    SE,
}

const HEXAGONAL_OFFSETS: [Point2D; 6] = [
    Point2D { x: 0, y: -1 },
    Point2D { x: 1, y: -1 },
    Point2D { x: -1, y: 0 },
    Point2D { x: 1, y: 0 },
    Point2D { x: -1, y: 1 },
    Point2D { x: 0, y: 1 },
];

const fn move_offset(m: &Move) -> Point2D {
    match m {
        Move::NW => HEXAGONAL_OFFSETS[0],
        Move::NE => HEXAGONAL_OFFSETS[1],
        Move::W => HEXAGONAL_OFFSETS[2],
        Move::E => HEXAGONAL_OFFSETS[3],
        Move::SW => HEXAGONAL_OFFSETS[4],
        Move::SE => HEXAGONAL_OFFSETS[5],
    }
}

fn parse_moves(input: &str) -> Vec<Vec<Move>> {
    enum NextMove {
        North,
        South,
        Default,
    }
    use NextMove::*;
    let mut move_sequences = Vec::with_capacity(input.lines().count());
    for line in input.lines() {
        let mut move_sequence = Vec::with_capacity(line.len()); // over-estimate
        let mut next_move = Default;
        for c in line.chars() {
            match c {
                'n' => next_move = North,
                's' => next_move = South,
                'e' => {
                    match next_move {
                        North => move_sequence.push(Move::NE),
                        South => move_sequence.push(Move::SE),
                        Default => move_sequence.push(Move::E),
                    }
                    next_move = Default;
                }
                'w' => {
                    match next_move {
                        North => move_sequence.push(Move::NW),
                        South => move_sequence.push(Move::SW),
                        Default => move_sequence.push(Move::W),
                    }
                    next_move = Default;
                }
                _ => panic!("Invalid move"),
            }
        }
        move_sequences.push(move_sequence);
    }
    move_sequences
}

fn perform_moves(input: &str) -> HashSet<Point2D> {
    let moves = parse_moves(input);
    let start_pos = Point2D::ZERO;
    let mut flipped_tiles = HashSet::with_capacity(input.lines().count());
    for sequence in &moves {
        // move from start
        let mut pos = start_pos;
        for m in sequence {
            pos += move_offset(m);
        }
        // flip tile at destination
        if !flipped_tiles.insert(pos) {
            flipped_tiles.remove(&pos);
        }
    }
    flipped_tiles
}

pub fn task1(input: &str) -> String {
    perform_moves(input).len().to_string()
}

fn count_flipped_neighbors(pos: &Point2D, fillped_tiles: &HashSet<Point2D>) -> usize {
    HEXAGONAL_OFFSETS
        .iter()
        .filter(|offset| fillped_tiles.contains(&(*pos + **offset)))
        .count()
}

fn perform_day(tiles_today: &HashSet<Point2D>) -> HashSet<Point2D> {
    let mut tiles_tomorrow = tiles_today.clone();
    let unflipped_tiles: HashSet<Point2D> = tiles_today
        .iter()
        .flat_map(|tile| HEXAGONAL_OFFSETS.iter().map(move |offset| *tile + *offset))
        .filter(|pos| !tiles_today.contains(pos))
        .collect();
    for pos in tiles_today.iter() {
        let flipped_neighbours = count_flipped_neighbors(pos, tiles_today);
        if flipped_neighbours == 0 || flipped_neighbours > 2 {
            tiles_tomorrow.remove(&pos);
        }
    }
    for pos in unflipped_tiles.iter() {
        let flipped_neighbours = count_flipped_neighbors(pos, tiles_today);
        if flipped_neighbours == 2 {
            tiles_tomorrow.insert(*pos);
        }
    }
    tiles_tomorrow
}

pub fn task2(input: &str) -> String {
    // TODO: change from hashset to array-based container
    // fixed sized, or dynamic?
    let mut flipped_tiles = perform_moves(input);
    for _day in 0..100 {
        flipped_tiles = perform_day(&flipped_tiles);
    }
    flipped_tiles.len().to_string()
}
