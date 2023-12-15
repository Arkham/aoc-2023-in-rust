advent_of_code::solution!(14);

use itertools::Itertools;
use parse_display::{Display, FromStr};
use std::collections::HashMap;

#[derive(Display, FromStr, Debug, Clone, PartialEq, Eq, Hash)]
enum Tile {
    #[display(".")]
    Empty,
    #[display("#")]
    Cube,
    #[display("O")]
    Round,
}

type Grid = Vec<Vec<Tile>>;

#[allow(dead_code)]
fn print_grid(grid: &Grid) {
    for row in grid {
        for tile in row {
            print!("{}", tile);
        }
        println!();
    }
    println!();
}

fn parse_input(input: &str) -> Grid {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect()
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid = parse_input(input);
    let rows = grid.len();
    let shifted = shift_north(&grid);

    let mut total = 0;
    for (i, row) in shifted.iter().enumerate() {
        for tile in row {
            if *tile == Tile::Round {
                total += rows - i;
            }
        }
    }
    Some(total)
}

fn rounds_first(a: &Tile, b: &Tile) -> std::cmp::Ordering {
    match (a, b) {
        (Tile::Round, Tile::Empty) => std::cmp::Ordering::Less,
        (Tile::Empty, Tile::Round) => std::cmp::Ordering::Greater,
        _ => std::cmp::Ordering::Equal,
    }
}

fn rounds_last(a: &Tile, b: &Tile) -> std::cmp::Ordering {
    match (a, b) {
        (Tile::Round, Tile::Empty) => std::cmp::Ordering::Greater,
        (Tile::Empty, Tile::Round) => std::cmp::Ordering::Less,
        _ => std::cmp::Ordering::Equal,
    }
}

fn shift_north(grid: &Grid) -> Grid {
    let transposed = transpose(grid);
    let shifted = shift_with_compare(&transposed, rounds_first);
    transpose(&shifted)
}

fn shift_south(grid: &Grid) -> Grid {
    let transposed = transpose(grid);
    let shifted = shift_with_compare(&transposed, rounds_last);
    transpose(&shifted)
}

fn shift_east(grid: &Grid) -> Grid {
    shift_with_compare(grid, rounds_last)
}

fn shift_west(grid: &Grid) -> Grid {
    shift_with_compare(grid, rounds_first)
}

fn shift_with_compare<F>(transposed: &[Vec<Tile>], compare: F) -> Vec<Vec<Tile>>
where
    F: Fn(&Tile, &Tile) -> std::cmp::Ordering + Copy,
{
    let shifted = transposed
        .iter()
        .map(|row| {
            Itertools::intersperse(
                row.split(|tile| *tile == Tile::Cube).map(|group| {
                    let mut group: Vec<_> = group.to_vec();
                    group.sort_by(compare);
                    group
                }),
                vec![Tile::Cube],
            )
            .flatten()
            .collect::<Vec<_>>()
        })
        .collect();
    shifted
}

fn transpose<T: Clone>(v: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let rows = v.len();
    let cols = v[0].len();

    let mut transposed = vec![vec![v[0][0].clone(); rows]; cols];

    for (r, row) in v.iter().enumerate() {
        for (c, _) in row.iter().enumerate() {
            transposed[c][r] = v[r][c].clone();
        }
    }

    transposed
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut grid = parse_input(input);
    let rows = grid.len();

    let mut visited = HashMap::new();
    let mut cache = vec![];
    let mut cycle_start = 0;
    let mut cycle_length = 0;
    let total_times = 1_000_000_000;

    for i in 0..total_times {
        grid = shift_north(&grid);
        grid = shift_west(&grid);
        grid = shift_south(&grid);
        grid = shift_east(&grid);

        if visited.contains_key(&grid) {
            cycle_start = visited[&grid];
            cycle_length = i - visited[&grid];
            break;
        } else {
            visited.insert(grid.clone(), i);
            cache.push(grid.clone());
        }
    }
    // dbg!(cycle_start, cycle_length);
    let desired = &cache[cycle_start + (total_times - 1 - cycle_start) % cycle_length];

    let mut total = 0;
    for (i, row) in desired.iter().enumerate() {
        for tile in row {
            if *tile == Tile::Round {
                total += rows - i;
            }
        }
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
