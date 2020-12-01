use anyhow::{self, Context as _};
use std::fs::read_to_string;

fn main() -> anyhow::Result<()> {
    let report = read_to_string("../input").context("Failed to open input file")?;
    let expenses = report
        .lines()
        .map(|record| record.parse().context("Failed to parse record"))
        .collect::<anyhow::Result<Vec<u32>>>()?;
    for (i, expense_1) in expenses.iter().enumerate() {
        for (j, expense_2) in expenses[i + 1..].iter().enumerate() {
            if expense_1 + expense_2 > 2020 {
                continue;
            }
            let target = 2020 - expense_1 - expense_2;
            if let Some(found) = expenses[j + 1..].iter().find(|entry| entry == &&target) {
                println!(
                    "Found entries {}, {}, and {} that multiply to {}!",
                    expense_1,
                    expense_2,
                    found,
                    expense_1 * expense_2 * found
                );
                return Ok(());
            }
        }
    }
    Err(anyhow::anyhow!("Could not find matching pair of expenses"))
}
