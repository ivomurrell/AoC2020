use itertools::Itertools;
use std::{collections::HashMap, fs::read_to_string};

fn generate_possible_matches(rules: &HashMap<u32, &str>, number: u32) -> Vec<String> {
    let rule = rules[&number];
    if rule.starts_with('"') {
        vec![rule[1..2].to_owned()]
    } else {
        rule.split(" | ")
            .map(|choice| {
                choice
                    .split_ascii_whitespace()
                    .map(|sub_rule| {
                        generate_possible_matches(
                            rules,
                            sub_rule.parse().expect("Could not parse sub-rule number"),
                        )
                    })
                    .multi_cartesian_product()
                    .map(|product| product.join(""))
            })
            .flatten()
            .collect()
    }
}

fn is_valid_message(matches_42: &[String], matches_31: &[String], message: &str) -> bool {
    let chunk_length = matches_42[0].len();
    let message_chunks = message.as_bytes().chunks_exact(chunk_length);
    let starting_matches = message_chunks
        .clone()
        .take_while(|&chunk| matches_42.iter().any(|target| target.as_bytes() == chunk))
        .count();
    let ending_matches = message_chunks
        .rev()
        .take_while(|&chunk| matches_31.iter().any(|target| target.as_bytes() == chunk))
        .count();
    starting_matches > 0
        && ending_matches > 0
        && starting_matches > ending_matches
        && starting_matches + ending_matches >= message.len() / chunk_length
}

fn main() {
    let input = read_to_string("../input").expect("Failed to open input file");
    let mut input_iter = input.split("\n\n");
    let rules: HashMap<u32, _> = input_iter
        .next()
        .expect("Input was empty")
        .lines()
        .map(|rule| {
            let (pos, rule) = rule.split_at(rule.find(':').expect("Could not parse rule"));
            (
                pos.parse().expect("Could not parse rule number"),
                &rule[2..],
            )
        })
        .collect();
    let possible_matches_42 = generate_possible_matches(&rules, 42);
    let possible_matches_31 = generate_possible_matches(&rules, 31);
    let matching_messages = input_iter
        .next()
        .expect("Input did not list received messages")
        .lines()
        .filter(|message| is_valid_message(&possible_matches_42, &possible_matches_31, message))
        .count();

    println!("The number of matching messages is {}!", matching_messages);
}
