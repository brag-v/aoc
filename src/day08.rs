use std::{
    cell::RefCell,
    ptr,
    rc::{Rc, Weak},
};

use lazysort::SortedBy;

#[derive(Debug)]
struct Point3D {
    x: i64,
    y: i64,
    z: i64,
}

impl Point3D {
    fn euclidean_distance(&self, other: &Self) -> f64 {
        (((self.x - other.x) * (self.x - other.x)
            + (self.y - other.y) * (self.y - other.y)
            + (self.z - other.z) * (self.z - other.z)) as f64)
            .sqrt()
    }
}

#[derive(Debug)]
struct Node {
    pos: Point3D,
    component: Weak<RefCell<Node>>,
    component_size: u32,
    rank: u8,
}

impl Node {
    fn is_root(&self) -> bool {
        ptr::eq(self, self.component.upgrade().unwrap().as_ptr())
    }
}

fn connect(lhs: Rc<RefCell<Node>>, rhs: Rc<RefCell<Node>>) {
    if is_connected(lhs.clone(), rhs.clone()) {
        return;
    }
    let lhs = find_component(lhs.clone());
    let rhs = find_component(rhs.clone());
    if lhs.borrow().rank >= rhs.borrow().rank {
        rhs.borrow_mut().component = lhs.borrow().component.clone();
        lhs.borrow_mut().component_size += rhs.borrow().component_size;
        if lhs.borrow().rank == rhs.borrow().rank {
            lhs.borrow_mut().rank += 1;
        }
    } else {
        lhs.borrow_mut().component = rhs.borrow().component.clone();
        rhs.borrow_mut().component_size += lhs.borrow().component_size;
    }
}

fn is_connected(lhs: Rc<RefCell<Node>>, rhs: Rc<RefCell<Node>>) -> bool {
    ptr::eq(find_component(lhs).as_ptr(), find_component(rhs).as_ptr())
}

fn find_component(node: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
    if !node.borrow().is_root() {
        let component = find_component(node.borrow_mut().component.upgrade().unwrap()).clone();
        node.borrow_mut().component = Rc::downgrade(&component);
    }
    node.borrow_mut().component.upgrade().unwrap().clone()
}

#[derive(Debug, Clone)]
struct Edge<T> {
    from: Rc<RefCell<Node>>,
    to: Rc<RefCell<Node>>,
    weight: T,
}

fn parse_points(input: &str) -> Vec<Rc<RefCell<Node>>> {
    input
        .lines()
        .map(|line| {
            let elems = line
                .split(',')
                .map(|elem| elem.parse().unwrap())
                .collect::<Box<[i64]>>();
            Rc::new_cyclic(|me| {
                RefCell::new(Node {
                    pos: Point3D {
                        x: elems[0],
                        y: elems[1],
                        z: elems[2],
                    },
                    component: me.clone(),
                    component_size: 1,
                    rank: 0,
                })
            })
        })
        .collect()
}

fn min_spanning_tree(nodes: &[Rc<RefCell<Node>>], max_connections: usize) -> Option<Edge<f64>> {
    let mut edges = Vec::new();
    for (i, from) in nodes.iter().enumerate() {
        for to in nodes[(i + 1)..].iter() {
            edges.push(Edge {
                from: from.clone(),
                to: to.clone(),
                weight: from.borrow().pos.euclidean_distance(&to.borrow().pos),
            });
        }
    }
    let mut connections = 0;
    for edge in edges
        .iter()
        .sorted_by(|a, b| a.weight.partial_cmp(&b.weight).unwrap())
    {
        connect(edge.from.clone(), edge.to.clone());
        if find_component(edge.from.clone()).borrow().component_size as usize == nodes.len() {
            return Some(edge.clone());
        }
        connections += 1; // we also count nodes already connected
        if connections == max_connections {
            return None;
        }
    }
    None
}

pub fn task1_connection_count(input: &str, max_connections: usize) -> String {
    let nodes = parse_points(input);
    min_spanning_tree(&nodes, max_connections);
    let components: Vec<Rc<RefCell<Node>>> = nodes
        .iter()
        .filter(|node| node.borrow().is_root())
        .map(Rc::clone)
        .collect();
    components
        .iter()
        .sorted_by(|a, b| {
            a.borrow()
                .component_size
                .cmp(&b.borrow().component_size)
                .reverse()
        })
        .take(3)
        .map(|node| node.borrow().component_size as u64)
        .product::<u64>()
        .to_string()
}

pub fn task1(input: &str) -> String {
    task1_connection_count(input, 1000)
}

pub fn task2(input: &str) -> String {
    let nodes = parse_points(input);
    let last_edge = min_spanning_tree(&nodes, usize::MAX).unwrap();
    (last_edge.from.borrow().pos.x * last_edge.to.borrow().pos.x).to_string()
}
