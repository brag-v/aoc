use std::collections::{HashSet, VecDeque};

fn parse_game_setup(input: &str) -> (VecDeque<u8>, VecDeque<u8>) {
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

fn play_combat_round(deck_1: &mut VecDeque<u8>, deck_2: &mut VecDeque<u8>) {
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

fn play_combat<'a>(deck_1: &'a mut VecDeque<u8>, deck_2: &'a mut VecDeque<u8>) -> &'a VecDeque<u8> {
    while !deck_1.is_empty() && !deck_2.is_empty() {
        play_combat_round(deck_1, deck_2);
    }
    if deck_1.is_empty() { deck_2 } else { deck_1 }
}

fn calculate_score(deck: &VecDeque<u8>) -> usize {
    deck.iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (position, value)| {
            acc + (position + 1) * *value as usize
        })
}

pub fn task1(input: &str) -> String {
    let (mut deck_1, mut deck_2) = parse_game_setup(input);
    let winner = play_combat(&mut deck_1, &mut deck_2);
    calculate_score(winner).to_string()
}

fn decks_to_key(deck_1: &VecDeque<u8>, deck_2: &VecDeque<u8>) -> Box<[u8]> {
    let mut key = Vec::with_capacity(deck_1.len() + 1 + deck_2.len());
    key.extend(deck_1.iter());
    key.push(0);
    key.extend(deck_2.iter());
    key.into_boxed_slice()
}

fn play_recursive_combat_round(deck_1: &mut VecDeque<u8>, deck_2: &mut VecDeque<u8>) {
    let card_1 = deck_1.pop_front().unwrap();
    let card_2 = deck_2.pop_front().unwrap();

    let player_1_wins = if card_1 as usize <= deck_1.len() && card_2 as usize <= deck_2.len() {
        let mut subdeck_1 = deck_1.iter().take(card_1 as usize).cloned().collect();
        let mut subdeck_2 = deck_2.iter().take(card_2 as usize).cloned().collect();
        play_recursive_combat(&mut subdeck_1, &mut subdeck_2)
    } else {
        card_1 > card_2
    };
    if player_1_wins {
        deck_1.push_back(card_1);
        deck_1.push_back(card_2);
    } else {
        deck_2.push_back(card_2);
        deck_2.push_back(card_1);
    }
}

fn play_recursive_combat(deck_1: &mut VecDeque<u8>, deck_2: &mut VecDeque<u8>) -> bool {
    let mut prev_hands = HashSet::new();
    while !deck_1.is_empty() && !deck_2.is_empty() {
        if !prev_hands.insert(decks_to_key(deck_1, deck_2)) {
            return true;
        }
        play_recursive_combat_round(deck_1, deck_2);
    }
    !deck_1.is_empty()
}

pub fn task2(input: &str) -> String {
    let (mut deck_1, mut deck_2) = parse_game_setup(input);
    let player_1_wins = play_recursive_combat(&mut deck_1, &mut deck_2);
    let winning_score = if player_1_wins {
        calculate_score(&deck_1)
    } else {
        calculate_score(&deck_2)
    };
    winning_score.to_string()
}
