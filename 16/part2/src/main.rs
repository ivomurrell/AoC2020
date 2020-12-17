use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    ops::RangeInclusive,
};

#[derive(Debug, Clone)]
struct TicketField {
    name: String,
    ranges: (RangeInclusive<u64>, RangeInclusive<u64>),
}

fn main() {
    let fields_re =
        Regex::new(r"(.+): (\d+)-(\d+) or (\d+)-(\d+)").expect("Could not compile regex");
    let input = read_to_string("../input").expect("Failed to open input file");

    // Parse input
    let mut notes = input.split("\n\n");
    let fields = notes.next().expect("Input was empty");
    let ticket_fields: Vec<TicketField> = fields
        .lines()
        .map(|field| {
            let caps = fields_re.captures(field).expect("Could not parse field");
            TicketField {
                name: caps[1].to_owned(),
                ranges: (
                    caps[2]
                        .parse()
                        .expect("Could not parse first starting bound")
                        ..=caps[3].parse().expect("Could not parse first ending bound"),
                    caps[4]
                        .parse()
                        .expect("Could not parse second starting bound")
                        ..=caps[5]
                            .parse()
                            .expect("Could not parse second ending bound"),
                ),
            }
        })
        .collect();
    let own_ticket: Vec<u64> = notes
        .next()
        .expect("Own ticket not listed")
        .lines()
        .nth(1)
        .expect("Own ticket has no fields")
        .split(',')
        .map(|num| num.parse().expect("Could not parse own ticket"))
        .collect();
    let mut valid_tickets = notes
        .next()
        .expect("Nearby tickets not listed")
        .lines()
        .skip(1)
        .map(|line| {
            line.split(',')
                .map(|field| field.parse().expect("Could not parse ticket"))
                .collect::<Vec<u64>>()
        })
        .filter(|ticket| {
            ticket.iter().all(|val| {
                ticket_fields.iter().any(|TicketField { ranges, .. }| {
                    ranges.0.contains(&val) || ranges.1.contains(&val)
                })
            })
        });

    // Check possible positions of each field independently
    let first_ticket = valid_tickets.next().expect("No valid tickets found");
    let mut possible_orders: Vec<_> = ticket_fields
        .iter()
        .map(|ticket_field| {
            let possible_positions: HashSet<_> = first_ticket
                .iter()
                .enumerate()
                .filter_map(move |(i, val)| {
                    if ticket_field.ranges.0.contains(val) || ticket_field.ranges.1.contains(val) {
                        Some(i)
                    } else {
                        None
                    }
                })
                .collect();
            (ticket_field, possible_positions)
        })
        .collect();
    for ticket in valid_tickets {
        for (TicketField { ranges, .. }, orders) in possible_orders.iter_mut() {
            orders.retain(|&pos| {
                let val = &ticket[pos];
                ranges.0.contains(val) || ranges.1.contains(val)
            })
        }
    }

    // Use process of elimination to find position each field can singularly be
    let mut order_map = HashMap::new();
    let mut found_positions = HashSet::new();
    while order_map.len() < ticket_fields.len() {
        for (ticket_field, positions) in possible_orders.iter() {
            let remaining_positions: Vec<_> =
                positions.difference(&found_positions).copied().collect();
            if remaining_positions.len() == 1 {
                let position = remaining_positions[0];
                order_map.insert(position, ticket_field);
                found_positions.insert(position);
            }
        }
    }

    // Calculate final result from depature fields
    let departure_product: u64 = order_map
        .into_iter()
        .filter_map(|(field_position, ticket_field)| {
            if ticket_field.name.starts_with("departure") {
                Some(own_ticket[field_position])
            } else {
                None
            }
        })
        .product();

    println!(
        "The product of the depature fields on the ticket is {}!",
        departure_product
    );
}
