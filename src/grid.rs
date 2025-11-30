//! Structs and measures for dealing with square grids,
//! a common format for advent of code tasks
#![allow(dead_code)]

use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

impl Coord {
    /// return a list of adjecent coordinates within the bounds (0..width) and (0..height)
    pub fn adjacent(&self, width_bound: usize, height_bound: usize) -> Vec<Coord> {
        let mut a = Vec::with_capacity(4);
        if self.x > 0 {
            a.push(Coord {
                x: self.x - 1,
                y: self.y,
            });
        }
        if self.y > 0 {
            a.push(Coord {
                x: self.x,
                y: self.y - 1,
            });
        }
        if self.x < width_bound - 1 {
            a.push(Coord {
                x: self.x + 1,
                y: self.y,
            });
        }
        if self.y < height_bound - 1 {
            a.push(Coord {
                x: self.x,
                y: self.y + 1,
            });
        }
        a
    }

    pub fn adjecent_with_diagonals(&self, width_bound: usize, height_bound: usize) -> Vec<Coord> {
        let mut a = Vec::with_capacity(8);
        if self.x > 0 {
            a.push(Coord {
                x: self.x - 1,
                y: self.y,
            });
        }
        if self.y > 0 {
            a.push(Coord {
                x: self.x,
                y: self.y - 1,
            });
        }
        if self.x < width_bound - 1 {
            a.push(Coord {
                x: self.x + 1,
                y: self.y,
            });
        }
        if self.y < height_bound - 1 {
            a.push(Coord {
                x: self.x,
                y: self.y + 1,
            });
        }
        if self.y > 0 && self.x > 0 {
            a.push(Coord {
                x: self.x - 1,
                y: self.y - 1,
            });
        }
        if self.x < width_bound - 1 && self.y > 0 {
            a.push(Coord {
                x: self.x + 1,
                y: self.y - 1,
            });
        }
        if self.x > 0 && self.y < height_bound - 1 {
            a.push(Coord {
                x: self.x - 1,
                y: self.y + 1,
            });
        }
        if self.x < width_bound - 1 && self.y < height_bound - 1 {
            a.push(Coord {
                x: self.x + 1,
                y: self.y + 1,
            });
        }
        a
    }

    pub fn manhattan_distance(&self, other: &Self) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

// TODO: turn into try from ?
impl From<&str> for Coord {
    fn from(value: &str) -> Coord {
        let (x, y) = value.split_once(',').unwrap();
        Coord {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
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

#[derive(Debug)]
pub struct Map<T> {
    map: Box<[T]>,
    width: usize,
    height: usize,
}

#[derive(Debug)]
pub enum ParseMapError {
    NotSquare,
    ParseCharError,
}

impl<T> Map<T> {
    pub fn new(map: Box<[T]>, width: usize, height: usize) -> Self {
        Self { map, width, height }
    }

    fn zero_sized() -> Map<T> {
        Map {
            map: Box::new([]),
            width: 0,
            height: 0,
        }
    }

    pub fn try_from_str(input: &str, mapping: fn(char) -> T) -> Result<Map<T>, ParseMapError> {
        if input.is_empty() {
            return Ok(Map::zero_sized());
        }
        let width = input.lines().next().unwrap().len();
        let height = input.len() / width;
        let map = input
            .lines()
            .flat_map(|line| {
                // if line.len() != width {
                //     return Err(ParseMapError::NotSquare);
                // }
                line.chars().map(mapping)
            })
            .collect::<Box<[T]>>();
        Ok(Map { map, width, height })
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn row(&self, index: usize) -> &[T] {
        &self.map[(index * self.width)..((index + 1) * (self.width))]
    }

    // pub fn col(&self, index: usize) -> &[T] {
    //     &self.map[(index * self.width)..((index + 1) * (self.width))]
    // }

    pub fn rows(&self) -> impl Iterator<Item = &[T]> {
        (0..self.height).map(|row| self.row(row))
    }

    // fn rows_mut(&self) -> impl Iterator<Item = impl Iterator<Item = &mut T>> {
    //     (0..self.height).map(move |y| (0..self.width).map(move |x| &mut self[Coord { x, y }]))
    // }

    // pub fn cols(&self) -> impl Iterator<Item = &[T]> {
    //     (0..self.height).map(|col| self.row(row))
    // }
}

impl<T: Clone> Map<T> {
    pub fn filled_with(value: T, width: usize, height: usize) -> Map<T> {
        Map {
            map: Box::from(vec![value; width * height]),
            width,
            height,
        }
    }
}

impl<T> Index<Coord> for Map<T> {
    type Output = T;

    fn index(&self, index: Coord) -> &Self::Output {
        &self.map[index.y * self.width + index.x]
    }
}

impl<T> IndexMut<Coord> for Map<T> {
    fn index_mut(&mut self, index: Coord) -> &mut T {
        &mut self.map[index.y * self.width + index.x]
    }
}

// struct MapNotSquareError(())
//
// impl<T: Clone> TryFrom<&[Vec<T>]> for Map<T> {
//     type Error = MapNotSquareError;
//
//     fn try_from(value: &[Vec<T>]) -> Result<Self, Self::Error> {
//         if value.len() == 0 {
//             return Ok(Map {
//                 map: Rc::new([]),
//                 width : 0,
//                 height: 0
//             });
//         }
//         let width = value[0].len();
//         if !value.iter().all(|row| row.len() == width) {
//             return Err(MapNotSquareError);
//         }
//
//         let height = value.len();
//         let map = Rc::new(value.concat());
//         Ok(Map {
//             map,
//             width,
//             height
//         })
//     }
// }

// fn from_iter<I: IntoIterator<Item = &[T]>(iter: I) -> Self {
//     let mut width = Some(())
//     let map: [T] = iter.into_iter().fold(Vec::new(), |acc, row| acc = [acc, row].concat());
//     Map {
//         map: Rc::new(map),
//         width:
//     }
// }
