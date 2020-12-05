use std::fs::read_to_string;

fn main() {
    let mut seat_ids: Vec<_> = read_to_string("../input")
        .expect("Failed to open input file")
        .lines()
        .map(|pass| {
            let (row, column) = pass.split_at(7);
            let row_bin = row.replace('F', "0").replace('B', "1");
            let column_bin = column.replace('L', "0").replace('R', "1");
            u32::from_str_radix(&row_bin, 2).expect("Could not parse row") * 8
                + u32::from_str_radix(&column_bin, 2).expect("Could not parse column")
        })
        .collect();
    seat_ids.sort();
    let min_seat_id = seat_ids[0];
    let missing_seat_id = seat_ids
        .into_iter()
        .enumerate()
        .find(|(i, seat_id)| *seat_id != min_seat_id + *i as u32)
        .expect("No seats were missing")
        .1
        - 1;
    println!("My seat ID is {}!", missing_seat_id);
}
