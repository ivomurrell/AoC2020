use std::fs::read_to_string;

fn main() {
    let shared_yes_sum: usize = read_to_string("../input")
        .expect("Failed to open input file")
        .split("\n\n")
        .map(|group| {
            let mut people = group.lines();
            let first_answers = people.next().expect("Group was empty").as_bytes().to_vec();
            people
                .fold(first_answers, |shared_answers, answers| {
                    shared_answers
                        .into_iter()
                        .filter(|answer| answers.contains(*answer as char))
                        .collect()
                })
                .len()
        })
        .sum();
    println!("The total number of shared 'yes's was {}!", shared_yes_sum);
}
