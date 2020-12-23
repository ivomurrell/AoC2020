use std::fs::read_to_string;

fn main() {
    let mut cups: Vec<_> = read_to_string("../input")
        .expect("Failed to open input file")
        .chars()
        .map(|num| num.to_digit(10).unwrap() as usize)
        .collect();
    let cups_len = cups.len();

    let mut current = *cups.first().unwrap();
    for _ in 0..100 {
        let current_index = cups.iter().position(|&cup| cup == current).unwrap() + 1;
        let mut moving_cups: Vec<_> = cups
            .drain(current_index..(current_index + 3).min(cups_len))
            .collect();
        if current_index + 3 > cups_len {
            moving_cups.extend(cups.drain(..current_index + 3 - cups_len));
        }
        let min_cup = *cups.iter().min().unwrap();
        let mut destination = current - 1;
        while !cups.contains(&destination) {
            if destination < min_cup {
                destination = *cups.iter().max().unwrap();
            } else {
                destination -= 1;
            }
        }
        let destination_index = cups.iter().position(|&cup| cup == destination).unwrap();
        for (j, moving_cup) in moving_cups.into_iter().enumerate() {
            cups.insert(destination_index + j + 1, moving_cup);
        }
        current = *cups
            .iter()
            .cycle()
            .skip_while(|&cup| *cup != current)
            .nth(1)
            .unwrap();
    }

    let final_labels: String = cups
        .iter()
        .cycle()
        .skip_while(|&cup| *cup != 1)
        .skip(1)
        .take(cups_len - 1)
        .map(ToString::to_string)
        .collect();
    println!(
        "The labels on the cups after 100 moves is {}!",
        final_labels
    );
}
