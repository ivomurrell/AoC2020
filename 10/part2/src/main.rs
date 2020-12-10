use std::{fs::read_to_string, iter};

fn main() {
    let mut adapters: Vec<u32> = read_to_string("../input")
        .expect("Failed to open input file")
        .lines()
        .map(|jolts| jolts.parse().expect("Could not parse joltage"))
        .collect();
    adapters.sort();
    let jolt_max = *adapters.iter().max().expect("No joltages listed in input");
    let (permutation_count, _, _) = adapters.into_iter().chain(iter::once(jolt_max + 3)).fold(
        (1u64, 0u32, 0),
        |(acc, contiguous_jolt_1, last), adapter| {
            let difference = adapter - last;
            if difference == 1 {
                (acc, contiguous_jolt_1 + 1, adapter)
            } else if contiguous_jolt_1 > 1 {
                let contiguous_permutations = match contiguous_jolt_1 {
                    2 => 2,
                    3 => 4,
                    4 => 7,
                    5 => 13,
                    6 => 24,
                    _ => unimplemented!("I don't understand how this pattern works!"),
                };
                (acc * contiguous_permutations, 0, adapter)
            } else {
                (acc, 0, adapter)
            }
        },
    );
    println!("The number of permutations is {}!", permutation_count);
}
