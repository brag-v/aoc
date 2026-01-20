use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::{Rc, Weak},
};

#[derive(Debug)]
struct Bag {
    id: usize,
    containers: Vec<Weak<RefCell<Bag>>>,
    contains: Vec<(u32, Weak<RefCell<Bag>>)>,
}

fn parse_bags(input: &str) -> HashMap<String, Rc<RefCell<Bag>>> {
    let mut bags = HashMap::new();
    let input = input
        .replace(" bags", "")
        .replace(" bag", "")
        .replace(".", "");
    // multiple allocations + order matters
    // also must use String instead of &str in hashmap
    input
        .lines()
        .map(|line| line.split_once(" contain").unwrap().0)
        .enumerate()
        .for_each(|(id, bag)| {
            bags.insert(
                bag.to_string(),
                Rc::new(RefCell::new(Bag {
                    containers: Vec::new(),
                    contains: Vec::new(),
                    id,
                })),
            );
        });
    for (container, containees) in input
        .lines()
        .map(|line| line.split_once(" contain ").unwrap())
    {
        if containees == "no other" {
            continue;
        }
        let container = bags.get(container).unwrap();
        containees.split(", ").for_each(|content| {
            let (count, containee) = content.split_once(' ').unwrap();
            let count = count.parse().unwrap();
            let containee = bags.get(containee).unwrap();
            containee
                .borrow_mut()
                .containers
                .push(Rc::downgrade(container));
            container
                .borrow_mut()
                .contains
                .push((count, Rc::downgrade(containee)));
        })
    }
    bags
}

fn container_count(bag: &Rc<RefCell<Bag>>) -> usize {
    let mut containers = HashSet::new();
    let mut exploring = vec![bag.clone()];
    while let Some(bag) = exploring.pop() {
        if !containers.insert(bag.borrow().id) {
            continue;
        }
        for containers in &bag.borrow().containers {
            exploring.push(Weak::upgrade(containers).unwrap());
        }
    }
    containers.len() - 1 // don't count bag itself
}

pub fn task1(input: &str) -> String {
    let bags = parse_bags(input);
    container_count(bags.get("shiny gold").unwrap()).to_string()
}

fn content_count(bag: &Rc<RefCell<Bag>>) -> usize {
    bag.borrow()
        .contains
        .iter()
        .map(|(count, inner)| *count as usize * (1 + content_count(&Weak::upgrade(inner).unwrap())))
        .sum()
}

pub fn task2(input: &str) -> String {
    let bags = parse_bags(input);
    content_count(bags.get("shiny gold").unwrap()).to_string()
}
