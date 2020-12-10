use std::fs::read_to_string;

fn main() {
    let mut adapters: Vec<u32> = read_to_string("../input")
        .expect("Failed to open input file")
        .lines()
        .map(|jolts| jolts.parse().expect("Could not parse joltage"))
        .collect();
    adapters.sort();
    let (jolt_1_count, jolt_3_count, _) = adapters.into_iter().fold(
        (0u32, 0u32, 0),
        |(jolt_1_count, jolt_3_count, last), adapter| {
            let difference = adapter - last;
            if difference == 1 {
                (jolt_1_count + 1, jolt_3_count, adapter)
            } else if difference == 3 {
                (jolt_1_count, jolt_3_count + 1, adapter)
            } else {
                (jolt_1_count, jolt_3_count, adapter)
            }
        },
    );
    // Add difference between last adapter and device's built-in adapter.
    let jolt_3_count = jolt_3_count + 1;

    println!(
        "{} adapters with 1-jolt difference and {} with 3-jolt difference. Solution is {}!",
        jolt_1_count,
        jolt_3_count,
        jolt_1_count * jolt_3_count
    );
}
