use itertools::Itertools;
use std::{
    collections::{BTreeMap, HashMap},
    fs::read_to_string,
};

fn main() {
    let mut allergen_foods: HashMap<_, Vec<_>> = HashMap::new();
    let input = read_to_string("../input").expect("Failed to open input file");
    for food in input.lines() {
        let (ingredients, allergens) = food
            .split(" (contains ")
            .collect_tuple()
            .expect("Allergens were not found");
        let ingredients: Vec<_> = ingredients.split_ascii_whitespace().collect();
        let mut allergens: Vec<_> = allergens.split(", ").collect();
        // Strip closing bracket from last allergen in list
        let last_allergen = allergens.last_mut().unwrap();
        *last_allergen = &last_allergen[..last_allergen.len() - 1];
        for allergen in allergens.iter().copied() {
            allergen_foods
                .entry(allergen)
                .or_default()
                .push(ingredients.clone());
        }
    }

    let mut filtered_allergens: HashMap<_, Vec<_>> = allergen_foods
        .iter()
        .map(|(allergen, ingredients)| {
            (
                allergen,
                ingredients[0]
                    .clone()
                    .into_iter()
                    .filter(move |ingredient| {
                        ingredients[1..]
                            .iter()
                            .all(|list| list.contains(ingredient))
                    })
                    .collect(),
            )
        })
        .collect();

    let mut definite_allergens = BTreeMap::new();
    while definite_allergens.len() != filtered_allergens.len() {
        for (allergen, ingredients) in filtered_allergens.clone() {
            if ingredients.len() == 1 {
                let definite_ingredient = ingredients[0];
                definite_allergens.insert(allergen, definite_ingredient);
                for filtered_ingredients in filtered_allergens.values_mut() {
                    if let Some(ingredient_index) = filtered_ingredients
                        .iter()
                        .position(|ing| ing == &definite_ingredient)
                    {
                        filtered_ingredients.remove(ingredient_index);
                    }
                }
            }
        }
    }

    let canonical_dangerous_ingredients = definite_allergens.values().join(",");
    println!(
        "The canonical dangerous ingredients list is \"{}\"!",
        canonical_dangerous_ingredients
    );
}
