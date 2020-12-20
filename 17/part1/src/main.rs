use itertools::Itertools;
use std::{fs::read_to_string, iter};

type CubeView = Vec<Vec<Vec<bool>>>;

fn expand_cube_view(cubes: CubeView) -> CubeView {
    let empty_row = vec![false; cubes[0][0].len() + 2];
    let empty_slice = vec![empty_row.clone(); cubes[0].len() + 2];

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
        .chain(iter::once(empty_slice))
        .collect()
}

fn main() {
    let input = read_to_string("../input")
        .expect("Failed to open input file")
        .lines()
        .map(|line| line.as_bytes().iter().map(|&cube| cube == b'#').collect())
        .collect();
    let mut cubes: CubeView = vec![input];
    let starting_height = cubes[0].len();
    let starting_width = cubes[0][0].len();
    cubes = expand_cube_view(cubes);
    for cycle in 1..=6 {
        let width = starting_width + cycle * 2;
        let height = starting_height + cycle * 2;
        let depth = cycle * 2 + 1;

        let empty_row = vec![false; width + 2];
        let empty_slice = vec![empty_row.clone(); height + 2];
        let expanded_cubes: Vec<Vec<Vec<_>>> = expand_cube_view(cubes);
        let expanded_cubes_ref = &expanded_cubes;
        let next_cubes = expanded_cubes
            .iter()
            .enumerate()
            .map(move |(x, slice)| {
                if x == 0 || x == depth + 1 {
                    return empty_slice.clone();
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
                                let active_neighbours =
                                    [[x - 1, x, x + 1], [y - 1, y, y + 1], [z - 1, z, z + 1]]
                                        .iter()
                                        .multi_cartesian_product()
                                        .filter(|product| {
                                            expanded_cubes_ref[*product[0]][*product[1]]
                                                [*product[2]]
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
            .collect();

        cubes = next_cubes;
    }

    let active_cubes: usize = cubes
        .into_iter()
        .map(|slice| {
            slice
                .into_iter()
                .map(|row| row.into_iter().filter(|is_active| *is_active).count())
                .sum::<usize>()
        })
        .sum();
    println!("Number of active cubes after 6 cycles is {}!", active_cubes);
}
