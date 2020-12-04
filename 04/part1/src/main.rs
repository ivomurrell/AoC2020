use std::fs::read_to_string;

const REQUIRED_FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

fn main() {
    let batches = read_to_string("../input").expect("Failed to open input file");
    let valid_count = batches
        .split_terminator("\n\n")
        .filter(|passport| {
            let field_names: Vec<_> = passport
                .split_whitespace()
                .filter_map(|field| field.split(':').next())
                .collect();
            REQUIRED_FIELDS
                .iter()
                .all(|field| field_names.contains(field))
        })
        .count();
    println!("{} valid passports found!", valid_count);
}
