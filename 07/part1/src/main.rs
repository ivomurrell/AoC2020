use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

fn find_container_bags(bag_map: &HashMap<String, Vec<&str>>, target_bag: &str) -> HashSet<String> {
    bag_map
        .iter()
        .filter_map(|(bag_name, containing)| {
            if containing.contains(&target_bag) {
                let mut found_bags = find_container_bags(bag_map, bag_name);
                found_bags.insert(bag_name.clone());
                Some(found_bags)
            } else {
                None
            }
        })
        .flatten()
        .collect()
}

fn main() {
    let bag_name_re =
        Regex::new(r"^(\w+ \w+) bags contain").expect("Bag name regex failed to compile");
    let containing_bags_re =
        Regex::new(r"\d+ (\w+ \w+) bags?").expect("Containing bags regex failed to compile");
    let rules = read_to_string("../input").expect("Failed to open input file");
    let bag_map: HashMap<_, _> = rules
        .lines()
        .map(|rule| {
            let bag_name = bag_name_re
                .captures(rule)
                .expect("Could not parse bag name")[1]
                .to_owned();
            let containing_bags: Vec<_> = containing_bags_re
                .captures_iter(rule)
                .map(|caps| {
                    caps.get(1)
                        .expect("Could not parse containing bag")
                        .as_str()
                })
                .collect();
            (bag_name, containing_bags)
        })
        .collect();
    let bag_count = find_container_bags(&bag_map, "shiny gold").len();
    println!("{} bag colours can contain shiny gold bags!", bag_count);
}
