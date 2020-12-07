use regex::Regex;
use std::{collections::HashMap, fs::read_to_string};

fn count_inner_bags(bag_map: &HashMap<String, Vec<(&str, u32)>>, target_bag: &str) -> u32 {
    bag_map[target_bag]
        .iter()
        .map(|(bag_name, bag_count)| count_inner_bags(bag_map, bag_name) * bag_count)
        .sum::<u32>()
        + 1
}

fn main() {
    let bag_name_re =
        Regex::new(r"^(\w+ \w+) bags contain").expect("Bag name regex failed to compile");
    let containing_bags_re =
        Regex::new(r"(\d+) (\w+ \w+) bags?").expect("Containing bags regex failed to compile");
    let rules = read_to_string("../input").expect("Failed to open input file");
    let bag_map = rules
        .lines()
        .map(|rule| {
            let bag_name = bag_name_re
                .captures(rule)
                .expect("Could not parse bag name")[1]
                .to_owned();
            let containing_bags: Vec<_> = containing_bags_re
                .captures_iter(rule)
                .map(|caps| {
                    (
                        caps.get(2)
                            .expect("Could not parse containing bag name")
                            .as_str(),
                        caps[1]
                            .parse()
                            .expect("Could not parse containing bag number"),
                    )
                })
                .collect();
            (bag_name, containing_bags)
        })
        .collect();
    let bag_count = count_inner_bags(&bag_map, "shiny gold") - 1;
    println!("There must be {} bags inside a shiny gold bag!", bag_count);
}
