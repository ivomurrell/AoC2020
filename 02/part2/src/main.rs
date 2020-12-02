use anyhow::{self, Context as _};
use regex::Regex;
use std::fs::read_to_string;

fn main() -> anyhow::Result<()> {
    let re = Regex::new(r"(?P<first>\d+)-(?P<second>\d+) (?P<target>\w): (?P<password>\w*)")
        .context("Failed to compile regex")?;
    let valid_count = read_to_string("../input")
        .context("Failed to open input file")?
        .lines()
        .filter_map(|raw_policy| {
            let policy = re.captures(raw_policy)?;
            let first: usize = policy.name("first")?.as_str().parse().ok()?;
            let second: usize = policy.name("second")?.as_str().parse().ok()?;
            let target: char = policy.name("target")?.as_str().parse().ok()?;
            let password = policy.name("password")?.as_str();

            let first_matches = password.chars().nth(first - 1)? == target;
            let second_matches = password.chars().nth(second - 1)? == target;
            if (first_matches || second_matches) && !(first_matches && second_matches) {
                Some(())
            } else {
                None
            }
        })
        .count();
    println!("{} valid passwords found!", valid_count);
    Ok(())
}
