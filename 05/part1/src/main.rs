use std::fs::read_to_string;

fn main() {
    let max_seat_id = read_to_string("../input")
        .expect("Failed to open input file")
        .lines()
        .map(|pass| {
            let (row, column) = pass.split_at(7);
            let row_bin = row.replace('F', "0").replace('B', "1");
            let column_bin = column.replace('L', "0").replace('R', "1");
            u32::from_str_radix(&row_bin, 2).expect("Could not parse row") * 8
                + u32::from_str_radix(&column_bin, 2).expect("Could not parse column")
        })
        .max()
        .expect("File was empty");
    println!("Max seat ID is {}!", max_seat_id);
}
