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
    let possible_matches = generate_possible_matches(&rules, 0);
    let matching_messages = input_iter
        .next()
        .expect("Input did not list received messages")
        .lines()
        .filter(|message| possible_matches.iter().any(|possible| possible == *message))
        .count();

    println!("The number of matching messages is {}!", matching_messages);
}
