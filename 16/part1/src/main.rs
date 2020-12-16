use regex::Regex;
use std::{fs::read_to_string, ops::RangeInclusive};

fn main() {
    let fields_re = Regex::new(r".+: (\d+)-(\d+) or (\d+)-(\d+)").expect("Could not compile regex");
    let input = read_to_string("../input").expect("Failed to open input file");
    let mut notes = input.split("\n\n");
    let fields = notes.next().expect("Input was empty");
    let ranges: Vec<RangeInclusive<u64>> = fields
        .lines()
        .map(|field| {
            let caps = fields_re.captures(field).expect("Could not parse field");
            vec![
                caps[1]
                    .parse()
                    .expect("Could not parse first starting bound")
                    ..=caps[2].parse().expect("Could not parse first ending bound"),
                caps[3]
                    .parse()
                    .expect("Could not parse second starting bound")
                    ..=caps[4]
                        .parse()
                        .expect("Could not parse second ending bound"),
            ]
        })
        .flatten()
        .collect();
    let error_rate: u64 = notes
        .nth(1)
        .expect("Nearby tickets not listed")
        .split_terminator(&[',', '\n'][..])
        .skip(1)
        .map(|field| field.parse().expect("Could not parse ticket"))
        .filter(|ticket| !ranges.iter().any(|range| range.contains(ticket)))
        .sum();

    println!("The ticket scanning error rate was {}!", error_rate);
}
