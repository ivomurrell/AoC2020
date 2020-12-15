use std::{collections::HashMap, fs::read_to_string};

fn parse_mask(mask: &str) -> (u64, u64) {
    mask.match_indices(&['0', '1'][..]).fold(
        (!0u64, 0u64),
        |(mask_0, mask_1), (i, bin)| match bin {
            "0" => (mask_0 ^ 1 << (35 - i), mask_1),
            "1" => (mask_0, mask_1 | 1 << (35 - i)),
            _ => unreachable!(),
        },
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
            let address: u32 = instruction[4..delimiter_close]
                .parse()
                .expect("Could not parse address");
            let value: u64 = instruction[delimiter_close + 4..]
                .parse()
                .expect("Could not parse value");
            let masked = (value & mask.0) | mask.1;
            memory.insert(address, masked);
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
