use std::{collections::HashSet, fs::read_to_string};

fn main() {
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
    let instructions_end = instructions.len();

    for swapped_instruction in 0..instructions_end {
        if instructions[swapped_instruction].0 == "acc" {
            continue;
        }
        let mut instruction_history: HashSet<usize> = HashSet::new();
        let mut acc = 0;
        let mut instruction_reg = 0;
        while let None = instruction_history.replace(instruction_reg) {
            let (mut op, arg) = instructions[instruction_reg];
            if instruction_reg == swapped_instruction {
                if op == "jmp" {
                    op = "nop"
                } else if op == "nop" {
                    op = "jmp"
                } else {
                    unreachable!("Should have skipped this iteration if op was 'acc'");
                }
            }
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

            if instruction_reg == instructions_end {
                println!(
                    "Fixed the infinite loop by changing the operation at position {}. \
                    The final value in the accumulator was {}!",
                    swapped_instruction, acc
                );
                return;
            }
        }
    }
    println!("Failed to find")
}
