use itertools::Itertools;
use std::{fs::read_to_string, iter};

type HyperCubeView = Vec<Vec<Vec<Vec<bool>>>>;

fn expand_hyper_cube_view(hyper_cube: HyperCubeView) -> HyperCubeView {
    let empty_row = vec![false; hyper_cube[0][0][0].len() + 2];
    let empty_slice = vec![empty_row.clone(); hyper_cube[0][0].len() + 2];
    let empty_cube = vec![empty_slice.clone(); hyper_cube[0].len() + 2];

    iter::once(empty_cube.clone())
        .chain(hyper_cube.into_iter().map(|cubes| {
            iter::once(empty_slice.clone())
                .chain(cubes.into_iter().map(|slice| {
                    iter::once(empty_row.clone())
                        .chain(slice.into_iter().map(|row| {
                            iter::once(false)
                                .chain(row)
                                .chain(iter::once(false))
                                .collect()
                        }))
                        .chain(iter::once(empty_row.clone()))
                        .collect()
                }))
                .chain(iter::once(empty_slice.clone()))
                .collect::<Vec<Vec<Vec<bool>>>>()
        }))
        .chain(iter::once(empty_cube))
        .collect()
}

fn main() {
    let input = read_to_string("../input")
        .expect("Failed to open input file")
        .lines()
        .map(|line| line.as_bytes().iter().map(|&cube| cube == b'#').collect())
        .collect();
    let mut hyper_cube: HyperCubeView = vec![vec![input]];
    let starting_height = hyper_cube[0][0].len();
    let starting_width = hyper_cube[0][0][0].len();
    hyper_cube = expand_hyper_cube_view(hyper_cube);

    for cycle in 1..=6 {
        let width = starting_width + cycle * 2;
        let height = starting_height + cycle * 2;
        let depth = cycle * 2 + 1;
        let cube_count = cycle * 2 + 1;

        let empty_row = vec![false; width + 2];
        let empty_slice = vec![empty_row.clone(); height + 2];
        let empty_cube = vec![empty_slice.clone(); depth + 2];
        let expanded_hyper_cube: Vec<Vec<Vec<_>>> = expand_hyper_cube_view(hyper_cube);
        let expanded_hyper_cube_ref = &expanded_hyper_cube;
        hyper_cube = expanded_hyper_cube
            .iter()
            .enumerate()
            .map(move |(w, cubes)| {
                if w == 0 || w == cube_count + 1 {
                    return empty_cube.clone();
                }
                cubes
                    .iter()
                    .enumerate()
                    .map(move |(x, slice)| {
                        if x == 0 || x == depth + 1 {
                            return vec![vec![false; width + 2]; height + 2];
                        }
                        slice
                            .iter()
                            .enumerate()
                            .map(move |(y, row)| {
                                if y == 0 || y == height + 1 {
                                    return vec![false; width + 2];
                                }
                                row.iter()
                                    .enumerate()
                                    .map(move |(z, is_active)| {
                                        if z == 0 || z == width + 1 {
                                            return false;
                                        }
                                        let active_neighbours = [
                                            [w - 1, w, w + 1],
                                            [x - 1, x, x + 1],
                                            [y - 1, y, y + 1],
                                            [z - 1, z, z + 1],
                                        ]
                                        .iter()
                                        .multi_cartesian_product()
                                        .filter(|product| {
                                            expanded_hyper_cube_ref[*product[0]][*product[1]]
                                                [*product[2]][*product[3]]
                                        })
                                        .count();
                                        if *is_active {
                                            // Includes the cube itself
                                            active_neighbours == 3 || active_neighbours == 4
                                        } else {
                                            active_neighbours == 3
                                        }
                                    })
                                    .collect()
                            })
                            .collect()
                    })
                    .collect()
            })
            .collect();
    }

    let active_cubes: usize = hyper_cube
        .into_iter()
        .map(|cube| {
            cube.into_iter()
                .map(|slice| {
                    slice
                        .into_iter()
                        .map(|row| row.into_iter().filter(|is_active| *is_active).count())
                        .sum::<usize>()
                })
                .sum::<usize>()
        })
        .sum();
    println!("Number of active cubes after 6 cycles is {}!", active_cubes);
}
