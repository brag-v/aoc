//! Structs and measures for dealing with square grids,
//! a common format for advent of code tasks
#![allow(dead_code)]

use std::ops::{Add, Index, IndexMut};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub struct Point2D {
    pub x: isize,
    pub y: isize,
}

impl Point2D {
    pub fn adjecent_with_offsets(
        &self,
        width_bound: usize,
        height_bound: usize,
        offsets: &[Point2D],
    ) -> impl Iterator<Item = Point2D> {
        let width_bound = width_bound as isize;
        let height_bound = height_bound as isize;
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
        self.adjecent_with_offsets(
            width_bound,
            height_bound,
            &[
                Point2D { x: -1, y: 0 },
                Point2D { x: 0, y: -1 },
                Point2D { x: 0, y: 1 },
                Point2D { x: 1, y: 0 },
            ],
        )
    }

    pub fn adjecent_with_diagonals(
        &self,
        width_bound: usize,
        height_bound: usize,
    ) -> impl Iterator<Item = Point2D> {
        self.adjecent_with_offsets(
            width_bound,
            height_bound,
            &[
                Point2D { x: -1, y: -1 },
                Point2D { x: -1, y: 0 },
                Point2D { x: -1, y: 1 },
                Point2D { x: 0, y: -1 },
                Point2D { x: 0, y: 1 },
                Point2D { x: 1, y: -1 },
                Point2D { x: 1, y: 0 },
                Point2D { x: 1, y: 1 },
            ],
        )
    }

    pub fn manhattan_distance(&self, other: &Self) -> usize {
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    North,
    South,
    West,
    East,
}

// TODO: bitvec for Map<bool>
#[derive(Debug)]
pub struct Map<T> {
    values: Box<[T]>,
    width: usize,
    height: usize,
}

#[derive(Debug)]
pub enum ParseMapError {
    NotSquare,
    ParseCharError(char),
}

impl<T> Map<T> {
    pub fn new(map: Box<[T]>, width: usize, height: usize) -> Self {
        Self {
            values: map,
            width,
            height,
        }
    }

    fn zero_sized() -> Map<T> {
        Map {
            values: Box::new([]),
            width: 0,
            height: 0,
        }
    }

    pub fn try_from_str(
        input: &str,
        mapping: fn(char) -> Option<T>,
    ) -> Result<Map<T>, ParseMapError> {
        if input.is_empty() {
            return Ok(Map::zero_sized());
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

        Ok(Map {
            values: map,
            width,
            height,
        })
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn area(&self) -> usize {
        self.width * self.height
    }

    pub fn row(&self, index: usize) -> &[T] {
        &self.values[(index * self.width)..((index + 1) * (self.width))]
    }

    pub fn rows(&self) -> impl Iterator<Item = &[T]> {
        (0..self.height).map(|row| self.row(row))
    }
}

impl<T: Clone> Map<T> {
    pub fn filled_with(value: T, width: usize, height: usize) -> Map<T> {
        Map {
            values: Box::from(vec![value; width * height]),
            width,
            height,
        }
    }
}

impl<T> Index<Point2D> for Map<T> {
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

impl<T> IndexMut<Point2D> for Map<T> {
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
