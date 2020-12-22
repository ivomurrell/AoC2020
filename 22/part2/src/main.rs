use std::{collections::VecDeque, fs::read_to_string};

enum WinningPlayer {
    Player1,
    Player2,
}

fn recursive_game(
    depth: u32,
    mut player_1: VecDeque<usize>,
    mut player_2: VecDeque<usize>,
) -> (WinningPlayer, VecDeque<usize>) {
    let mut hand_1_history = Vec::new();
    let mut hand_2_history = Vec::new();
    while !player_1.is_empty() && !player_2.is_empty() {
        if hand_1_history.contains(&player_1) || hand_2_history.contains(&player_2) {
            return (WinningPlayer::Player1, player_1);
        } else {
            hand_1_history.push(player_1.clone());
            hand_2_history.push(player_2.clone());
        }

        let card_1 = player_1.pop_front().unwrap();
        let card_2 = player_2.pop_front().unwrap();

        let winning_player = if card_1 > player_1.len() || card_2 > player_2.len() {
            if card_1 > card_2 {
                WinningPlayer::Player1
            } else {
                WinningPlayer::Player2
            }
        } else {
            recursive_game(
                depth + 1,
                player_1.iter().take(card_1).copied().collect(),
                player_2.iter().take(card_2).copied().collect(),
            )
            .0
        };

        match winning_player {
            WinningPlayer::Player1 => {
                player_1.push_back(card_1);
                player_1.push_back(card_2);
            }
            WinningPlayer::Player2 => {
                player_2.push_back(card_2);
                player_2.push_back(card_1);
            }
        }
    }

    if player_1.is_empty() {
        (WinningPlayer::Player2, player_2)
    } else {
        (WinningPlayer::Player1, player_1)
    }
}

fn main() {
    let input = read_to_string("../input").expect("Failed to open input file");
    let mut player_decks = input.split_terminator("\n\n").map(|deck| {
        deck.lines()
            .skip(1)
            .map(|card| card.parse().expect("Could not parse card"))
            .collect()
    });
    let (player_1, player_2): (VecDeque<usize>, _) = (
        player_decks.next().expect("Input was empty"),
        player_decks
            .next()
            .expect("Second player's deck not listed"),
    );

    let final_score: usize = recursive_game(0, player_1, player_2)
        .1
        .into_iter()
        .rev()
        .enumerate()
        .map(|(pos, card)| card * (pos + 1))
        .sum();
    println!("The winning player's score was {}!", final_score);
}
