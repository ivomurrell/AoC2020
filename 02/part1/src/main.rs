use anyhow::{self, Context as _};
use regex::Regex;
use std::fs::read_to_string;

fn main() -> anyhow::Result<()> {
    let re = Regex::new(r"(?P<min>\d+)-(?P<max>\d+) (?P<target>\w): (?P<password>\w*)")
        .context("Failed to compile regex")?;
    let valid_count = read_to_string("../input")
        .context("Failed to open input file")?
        .lines()
        .filter_map(|raw_policy| {
            let policy = re.captures(raw_policy)?;
            let min: usize = policy.name("min")?.as_str().parse().ok()?;
            let max: usize = policy.name("max")?.as_str().parse().ok()?;
            let target = policy.name("target")?.as_str();
            let password = policy.name("password")?.as_str();

            let target_occurences = password.matches(target).count();
            if target_occurences >= min && target_occurences <= max {
                Some(())
            } else {
                None
            }
        })
        .count();
    println!("{} valid passwords found!", valid_count);
    Ok(())
}
