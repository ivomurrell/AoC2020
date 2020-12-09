use arraydeque::{ArrayDeque, Wrapping};
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("../input").expect("Failed to open input file");
    let mut num_iter = input
        .lines()
        .map(|num| num.parse().expect("Could not parse number"));
    let mut num_buffer: ArrayDeque<[u32; 25], Wrapping> = num_iter.by_ref().take(25).collect();
    let unsummable_num = num_iter
        .find(|target| {
            for (i, num_1) in num_buffer.iter().enumerate() {
                if num_buffer
                    .iter()
                    .skip(i + 1)
                    .any(|num_2| num_1 + num_2 == *target)
                {
                    num_buffer.push_back(*target);
                    return false;
                }
            }
            return true;
        })
        .expect("Could not find unsummable number");

    println!(
        "The first number that cannot be summed is {}!",
        unsummable_num
    );
}
