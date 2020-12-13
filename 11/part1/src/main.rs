use std::fs::read_to_string;

fn main() {
    let mut seat_layout: Vec<_> = read_to_string("../input")
        .expect("Failed to open input file")
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect();

    loop {
        let next_round = seat_layout
            .iter()
            .enumerate()
            .map(|(i, line)| {
                line.iter()
                    .enumerate()
                    .map(|(j, &seat)| {
                        if seat == b'L' || seat == b'#' {
                            let adjacent_occupied = [
                                (i.checked_sub(1).zip(j.checked_sub(1))),
                                i.checked_sub(1).map(|i| (i, j)),
                                i.checked_sub(1).map(|i| (i, j + 1)),
                                j.checked_sub(1).map(|j| (i, j)),
                                Some((i, j + 1)),
                                j.checked_sub(1).map(|j| (i + 1, j)),
                                Some((i + 1, j)),
                                Some((i + 1, j + 1)),
                            ]
                            .iter()
                            .map(|&indices| {
                                let (i, j) = indices?;
                                Some(seat_layout.get(i)?.get(j)? == &b'#')
                            })
                            .filter(|occupied| occupied.unwrap_or_default())
                            .count();

                            if seat == b'L' && adjacent_occupied == 0 {
                                b'#'
                            } else if seat == b'#' && adjacent_occupied >= 4 {
                                b'L'
                            } else {
                                seat
                            }
                        } else {
                            seat
                        }
                    })
                    .collect()
            })
            .collect();

        if seat_layout == next_round {
            let occupied_seats: usize = seat_layout
                .into_iter()
                .map(|row| row.into_iter().filter(|&seat| seat == b'#').count())
                .sum();
            println!(
                "After stabilising there are {} occupied seats!",
                occupied_seats
            );
            return;
        }

        seat_layout = next_round;
    }
}
