use std::fs::read_to_string;

fn main() {
    let input = read_to_string("../input").expect("Failed to open input file");
    let mut notes = input.lines();
    let earliest_timestamp: u32 = notes
        .next()
        .expect("Input file was empty")
        .parse()
        .expect("Could not parse timestamp");
    let (earliest_bus, time_to_wait) = notes
        .next()
        .expect("Input file did not include timetable")
        .split(',')
        .filter_map(|id| id.parse().ok())
        .map(|id: u32| (id, id - earliest_timestamp % id))
        .min_by_key(|(_, time_to_wait)| *time_to_wait)
        .expect("Timetable was empty");
    println!(
        "The earliest bus is {} and you'd have to wait {} minutes, so the result is {}!",
        earliest_bus,
        time_to_wait,
        earliest_bus * time_to_wait
    );
}
