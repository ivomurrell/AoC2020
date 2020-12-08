use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let mut instruction_history: HashSet<usize> = HashSet::new();
    let mut acc = 0;
    let mut instruction_reg = 0;
    let instructions_input = read_to_string("../input").expect("Failed to open input file");
    let instructions: Vec<(&str, i32)> = instructions_input
        .lines()
        .map(|instruction| {
            let (op, arg) = instruction.split_at(3);
            (
                op,
                arg[1..]
                    .parse()
                    .expect("Could not parse instruction argument"),
            )
        })
        .collect();

    while let None = instruction_history.replace(instruction_reg) {
        let (op, arg) = instructions[instruction_reg];
        match op {
            "acc" => {
                acc += arg;
                instruction_reg += 1;
            }
            // We have to assume the input doesn't take us to a negative instruction.
            "jmp" => instruction_reg = (instruction_reg as i32 + arg) as usize,
            "nop" => instruction_reg += 1,
            _ => panic!("Unrecognised operation"),
        }
    }

    println!(
        "Before the infinite loop, the value in the accumulator was {}!",
        acc
    );
}
