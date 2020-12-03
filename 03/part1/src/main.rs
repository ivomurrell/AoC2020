use anyhow::{self, Context as _};
use std::fs::read_to_string;

fn main() -> anyhow::Result<()> {
    let map = read_to_string("../input").context("Failed to open input file")?;
    let pattern_length = map.lines().next().context("Input was empty")?.len();
    let tree_encounters = map
        .lines()
        .enumerate()
        .skip(1)
        // Assume that all lines are the same length
        .filter(|(i, row)| row.as_bytes()[(i * 3) % pattern_length] == b'#')
        .count();
    println!("Would encounter {} trees on the slope!", tree_encounters);
    Ok(())
}
