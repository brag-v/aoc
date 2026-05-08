//! Structs and measures for dealing with square grids,
//! a common format for advent of code tasks
#![allow(dead_code)]

use std::{
    fmt::{self, Display},
    mem,
    ops::{Add, AddAssign, Index, IndexMut, Mul, Sub},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Hash)]
pub struct Point2D {
    pub x: i64,
    pub y: i64,
}

impl Point2D {
    pub const ZERO: Point2D = Point2D { x: 0, y: 0 };

    pub const ADJACENT_OFFSETS: [Point2D; 4] = [
        Point2D { x: -1, y: 0 },
        Point2D { x: 0, y: -1 },
        Point2D { x: 0, y: 1 },
        Point2D { x: 1, y: 0 },
    ];

    pub const ADJACENT_WITH_DIAGONAL_OFFSETS: [Point2D; 8] = [
        Point2D { x: -1, y: 0 },
        Point2D { x: 0, y: -1 },
        Point2D { x: 0, y: 1 },
        Point2D { x: 1, y: 0 },
        Point2D { x: -1, y: -1 },
        Point2D { x: -1, y: 1 },
        Point2D { x: 1, y: -1 },
        Point2D { x: 1, y: 1 },
    ];

    pub fn adjecent_with_offsets(
        &self,
        width_bound: usize,
        height_bound: usize,
        offsets: &[Point2D],
    ) -> impl Iterator<Item = Point2D> {
        let width_bound = width_bound as i64;
        let height_bound = height_bound as i64;
        offsets
            .iter()
            .map(|offset| *self + *offset)
            // TODO: move filter to other function?
            .filter(move |adj| {
                adj.x >= 0 && adj.x < width_bound && adj.y >= 0 && adj.y < height_bound
            })
    }

    /// return a list of adjecent coordinates within the bounds (0..width) and (0..height)
    pub fn adjacent(
        &self,
        width_bound: usize,
        height_bound: usize,
    ) -> impl Iterator<Item = Point2D> {
        self.adjecent_with_offsets(width_bound, height_bound, &Self::ADJACENT_OFFSETS)
    }

    pub fn adjecent_with_diagonals(
        &self,
        width_bound: usize,
        height_bound: usize,
    ) -> impl Iterator<Item = Point2D> {
        self.adjecent_with_offsets(
            width_bound,
            height_bound,
            &Self::ADJACENT_WITH_DIAGONAL_OFFSETS,
        )
    }

