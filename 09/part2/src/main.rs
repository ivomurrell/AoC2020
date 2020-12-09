use arraydeque::{ArrayDeque, Wrapping};
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("../input").expect("Failed to open input file");
    let numbers: Vec<u64> = input
        .lines()
        .map(|num| num.parse().expect("Could not parse number"))
        .collect();
    let mut num_buffer: ArrayDeque<[_; 25], Wrapping> = numbers[..25].iter().collect();
    let unsummable_num = numbers[25..]
        .iter()
        .find(|target| {
            for (i, num_1) in num_buffer.iter().enumerate() {
                if num_buffer
                    .iter()
                    .skip(i + 1)
                    .any(|num_2| *num_1 + *num_2 == **target)
                {
                    num_buffer.push_back(*target);
                    return false;
                }
            }
            return true;
        })
        .expect("Could not find unsummable number");

    let num_length = numbers.len();
    'outer: for i in 0..num_length {
        for j in i..num_length {
            let contiguous_iter = numbers[i..j].iter();
            let contiguous_sum: u64 = contiguous_iter.clone().sum();
            if contiguous_sum == *unsummable_num {
                println!(
                    "The numbers {:?} sum to {}, so the encryption weakness is {}!",
                    numbers[i..j].to_vec(),
                    unsummable_num,
                    contiguous_iter.clone().min().unwrap() + contiguous_iter.clone().max().unwrap()
                );
                return;
            } else if contiguous_sum > *unsummable_num {
                continue 'outer;
            }
        }
    }
    println!("Could not find encryption weakness");
}
