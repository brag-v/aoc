use crate::ac3::ac3;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Food {
    ingredients: Vec<String>,
    allergens: Vec<String>,
}

fn parse_food_list(input: &str) -> Vec<Food> {
    input
        .lines()
        .map(|line| {
            let (ingerdietns, allergens) = line.split_once(" (contains ").unwrap();
            let ingredients = ingerdietns.split(' ').map(|s| s.to_string()).collect();
            let allergens = allergens
                .strip_suffix(")")
                .unwrap()
                .split(", ")
                .map(str::to_string)
                .collect();
            Food {
                ingredients,
                allergens,
            }
        })
        .collect()
}

fn possible_allergen_containers(foods: &[Food]) -> HashMap<&str, HashSet<&str>> {
    let mut allergen_containers = HashMap::new();
    for food in foods {
        let ingredient_set: HashSet<&str> = food.ingredients.iter().map(|s| s.as_ref()).collect();
        for allergen in &food.allergens {
            allergen_containers
                .entry(allergen.as_ref())
                .and_modify(|ingredients: &mut HashSet<&str>| {
                    ingredients.retain(|ingredient| ingredient_set.contains(ingredient))
                })
                .or_insert(ingredient_set.clone());
        }
    }
    allergen_containers
}

pub fn task1(input: &str) -> String {
    let foods = parse_food_list(input);
    let mapping = possible_allergen_containers(&foods);
    let combined_containers =
        mapping
            .values()
            .fold(HashSet::<&str>::new(), |mut combined, containers| {
                combined.extend(containers);
                combined
            });
    foods
        .iter()
        .flat_map(|food| food.ingredients.iter().map(|s| s.as_ref()))
        .filter(|ingredient| !combined_containers.contains(ingredient))
        .count()
        .to_string()
}

pub fn task2(input: &str) -> String {
    let foods = parse_food_list(input);
    let mapping = possible_allergen_containers(&foods);
    let sorted_allergens = mapping.keys().sorted_unstable();
    let mut sorted_container_lists: Vec<Vec<&str>> = sorted_allergens
        .map(|allergen| mapping.get(allergen).unwrap().iter().copied().collect())
        .collect();
    ac3(&mut sorted_container_lists);
    sorted_container_lists
        .iter()
        .map(|ingredient_list| ingredient_list[0])
        .join(",")
}
