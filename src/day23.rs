use std::{
    cell::RefCell,
    iter::once,
    rc::{Rc, Weak},
};

use itertools::Itertools;

#[derive(Debug)]
struct LinkedNode {
    value: usize,
    next: Weak<RefCell<LinkedNode>>,
}

type NodeRef = Rc<RefCell<LinkedNode>>;

fn nodes_by_value_unlinked(n: usize) -> Vec<NodeRef> {
    let zero_node = Rc::new_cyclic(|me| {
        RefCell::new(LinkedNode {
            value: 0,
            next: me.clone(),
        })
    }); // zero node for zero-indexing ease
    let mut cups_by_value: Vec<NodeRef> = vec![zero_node];
    for i in 1..=n {
        cups_by_value.push(Rc::new_cyclic(|me| {
            RefCell::new(LinkedNode {
                value: i,
                next: me.clone(),
            })
        }));
    }
    cups_by_value
}

fn compute_target_label(
    mut current_value: usize,
    collected_values: &[usize],
    max_index: usize,
) -> usize {
    loop {
        current_value -= 1;
        if current_value == 0 {
            current_value = max_index;
        }
        if !collected_values.contains(&current_value) {
            return current_value;
        }
    }
}

fn perfom_round(cups: &[NodeRef], current_cup: NodeRef) {
    let mut collected_cups = Vec::with_capacity(3);
    let mut next_cup = current_cup.clone();
    for _ in 0..3 {
        let next_next_cup = next_cup.borrow().next.clone().upgrade().unwrap();
        next_cup = next_next_cup;
        collected_cups.push(next_cup.clone());
    }
    let target_label = compute_target_label(
        current_cup.borrow().value,
        &collected_cups
            .iter()
            .map(|cup| cup.borrow().value)
            .collect::<Box<[usize]>>(),
        cups.len() - 1,
    );
    let target_cup = cups[target_label].clone();

    // move collected cups to after target cup
    current_cup.borrow_mut().next = collected_cups[2].borrow().next.clone();
    collected_cups[2].borrow_mut().next = target_cup.borrow().next.clone();
    target_cup.borrow_mut().next = Rc::downgrade(&collected_cups[0]);
}

pub fn task1(input: &str) -> String {
    let cups_by_value = nodes_by_value_unlinked(input.len());

    let digits: Box<[usize]> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    for (a, b) in digits
        .iter()
        .cloned()
        .chain(once(digits[0]))
        .tuple_windows()
    {
        cups_by_value[a].borrow_mut().next = Rc::downgrade(&cups_by_value[b]);
    }

    let start_value = digits[0];
    let mut node = cups_by_value[start_value].clone();

    for _ in 0..100 {
        perfom_round(&cups_by_value, node.clone());
        let next_node = node.borrow().next.clone().upgrade().unwrap();
        node = next_node;
    }
    node = cups_by_value[1].borrow().next.upgrade().unwrap();
    let mut result = String::new();
    while node.borrow().value != 1 {
        result.push((node.borrow().value as u8 + b'0') as char);
        let next_node = node.borrow().next.clone().upgrade().unwrap();
        node = next_node;
    }

    result
}

pub fn task2(input: &str) -> String {
    let cups_by_value = nodes_by_value_unlinked(1_000_000);

    let digits: Box<[usize]> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    for (from, to) in digits
        .iter()
        .cloned()
        .chain((input.len() + 1)..=1_000_000)
        .chain(once(digits[0]))
        .tuple_windows()
    {
        cups_by_value[from].borrow_mut().next = Rc::downgrade(&cups_by_value[to]);
    }

    let start_value = digits[0];
    let mut node = cups_by_value[start_value].clone();

    for _ in 0..10_000_000 {
        perfom_round(&cups_by_value, node.clone());
        let next_node = node.borrow().next.clone().upgrade().unwrap();
        node = next_node;
    }

    node = cups_by_value[1].clone();
    let mut result = 1;
    for _ in 0..2 {
        let next_node = node.borrow().next.clone().upgrade().unwrap();
        node = next_node;
        result *= node.borrow().value;
    }
    result.to_string()
}
