use std::fs::read_to_string;

fn main() {
    let mut timetable: Vec<(usize, usize)> = read_to_string("../input")
        .expect("Failed to open input file")
        .lines()
        .nth(1)
        .expect("Input file did not include timetable")
        .split(',')
        .enumerate()
        .filter_map(|(delay, id)| {
            let id = id.parse().ok()?;
            Some((id, (id - delay % id) % id))
        })
        .collect();
    timetable.sort_by_key(|(id, _)| *id);
    let mut timetable_iter = timetable.into_iter().rev();
    let (mut current_product, mut test_time) = timetable_iter.next().expect("Timetable was empty");
    for (id, delay) in timetable_iter {
        while test_time % id != delay {
            test_time += current_product
        }
        current_product *= id;
    }
    println!(
        "The earliest timestamp that meets the criteria is {}!",
        test_time
    );
}
