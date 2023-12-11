advent_of_code::solution!(11);

use parse_display::{Display, FromStr};
use std::collections::HashSet;

#[derive(Display, FromStr, PartialEq, Debug, Clone)]
enum Tile {
    #[display(".")]
    Empty,
    #[display("#")]
    Galaxy,
}

type Pos = (usize, usize);

struct Universe {
    galaxies: HashSet<Pos>,
    rows: usize,
    cols: usize,
}

pub fn part_one(input: &str) -> Option<u64> {
    let debug = false;
    let original = parse_input(input);
    if debug {
        print_universe(&original)
    }
    let expanded = expand_universe(&original, 2);
    if debug {
        print_universe(&expanded);
    }

    Some(find_distances_sum(&expanded))
}

pub fn part_two(input: &str) -> Option<u64> {
    let original = parse_input(input);
    let expanded = expand_universe(&original, 1000000);
    Some(find_distances_sum(&expanded))
}

fn parse_input(input: &str) -> Universe {
    let map: Vec<Vec<Tile>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect()
        })
        .collect();
    let mut galaxies = HashSet::new();
    for (y, row) in map.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if tile == &Tile::Galaxy {
                galaxies.insert((x, y));
            }
        }
    }
    Universe {
        galaxies,
        rows: map.len(),
        cols: map[0].len(),
    }
}

fn expand_universe(uni: &Universe, factor: usize) -> Universe {
    let to_add: usize = factor - 1;
    let galaxies_coords = uni.galaxies.iter().cloned().collect::<Vec<_>>();
    let (all_xs, all_ys): (HashSet<usize>, HashSet<usize>) = galaxies_coords.into_iter().unzip();

    let mut empty_row_indexes = vec![];
    for i in 0..uni.rows {
        if !all_ys.contains(&i) {
            empty_row_indexes.push(i);
        }
    }
    let mut empty_col_indexes = vec![];
    for i in 0..uni.cols {
        if !all_xs.contains(&i) {
            empty_col_indexes.push(i);
        }
    }

    let mut new_galaxies = HashSet::new();
    for galaxy in &uni.galaxies {
        let (x, y) = galaxy;
        let empty_cols_before = empty_col_indexes.iter().filter(|i| **i < *x).count();
        let empty_rows_before = empty_row_indexes.iter().filter(|i| **i < *y).count();
        let new_x = x + empty_cols_before * to_add;
        let new_y = y + empty_rows_before * to_add;
        new_galaxies.insert((new_x, new_y));
    }

    Universe {
        galaxies: new_galaxies,
        rows: uni.rows + empty_row_indexes.len() * to_add,
        cols: uni.cols + empty_col_indexes.len() * to_add,
    }
}

#[allow(dead_code)]
fn print_universe(uni: &Universe) {
    for row in 0..uni.rows {
        for col in 0..uni.cols {
            if uni.galaxies.contains(&(col, row)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn find_distances_sum(uni: &Universe) -> u64 {
    let galaxies_coords = uni.galaxies.iter().cloned().collect::<Vec<_>>();
    let mut total = 0;
    for i in 0..galaxies_coords.len() {
        for j in i + 1..galaxies_coords.len() {
            let (x1, y1) = galaxies_coords[i];
            let (x2, y2) = galaxies_coords[j];
            let distance = (x1 as i64 - x2 as i64).abs() + (y1 as i64 - y2 as i64).abs();
            total += distance as u64;
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(82000210));
    }
}