    pub const fn manhattan_distance(&self, other: &Self) -> u64 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

// TODO: turn into try from ?
impl From<&str> for Point2D {
    fn from(value: &str) -> Point2D {
        let (x, y) = value.split_once(',').unwrap();
        Point2D {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        }
    }
}

impl Add<Point2D> for Point2D {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Point2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign<Point2D> for Point2D {
    fn add_assign(&mut self, rhs: Point2D) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub<Point2D> for Point2D {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Point2D {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<i64> for Point2D {
    type Output = Self;

    fn mul(self, rhs: i64) -> Self::Output {
        Point2D {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    pub const fn as_offset(&self) -> Point2D {
        match self {
            Self::North => Point2D { x: 0, y: -1 },
            Self::South => Point2D { x: 0, y: 1 },
            Self::West => Point2D { x: -1, y: 0 },
            Self::East => Point2D { x: 1, y: 0 },
        }
    }

    pub const fn left(&self) -> Self {
        match self {
            Self::North => Self::West,
            Self::South => Self::East,
            Self::West => Self::South,
            Self::East => Self::North,
        }
    }

    pub const fn right(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::South => Self::West,
            Self::West => Self::North,
            Self::East => Self::South,
        }
    }

    pub const fn flip(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::West => Self::East,
            Self::East => Self::West,
        }
    }

    pub const fn transform(&self, transform: &SquareSym) -> Self {
        match (self, transform) {
            (_, Nop) => *self,
            (_, Rot1) => self.left(),
            (_, Rot2) => self.flip(),
            (_, Rot3) => self.right(),
            (Self::North | Self::South, FlipX) => self.flip(),
            (Self::West | Self::East, FlipX) => *self,
            (Self::North | Self::South, FlipY) => *self,
            (Self::East | Self::West, FlipY) => self.flip(),
            (Self::North | Self::South, FlipAC) => self.left(),
            (Self::East | Self::West, FlipAC) => self.right(),
            (Self::North | Self::South, FlipBD) => self.right(),
            (Self::East | Self::West, FlipBD) => self.left(),
        }
    }
}

// TODO: bitvec for Map<bool>
#[derive(Debug)]
pub struct Grid<T> {
    values: Box<[T]>,
    width: usize,
    height: usize,
}

#[derive(Debug)]
pub enum ParseMapError {
    NotSquare,
    ParseCharError(char),
}

impl<T> Grid<T> {
    pub fn new(map: Box<[T]>, width: usize, height: usize) -> Self {
        Self {
            values: map,
            width,
            height,
        }
    }

    fn zero_sized() -> Grid<T> {
        Grid {
            values: Box::new([]),
            width: 0,
            height: 0,
        }
    }

    pub fn try_from_str(
        input: &str,
        mapping: fn(char) -> Option<T>,
    ) -> Result<Grid<T>, ParseMapError> {
        if input.is_empty() {
            return Ok(Grid::zero_sized());
        }

        let width = input.lines().next().unwrap().len();

        if !input.lines().all(|line| line.len() == width) {
            return Err(ParseMapError::NotSquare);
        }

        let map = input
            .lines()
            .flat_map(|line| {
                line.chars()
                    .map(|c| mapping(c).ok_or(ParseMapError::ParseCharError(c)))
            })
            .collect::<Result<Box<[T]>, ParseMapError>>()?;

        let height = input.lines().count();

        Ok(Grid {
            values: map,
            width,
            height,
        })
    }

    pub const fn height(&self) -> usize {
        self.height
    }

    pub const fn width(&self) -> usize {
        self.width
    }

    pub const fn area(&self) -> usize {
        self.width * self.height
    }

    pub fn row(&self, index: usize) -> &[T] {
        &self.values[(index * self.width)..((index + 1) * (self.width))]
    }

    pub fn rows(&self) -> impl Iterator<Item = &[T]> {
        (0..self.height).map(|row| self.row(row))
    }

    pub fn col(&self, index: usize) -> impl DoubleEndedIterator<Item = &T> {
        (0..self.height).map(move |row| &self.values[row * self.width + index])
    }

    pub fn contains(&self, point: &Point2D) -> bool {
        (0..self.width()).contains(&(point.x as usize))
            && (0..self.height()).contains(&(point.y as usize))
    }
}

impl<T: Clone> Grid<T> {
    pub fn filled_with(value: T, width: usize, height: usize) -> Grid<T> {
        Grid {
            values: Box::from(vec![value; width * height]),
            width,
            height,
        }
    }

    pub fn transform(&self, operation: &SquareSym) -> Grid<T> {
        let (width, height) = match operation {
            Nop | Rot2 | FlipX | FlipY => (self.width, self.height),
            Rot1 | Rot3 | FlipAC | FlipBD => (self.height, self.width),
        };

        Grid {
            width,
            height,
            values: (0..height)
                .flat_map(|x| {
                    (0..width).map(move |y| {
                        let (x, y) = match operation {
                            Nop => (x, y),
                            Rot1 => (y, self.height() - 1 - x),
                            Rot2 => (self.width() - 1 - x, self.height() - 1 - y),
                            Rot3 => (self.width() - y, x),
                            FlipX => (x, self.height() - 1 - y),
                            FlipY => (self.width() - 1 - x, y),
                            FlipAC => (y, x),
                            FlipBD => (self.width() - 1 - y, self.height() - 1 - x),
                        };
                        self[Point2D {
                            x: x as i64,
                            y: y as i64,
                        }]
                        .clone()
                    })
                })
                .collect(),
        }
    }
}

impl<T> Index<Point2D> for Grid<T> {
    type Output = T;

    fn index(&self, index: Point2D) -> &Self::Output {
        let x: usize = index
            .x
            .try_into()
            .unwrap_or_else(|_| panic!("index {index:?} out of bounds"));
        let y: usize = index
            .y
            .try_into()
            .unwrap_or_else(|_| panic!("index {index:?} out of bounds"));
        &self.values[y * self.width + x]
    }
}

impl<T> IndexMut<Point2D> for Grid<T> {
    fn index_mut(&mut self, index: Point2D) -> &mut T {
        let x: usize = index
            .x
            .try_into()
            .unwrap_or_else(|_| panic!("index {index:?} out of bounds"));
        let y: usize = index
            .y
            .try_into()
            .unwrap_or_else(|_| panic!("index {index:?} out of bounds"));
        &mut self.values[y * self.width + x]
    }
}

impl<T: Display> Display for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.rows() {
            for tile in row {
                write!(f, "{}", tile)?
            }
            writeln!(f)?
        }
        Ok(())
    }
}

/// Symmetries on the the Square:
/// A - - B
/// |     |
/// |     |
/// C - - D
#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum SquareSym {
    /// No operation
    Nop = 0,
    /// rotate counter-clockwise 90°  
    Rot1 = 1,
    /// rotate 180°  
    Rot2 = 2,
    /// rotate clockwise 90°  
    Rot3 = 3,
    /// flip thorugh x-axis; flip vertically
    FlipX = 4,
    /// flip thorugh y-axis; flip horizontally
    FlipY = 5,
    /// flip though line going from top left corner to bottom right corner, same as matrix transpose
    FlipAC = 6,
    /// flip though line going from top right corner to bottom left corner
    FlipBD = 7,
}

use SquareSym::{FlipAC, FlipBD, FlipX, FlipY, Nop, Rot1, Rot2, Rot3};

impl SquareSym {
    const CAYLAY_TABLE: [[SquareSym; 8]; 8] = [
        [Nop, Rot1, Rot2, Rot3, FlipX, FlipY, FlipAC, FlipBD],
        [Rot1, Rot2, Rot3, Nop, FlipBD, FlipAC, FlipX, FlipY],
        [Rot2, Rot3, Nop, Rot1, FlipY, FlipX, FlipBD, FlipAC],
        [Rot3, Nop, Rot1, Rot2, FlipAC, FlipBD, FlipY, FlipX],
        [FlipX, FlipAC, FlipY, FlipBD, Nop, Rot2, Rot1, Rot3],
        [FlipY, FlipBD, FlipX, FlipAC, Rot2, Nop, Rot3, Rot1],
        [FlipAC, FlipX, FlipBD, FlipY, Rot3, Rot1, Nop, Rot2],
        [FlipBD, FlipY, FlipAC, FlipX, Rot1, Rot3, Rot2, Nop],
    ];

    pub const fn transform(&self, other: &SquareSym) -> SquareSym {
        SquareSym::CAYLAY_TABLE[*self as usize][*other as usize]
    }

    pub const fn inverse(&self) -> SquareSym {
        match self {
            Rot1 => Rot3,
            Rot3 => Rot1,
            _ => *self,
        }
    }

    /// direction of north side after transform
    pub const fn transform_direction(&self) -> Direction {
        match self {
            Nop => Direction::North,
            Rot1 => Direction::West,
            Rot2 => Direction::South,
            Rot3 => Direction::East,
            FlipX => Direction::South,
            FlipY => Direction::North,
            FlipAC => Direction::West,
            FlipBD => Direction::East,
        }
    }

    pub const fn from_num(num: u8) -> SquareSym {
        if num < 8 {
            unsafe { mem::transmute::<u8, SquareSym>(num) }
        } else {
            panic!()
        }
    }
}
