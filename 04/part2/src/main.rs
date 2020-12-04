use lazy_static::lazy_static;
use regex::Regex;
use std::fs::read_to_string;

const REQUIRED_FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
const EYE_COLOURS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

fn field_validator((name, value): (&str, &str)) -> bool {
    lazy_static! {
        static ref HAIR_RE: Regex =
            Regex::new(r"^#[0-9a-f]{6}$").expect("Hair regex failed to compile");
        static ref ID_RE: Regex =
            Regex::new(r"^\d{9}$").expect("Password ID regex failed to compile");
    }

    fn string_in_range(s: &str, min: i32, max: i32) -> bool {
        s.parse().map_or(false, |x| min <= x && x <= max)
    };

    match name {
        "byr" => string_in_range(value, 1920, 2002),
        "iyr" => string_in_range(value, 2010, 2020),
        "eyr" => string_in_range(value, 2020, 2030),
        "hgt" => {
            if value.ends_with("cm") {
                string_in_range(&value[..value.len() - 2], 150, 193)
            } else if value.ends_with("in") {
                string_in_range(&value[..value.len() - 2], 59, 76)
            } else {
                false
            }
        }
        "hcl" => HAIR_RE.is_match(value),
        "ecl" => EYE_COLOURS.contains(&value),
        "pid" => ID_RE.is_match(value),
        "cid" => true,
        _ => false,
    }
}

fn main() {
    let batches = read_to_string("../input").expect("Failed to open input file");
    let valid_count = batches
        .split_terminator("\n\n")
        .filter(|passport| {
            let fields: Vec<_> = passport
                .split_whitespace()
                .filter_map(|field| {
                    let mut split = field.split(':');
                    Some((split.next()?, split.next()?))
                })
                .collect();
            REQUIRED_FIELDS
                .iter()
                .all(|required_field| fields.iter().any(|field| field.0 == *required_field))
                && fields.into_iter().all(field_validator)
        })
        .inspect(|valid_passport| println!("valid: {}\n", valid_passport))
        .count();
    println!("{} valid passports found!", valid_count);
}
