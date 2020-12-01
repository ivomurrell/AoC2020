use anyhow::{self, Context as _};
use std::fs::read_to_string;

fn main() -> anyhow::Result<()> {
    let report = read_to_string("../input").context("Failed to open input file")?;
    let expenses = report
        .lines()
        .map(|record| record.parse().context("Failed to parse record"))
        .collect::<anyhow::Result<Vec<u32>>>()?;
    for (i, expense) in expenses.iter().enumerate() {
        let target = 2020 - expense;
        if let Some(found) = expenses[i + 1..].iter().find(|entry| entry == &&target) {
            println!(
                "Found entries {} and {} that multiply to {}!",
                expense,
                found,
                expense * found
            );
            return Ok(());
        }
    }
    Err(anyhow::anyhow!("Could not find matching pair of expenses"))
}
