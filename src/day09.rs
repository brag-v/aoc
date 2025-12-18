use lazysort::SortedBy;

// note: the biggest areas barley fits in the u32 range
fn parse_nums(input: &str) -> Box<[(u32, u32)]> {
    input
        .lines()
        .map(|line| line.split_once(',').unwrap())
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .collect()
}

pub fn task1(input: &str) -> String {
    let vertecies = parse_nums(input);
    vertecies.iter()
        .enumerate()
        .flat_map(|(i, lhs)| vertecies[(i + 1)..].iter().map(move |rhs| (lhs, rhs)))
        .map(|(lhs, rhs)| (lhs.0.abs_diff(rhs.0) + 1) * (lhs.1.abs_diff(rhs.1) + 1))
        .max()
        .unwrap()
        .to_string()
}

#[derive(Debug)]
struct Rectangle {
    top: u32,
    bottom: u32,
    left: u32,
    right: u32,
}

impl Rectangle {
    const fn area(&self) -> u32 {
        (self.bottom - self.top + 1) * (self.right - self.left + 1)
    }
}

#[derive(Debug)]
struct Line {
    start: u32,
    end: u32,
    offset: u32,
    vertical: bool,
}

impl Line {
    const fn intersects_rectangle(&self, rect: &Rectangle) -> bool {
        if self.vertical {
            rect.top < self.offset
                && self.offset < rect.bottom
                && self.start < rect.right
                && rect.left < self.end
        } else {
            rect.left < self.offset
                && self.offset < rect.right
                && self.start < rect.bottom
                && rect.top < self.end
        }
    }
}

fn points_to_line(a: &(u32, u32), b: &(u32, u32)) -> Line {
    if a.0 == b.0 {
        Line {
            start: u32::min(a.1, b.1),
            end: u32::max(a.1, b.1),
            offset: a.0,
            vertical: false,
        }
    } else {
        assert!(a.1 == b.1);
        Line {
            start: u32::min(a.0, b.0),
            end: u32::max(a.0, b.0),
            offset: a.1,
            vertical: true,
        }
    }
}

pub fn task2(input: &str) -> String {
    let vertecies = parse_nums(input);
    let mut lines: Vec<Line> = vertecies
        .windows(2)
        .map(|line| points_to_line(&line[0], &line[1]))
        .collect();
    lines.push(points_to_line(vertecies.first().unwrap(), vertecies.last().unwrap()));

    vertecies.iter()
        .enumerate()
        .flat_map(|(i, lhs)| {
            vertecies[(i + 1)..].iter().map(|rhs| Rectangle {
                top: u32::min(lhs.1, rhs.1),
                bottom: u32::max(lhs.1, rhs.1),
                left: u32::min(lhs.0, rhs.0),
                right: u32::max(lhs.0, rhs.0),
            })
        })
        .sorted_by(|a, b| a.area().cmp(&b.area()).reverse())
        // this filter doesn't filter out rectangles formed by "outside corneres", 
        // e.g. it's completley outside the bounded area,
        .find(|rect| !lines.iter().any(|line| line.intersects_rectangle(rect)))
        .unwrap()
        .area()
        .to_string()
}
