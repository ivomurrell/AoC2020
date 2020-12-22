use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

fn main() {
    let mut ingredient_list = Vec::new();
    let mut allergen_list: HashMap<_, Vec<_>> = HashMap::new();
    let input = read_to_string("../input").expect("Failed to open input file");
    for food in input.lines() {
        let (ingredients, allergens) = food
            .split(" (contains ")
            .collect_tuple()
            .expect("Allergens were not found");
        let mut ingredients: Vec<_> = ingredients.split_ascii_whitespace().collect();
        for allergen in allergens.split(", ") {
            let allergen = allergen.trim_end_matches(')');
            allergen_list
                .entry(allergen)
                .or_default()
                .push(ingredients.clone());
        }
        ingredient_list.append(&mut ingredients);
    }

    let possible_allergens: HashSet<&str> = allergen_list
        .values()
        .flat_map(|ingredients| {
            ingredients[0]
                .clone()
                .into_iter()
                .filter(move |ingredient| {
                    ingredients[1..]
                        .iter()
                        .all(|list| list.contains(ingredient))
                })
        })
        .collect();

    let ingredient_set: HashSet<_> = ingredient_list.iter().copied().collect();
    let no_allergens: Vec<_> = ingredient_set.difference(&possible_allergens).collect();
    let no_allergen_count = ingredient_list
        .into_iter()
        .filter(|ingredient| no_allergens.contains(&&ingredient))
        .count();

    println!(
        "Ingredients that cannot contain allergens appear {} times!",
        no_allergen_count
    );
}
