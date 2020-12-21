use itertools::Itertools;
use std::fs::read_to_string;

fn main() {
    let tiles: Vec<_> = read_to_string("../input")
        .expect("Failed to open input file")
        .split_terminator("\n\n")
        .map(|tile| {
            let mut lines = tile.lines();
            let tile_number: u64 = lines.next()?[5..9]
                .parse()
                .expect("Could not parse tile number");
            let mut borders = [String::new(), String::new(), String::new(), String::new()];
            let first_line = lines.next()?;
            let line_length = first_line.len();
            borders[0] = first_line.to_owned();
            borders[1].push(borders[0].chars().last()?);
            borders[3].push(borders[0].chars().next()?);
            for line in lines.by_ref().take(line_length - 2) {
                borders[1].push(line.chars().last()?);
                borders[3].push(line.chars().next()?);
            }
            let last_line = lines.next()?;
            borders[2] = last_line.chars().collect();
            borders[1].push(borders[2].chars().last()?);
            borders[3].push(borders[2].chars().next()?);
            let encoded: Vec<_> = borders
                .iter()
                .map(|border_chars| {
                    let reversed = &border_chars.chars().rev().collect();
                    let to_bin = |s: &String| {
                        u16::from_str_radix(&s.replace('#', "1").replace('.', "0"), 2)
                            .expect("Unexpected characters in tile")
                    };
                    to_bin(border_chars).min(to_bin(reversed))
                })
                .collect();
            let borders = [encoded[0], encoded[1], encoded[2], encoded[3]];
            Some((tile_number, borders))
        })
        .collect::<Option<_>>()
        .expect("Tile definition was not formatted correctly");

    let mut patterns: Vec<_> = tiles
        .iter()
        .map(|(_, patterns)| patterns)
        .flatten()
        .collect();
    patterns.sort();
    let unique_patterns: Vec<_> = patterns
        .into_iter()
        .map(|&pattern| (pattern, 1))
        .coalesce(|(x, x_count), (y, y_count)| {
            if x == y {
                Ok((x, x_count + 1))
            } else {
                Err(((x, x_count), (y, y_count)))
            }
        })
        .filter_map(|(pattern, count)| if count == 1 { Some(pattern) } else { None })
        .collect();

    let corner_product: u64 = tiles
        .iter()
        .filter_map(|(tile_num, patterns)| {
            if patterns
                .iter()
                .cycle()
                .take(5)
                .tuple_windows()
                .any(|(x, y)| unique_patterns.contains(x) && unique_patterns.contains(y))
            {
                Some(tile_num)
            } else {
                None
            }
        })
        // Short circuit the filtering when we've found all the corners
        .take(4)
        .product();
    println!(
        "The corner tiles multiplied together is {}!",
        corner_product
    );
}
