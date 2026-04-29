use std::collections::VecDeque;

fn parse_game_setup(input: &str) -> (VecDeque<u32>, VecDeque<u32>) {
    let decks = input.split_once("\n\n").unwrap();

    let deck_1 = decks
        .0
        .lines()
        .skip(1)
        .map(|line| line.parse().unwrap())
        .collect();
    let deck_2 = decks
        .1
        .lines()
        .skip(1)
        .map(|line| line.parse().unwrap())
        .collect();
    (deck_1, deck_2)
}

fn play_round(deck_1: &mut VecDeque<u32>, deck_2: &mut VecDeque<u32>) {
    let card_1 = deck_1.pop_front().unwrap();
    let card_2 = deck_2.pop_front().unwrap();
    if card_1 > card_2 {
        deck_1.push_back(card_1);
        deck_1.push_back(card_2);
    } else {
        deck_2.push_back(card_2);
        deck_2.push_back(card_1);
    }
}

fn play_game<'a>(
    deck_1: &'a mut VecDeque<u32>,
    deck_2: &'a mut VecDeque<u32>,
) -> &'a VecDeque<u32> {
    while !deck_1.is_empty() && !deck_2.is_empty() {
        play_round(deck_1, deck_2);
    }
    if deck_1.is_empty() { deck_2 } else { deck_1 }
}

fn calculate_score(deck: &VecDeque<u32>) -> u32 {
    deck.iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (position, value)| {
            acc + (position as u32 + 1) * value
        })
}

pub fn task1(input: &str) -> String {
    let (mut deck_1, mut deck_2) = parse_game_setup(input);
    let winner = play_game(&mut deck_1, &mut deck_2);
    calculate_score(winner).to_string()
}

pub fn task2(_input: &str) -> String {
    todo!("Day 1 task 2")
}
