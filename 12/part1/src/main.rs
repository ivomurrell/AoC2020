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
                "N" => ((x + amount, y), r),
                "S" => ((x - amount, y), r),
                "E" => ((x, y + amount), r),
                "W" => ((x, y - amount), r),
                "L" => ((x, y), r - amount),
                "R" => ((x, y), r + amount),
                "F" => match (r % 360).abs() / 90 {
                    0 => ((x + amount, y), r),
                    1 => ((x, y + amount), r),
                    2 => ((x - amount, y), r),
                    3 => ((x, y - amount), r),
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
