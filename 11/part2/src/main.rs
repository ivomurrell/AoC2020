use std::fs::read_to_string;

fn scan_for_seats<F>(layout: &[Vec<u8>], scanner: F) -> bool
where
    F: Fn(usize) -> Option<(usize, usize)>,
{
    for step in 1.. {
        match scanner(step).and_then(|(row, column)| Some(layout.get(row)?.get(column)?)) {
            Some(&seat) => {
                if seat == b'.' {
                    continue;
                } else {
                    return seat == b'#';
                }
            }
            None => break,
        }
    }
    false
}

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
                            let scanners: [&dyn Fn(usize) -> Option<(usize, usize)>; 8] = [
                                &|step: usize| i.checked_sub(step).zip(j.checked_sub(step)),
                                &|step: usize| i.checked_sub(step).map(|i| (i, j)),
                                &|step: usize| i.checked_sub(step).map(|i| (i, j + step)),
                                &|step: usize| j.checked_sub(step).map(|j| (i, j)),
                                &|step: usize| Some((i, j + step)),
                                &|step: usize| j.checked_sub(step).map(|j| (i + step, j)),
                                &|step: usize| Some((i + step, j)),
                                &|step: usize| Some((i + step, j + step)),
                            ];
                            let adjacent_occupied = scanners
                                .iter()
                                .filter(|&scanner| scan_for_seats(&seat_layout, scanner))
                                .count();

                            if seat == b'L' && adjacent_occupied == 0 {
                                b'#'
                            } else if seat == b'#' && adjacent_occupied >= 5 {
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
