use anyhow::{self, Context as _};
use std::fs::read_to_string;

const SLOPES: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

fn main() -> anyhow::Result<()> {
    let map = read_to_string("../input").context("Failed to open input file")?;
    let pattern_length = map.lines().next().context("Input was empty")?.len();
    let tree_product: usize = SLOPES
        .iter()
        .map(|(right, down)| {
            let tree_encounters = map
                .lines()
                .step_by(*down)
                .enumerate()
                .skip(1)
                // Assume that all lines are the same length
                .filter(|(i, row)| row.as_bytes()[(i * right) % pattern_length] == b'#')
                .count();
            println!(
                "When going {} right and {} down, {} trees are encountered.",
                right, down, tree_encounters
            );
            tree_encounters
        })
        .product();
    println!("The product of all the slopes is {}!", tree_product);
    Ok(())
}
