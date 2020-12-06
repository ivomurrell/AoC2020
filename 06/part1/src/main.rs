use std::fs::read_to_string;

fn main() {
    let group_sum: usize = read_to_string("../input")
        .expect("Failed to open input file")
        .split("\n\n")
        .map(|group| {
            let mut group = group.as_bytes().to_vec();
            group.sort();
            group.dedup();
            group
                .into_iter()
                .filter(|answer| answer.is_ascii_alphabetic())
                .count()
        })
        .sum();
    println!("The total number of 'yes's was {}!", group_sum);
}
