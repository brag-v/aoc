use std::collections::HashSet;

#[derive(Debug)]
enum Move {
    NW,
    NE,
    W,
    E,
    SW,
    SE,
}

const fn move_offset(m: &Move) -> (i32, i32) {
    match m {
        Move::NW => (-1, -1),
        Move::NE => (1, -1),
        Move::W => (-2, 0),
        Move::E => (2, 0),
        Move::SW => (-1, 1),
        Move::SE => (1, 1),
    }
}

fn parse_moves(input: &str) -> Vec<Vec<Move>> {
    enum NextMove {
        North,
        Sourth,
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
                's' => next_move = Sourth,
                'e' => {
                    match next_move {
                        North => move_sequence.push(Move::NE),
                        Sourth => move_sequence.push(Move::SE),
                        Default => move_sequence.push(Move::E),
                    }
                    next_move = Default;
                }
                'w' => {
                    match next_move {
                        North => move_sequence.push(Move::NW),
                        Sourth => move_sequence.push(Move::SW),
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

fn perform_moves(input: &str) -> HashSet<(i32, i32)> {
    let moves = parse_moves(input);
    let start_pos = (0, 0);
    let mut flipped_tiles = HashSet::with_capacity(input.lines().count());
    for sequence in &moves {
        // move from start
        let mut pos = start_pos;
        for m in sequence {
            let offset = move_offset(m);
            pos.0 += offset.0;
            pos.1 += offset.1;
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

const ADJACENT_OFFSETS: [(i32, i32); 6] = [(-1, -1), (1, -1), (-2, 0), (2, 0), (-1, 1), (1, 1)];

fn count_flipped_neighbors(pos: &(i32, i32), fillped_tiles: &HashSet<(i32, i32)>) -> usize {
    ADJACENT_OFFSETS
        .iter()
        .filter(|(off_x, off_y)| fillped_tiles.contains(&(pos.0 + off_x, pos.1 + off_y)))
        .count()
}

fn perform_day(tiles_today: &HashSet<(i32, i32)>) -> HashSet<(i32, i32)> {
    let mut tiles_tomorrow = tiles_today.clone();
    let unflipped_tiles: HashSet<(i32, i32)> = tiles_today
        .iter()
        .flat_map(|(tile_x, tile_y)| {
            ADJACENT_OFFSETS
                .iter()
                .map(move |(off_x, off_y)| (tile_x + off_x, tile_y + off_y))
        })
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
    let mut flipped_tiles = perform_moves(input);
    for _day in 0..100 {
        flipped_tiles = perform_day(&flipped_tiles);
    }
    flipped_tiles.len().to_string()
}
