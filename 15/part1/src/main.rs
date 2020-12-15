use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let mut memory: HashMap<usize, _> = read_to_string("../input")
        .expect("Failed to open input file")
        .split(',')
        .enumerate()
        .map(|(step, num)| (num.parse().expect("Could not parse number"), step))
        .collect();
    let mut next = 0;
    for step in memory.len()..2019 {
        next = match memory.insert(next, step) {
            Some(last) => step - last,
            None => 0,
        }
    }
    println!("2020th number in the memory game is {}!", next);
}
