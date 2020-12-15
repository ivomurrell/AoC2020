use std::{collections::HashMap, fs::read_to_string};

fn parse_mask(mask: &str) -> (u64, Vec<usize>) {
    (
        mask.match_indices('1')
            .fold(0u64, |mask_1, (i, _)| mask_1 | 1 << (35 - i)),
        mask.match_indices('X').map(|(i, _)| 35 - i).collect(),
    )
}

fn main() {
    let initialisation_program = read_to_string("../input").expect("Failed to open input file");
    let mut memory = HashMap::new();
    let mut instructions = initialisation_program.lines();
    let initial_mask = &instructions.next().expect("Instruction file was empty")[7..];
    instructions.fold(parse_mask(initial_mask), |mask, instruction| {
        if instruction.starts_with("mem") {
            let delimiter_close = instruction
                .find(']')
                .expect("Could not find closing delimiter for address");
            let address: u64 = instruction[4..delimiter_close]
                .parse()
                .expect("Could not parse address");
            let value: u64 = instruction[delimiter_close + 4..]
                .parse()
                .expect("Could not parse value");
            let masked = address | mask.0;
            let mut floating_addresses = vec![masked];
            for floating_pos in mask.1.iter() {
                floating_addresses = floating_addresses
                    .into_iter()
                    .map(|floating| {
                        vec![
                            floating & (!0u64 ^ (1 << floating_pos)),
                            floating | 1 << floating_pos,
                        ]
                    })
                    .flatten()
                    .collect();
            }
            for address in floating_addresses {
                memory.insert(address, value);
            }
        } else if instruction.starts_with("mask") {
            return parse_mask(&instruction[7..]);
        } else {
            panic!("Unrecognised instruction");
        }
        mask
    });

    let result: u64 = memory.values().sum();
    println!("The sum of all values in memory is {}!", result);
}
