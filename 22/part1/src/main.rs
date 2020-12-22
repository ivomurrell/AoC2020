use std::{collections::VecDeque, fs::read_to_string};

fn main() {
    let input = read_to_string("../input").expect("Failed to open input file");
    let mut player_decks = input.split_terminator("\n\n").map(|deck| {
        deck.lines()
            .skip(1)
            .map(|card| card.parse().expect("Could not parse card"))
            .collect()
    });
    let (mut player_1, mut player_2): (VecDeque<usize>, _) = (
        player_decks.next().expect("Input was empty"),
        player_decks
            .next()
            .expect("Second player's deck not listed"),
    );

    while !player_1.is_empty() && !player_2.is_empty() {
        let card_1 = player_1.pop_front().unwrap();
        let card_2 = player_2.pop_front().unwrap();
        if card_1 > card_2 {
            player_1.push_back(card_1);
            player_1.push_back(card_2);
        } else {
            player_2.push_back(card_2);
            player_2.push_back(card_1);
        }
    }

    let winner = if player_1.is_empty() {
        player_2
    } else {
        player_1
    };
    let final_score: usize = winner
        .into_iter()
        .rev()
        .enumerate()
        .map(|(pos, card)| card * (pos + 1))
        .sum();
    println!("The winning player's score was {}!", final_score);
}
