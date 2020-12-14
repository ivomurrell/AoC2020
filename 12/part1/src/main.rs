use std::fs::read_to_string;

fn main() {
    let ((x, y), _) = read_to_string("../input")
        .expect("Failed to open input file")
        .lines()
        .map(|instruction| {
            let (action, amount) = instruction.split_at(1);
            (
                action,
                amount
                    .parse::<i32>()
                    .expect("Could not parse instruction amount"),
            )
        })
        .fold(
            ((0i32, 0i32), 90i32),
            |((x, y), r), (action, amount)| match action {
                "N" => ((x, y + amount), r),
                "S" => ((x, y - amount), r),
                "E" => ((x + amount, y), r),
                "W" => ((x - amount, y), r),
                "L" => ((x, y), r - amount),
                "R" => ((x, y), r + amount),
                "F" => match (r % 360).abs() / 90 {
                    0 => ((x, y + amount), r),
                    1 => ((x + amount, y), r),
                    2 => ((x, y - amount), r),
                    3 => ((x - amount, y), r),
                    _ => unreachable!(),
                },
                _ => panic!("Unrecognised action"),
            },
        );
    println!(
        "The ship's location is ({}, {}), so a Manhattan distance of {}!",
        x,
        y,
        x.abs() + y.abs()
    );
}
