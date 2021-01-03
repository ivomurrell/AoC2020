use nom::{branch::alt, bytes::complete::tag, multi::fold_many1, IResult};
use std::{collections::HashSet, fs::read_to_string};

fn parse_direction(input: &str) -> IResult<&str, &str> {
    alt((
        tag("se"),
        tag("sw"),
        tag("ne"),
        tag("nw"),
        tag("e"),
        tag("w"),
    ))(input)
}

fn parse_tile_position(input: &str) -> IResult<&str, (i64, i64)> {
    fold_many1(
        parse_direction,
        (0, 0),
        |(current_x, current_y), direction| match direction {
            "se" => (current_x + 1, current_y - 1),
            "sw" => (current_x - 1, current_y - 1),
            "ne" => (current_x + 1, current_y + 1),
            "nw" => (current_x - 1, current_y + 1),
            "e" => (current_x + 2, current_y),
            "w" => (current_x - 2, current_y),
            _ => unreachable!(),
        },
    )(input)
}

fn main() {
    let mut flipped_set = HashSet::new();
    read_to_string("../input")
        .expect("Failed to open input file")
        .lines()
        .map(|directions| {
            parse_tile_position(directions)
                .expect("Failed to parse directions")
                .1
        })
        .for_each(|flipped_tile| {
            if !flipped_set.remove(&flipped_tile) {
                flipped_set.insert(flipped_tile);
            }
        });
    let black_tiles = flipped_set.len();
    println!("The number of black tiles is {}!", black_tiles);
}
