use lazysort::SortedBy;
use std::{
    cell::RefCell,
    ptr,
    rc::{Rc, Weak},
};

#[derive(Debug)]
struct Point3D {
    x: i64,
    y: i64,
    z: i64,
}

impl Point3D {
    fn euclidean_distance(&self, other: &Self) -> f64 {
        let diff_x = self.x - other.x;
        let diff_y = self.y - other.y;
        let diff_z = self.z - other.z;
        ((diff_x * diff_x + diff_y * diff_y + diff_z * diff_z) as f64).sqrt()
    }
}

#[derive(Debug)]
struct Node {
    pos: Point3D,
    /// (transitively) points to the representative of current component
    component: WeakNodeRef,
    /// number of elements with current node as ancestor
    children_count: u32,
    /// upper bound for depth of tree
    rank: u8,
}

type NodeRef = Rc<RefCell<Node>>;
type WeakNodeRef = Weak<RefCell<Node>>;

impl Node {
    fn is_root(&self) -> bool {
        ptr::eq(self, self.component.upgrade().unwrap().as_ptr())
    }
}

fn connect(lhs: &NodeRef, rhs: &NodeRef) {
    if is_connected(lhs, rhs) {
        return;
    }
    let lhs = find_component(lhs);
    let rhs = find_component(rhs);
    if lhs.borrow().rank >= rhs.borrow().rank {
        rhs.borrow_mut().component = lhs.borrow().component.clone();
        lhs.borrow_mut().children_count += rhs.borrow().children_count;
        if lhs.borrow().rank == rhs.borrow().rank {
            lhs.borrow_mut().rank += 1;
        }
    } else {
        lhs.borrow_mut().component = rhs.borrow().component.clone();
        rhs.borrow_mut().children_count += lhs.borrow().children_count;
    }
}

fn is_connected(lhs: &NodeRef, rhs: &NodeRef) -> bool {
    ptr::eq(find_component(lhs).as_ptr(), find_component(rhs).as_ptr())
}

fn find_component(node: &NodeRef) -> NodeRef {
    if !node.borrow().is_root() {
        let component = find_component(&node.borrow_mut().component.upgrade().unwrap()).clone();
        node.borrow_mut().component = Rc::downgrade(&component);
    }
    node.borrow_mut().component.upgrade().unwrap().clone()
}

#[derive(Debug, Clone)]
struct Edge<T> {
    from: NodeRef,
    to: NodeRef,
    weight: T,
}

fn parse_points(input: &str) -> Box<[NodeRef]> {
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
                    children_count: 1,
                    rank: 0,
                })
            })
        })
        .collect()
}

fn kruskal(nodes: &[NodeRef], max_connections: usize) -> Option<Edge<f64>> {
    let mut edges = Vec::new();
    for (i, from) in nodes.iter().enumerate() {
        for to in &nodes[(i + 1)..] {
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
        connect(&edge.from, &edge.to);
        if find_component(&edge.from).borrow().children_count as usize == nodes.len() {
            return Some(edge.clone());
        }
        connections += 1; // we also count nodes already connected
        if connections == max_connections {
            return Some(edge.clone());
        }
    }
    None
}

pub fn task1_connection_count(input: &str, max_connections: usize) -> String {
    let nodes = parse_points(input);
    kruskal(&nodes, max_connections);
    nodes
        .iter()
        .filter(|node| node.borrow().is_root())
        .sorted_by(|a, b| {
            a.borrow()
                .children_count
                .cmp(&b.borrow().children_count)
                .reverse()
        })
        .take(3)
        .map(|node| u64::from(node.borrow().children_count))
        .product::<u64>()
        .to_string()
}

pub fn task1(input: &str) -> String {
    task1_connection_count(input, 1000)
}

pub fn task2(input: &str) -> String {
    let nodes = parse_points(input);
    let last_edge = kruskal(&nodes, usize::MAX).unwrap();
    (last_edge.from.borrow().pos.x * last_edge.to.borrow().pos.x).to_string()
}
