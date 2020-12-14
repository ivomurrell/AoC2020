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
            ((0i32, 0i32), (10i32, 1i32)),
            |((x, y), (way_x, way_y)), (action, amount)| match action {
                "N" => ((x, y), (way_x, way_y + amount)),
                "S" => ((x, y), (way_x, way_y - amount)),
                "E" => ((x, y), (way_x + amount, way_y)),
                "W" => ((x, y), (way_x - amount, way_y)),
                "L" => match amount {
                    90 => ((x, y), (-way_y, way_x)),
                    180 => ((x, y), (-way_x, -way_y)),
                    270 => ((x, y), (way_y, -way_x)),
                    _ => panic!("Unexpected rotation angle!"),
                },
                "R" => match amount {
                    90 => ((x, y), (way_y, -way_x)),
                    180 => ((x, y), (-way_x, -way_y)),
                    270 => ((x, y), (-way_y, way_x)),
                    _ => panic!("Unexpected rotation angle!"),
                },
                "F" => ((x + way_x * amount, y + way_y * amount), (way_x, way_y)),
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
