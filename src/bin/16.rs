advent_of_code::solution!(16);

use colored::Colorize;
use parse_display::{Display, FromStr};
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Display, FromStr, PartialEq, Eq)]
enum Tile {
    #[display(".")]
    Empty,
    #[display("/")]
    MirrorSlash,
    #[display("\\")]
    MirrorBackslash,
    #[display("|")]
    SplitterVertical,
    #[display("-")]
    SplitterHorizontal,
}

fn parse_input(input: &str) -> Vec<Vec<Tile>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect()
        })
        .collect()
}

#[derive(Debug, Display, PartialEq, Eq, Clone, Copy, Hash)]
enum Dir {
    #[display("^")]
    Up = 0,
    #[display("v")]
    Down = 1,
    #[display("<")]
    Left = 2,
    #[display(">")]
    Right = 3,
}

type Pos = (i64, i64);

struct EnergyMap {
    rows: usize,
    cols: usize,
    visited: HashMap<Pos, HashSet<Dir>>,
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid = parse_input(input);
    // print_grid(&grid);
    let energy_map = energize_grid(&grid, (-1, 0), Dir::Right);
    // print_energy_map(&energy_map, &grid);
    Some(energy_map.visited.len())
}

fn unique_insert(energy_map: &mut EnergyMap, next: &mut Vec<(Pos, Dir)>, pos: Pos, dir: Dir) {
    if let Some(values) = energy_map.visited.get_mut(&pos) {
        if !values.contains(&dir) {
            values.insert(dir);
            next.push((pos, dir));
        }
    } else {
        energy_map
            .visited
            .insert(pos, vec![dir].into_iter().collect());
        next.push((pos, dir));
    }
}

fn energize_grid(grid: &[Vec<Tile>], start_pos: Pos, dir: Dir) -> EnergyMap {
    let mut energy_map = EnergyMap {
        visited: HashMap::new(),
        rows: grid.len(),
        cols: grid[0].len(),
    };
    let mut current = vec![(start_pos, dir)];

    while !current.is_empty() {
        let mut next = Vec::new();

        for ((x, y), dir) in &current {
            let (dx, dy) = dir_offset(dir);

            let new_pos = (x + dx, y + dy);
            if new_pos.0 >= 0
                && new_pos.0 < energy_map.cols as i64
                && new_pos.1 >= 0
                && new_pos.1 < energy_map.rows as i64
            {
                match grid[new_pos.1 as usize][new_pos.0 as usize] {
                    Tile::Empty => {
                        unique_insert(&mut energy_map, &mut next, new_pos, *dir);
                    }
                    Tile::MirrorSlash => {
                        let new_dir = match dir {
                            Dir::Up => Dir::Right,
                            Dir::Down => Dir::Left,
                            Dir::Left => Dir::Down,
                            Dir::Right => Dir::Up,
                        };
                        unique_insert(&mut energy_map, &mut next, new_pos, new_dir)
                    }
                    Tile::MirrorBackslash => {
                        let new_dir = match dir {
                            Dir::Up => Dir::Left,
                            Dir::Down => Dir::Right,
                            Dir::Left => Dir::Up,
                            Dir::Right => Dir::Down,
                        };
                        unique_insert(&mut energy_map, &mut next, new_pos, new_dir);
                    }
                    Tile::SplitterVertical => match dir {
                        Dir::Up | Dir::Down => {
                            unique_insert(&mut energy_map, &mut next, new_pos, *dir);
                        }
                        Dir::Left | Dir::Right => {
                            unique_insert(&mut energy_map, &mut next, new_pos, Dir::Up);
                            unique_insert(&mut energy_map, &mut next, new_pos, Dir::Down)
                        }
                    },
                    Tile::SplitterHorizontal => match dir {
                        Dir::Left | Dir::Right => {
                            unique_insert(&mut energy_map, &mut next, new_pos, *dir);
                        }
                        Dir::Up | Dir::Down => {
                            unique_insert(&mut energy_map, &mut next, new_pos, Dir::Left);
                            unique_insert(&mut energy_map, &mut next, new_pos, Dir::Right);
                        }
                    },
                }
            }
        }
        current = next;
    }

    energy_map
}

fn dir_offset(dir: &Dir) -> Pos {
    match dir {
        Dir::Up => (0, -1),
        Dir::Down => (0, 1),
        Dir::Left => (-1, 0),
        Dir::Right => (1, 0),
    }
}

#[allow(dead_code)]
fn print_grid(grid: &[Vec<Tile>]) {
    for row in grid {
        for tile in row {
            print!("{}", tile);
        }
        println!();
    }
    println!();
}

#[allow(dead_code)]
fn print_energy_map(energy_map: &EnergyMap, grid: &[Vec<Tile>]) {
    (0..energy_map.rows).for_each(|y| {
        for x in 0..energy_map.cols {
            if grid[y][x] == Tile::Empty {
                if let Some(dirs) = energy_map.visited.get(&(x as i64, y as i64)) {
                    if dirs.len() == 1 {
                        print!("{}", dirs.iter().next().unwrap());
                    } else {
                        print!("{}", dirs.len());
                    }
                } else {
                    print!(".");
                }
            } else {
                print!("{}", grid[y][x].to_string().red());
            }
        }
        println!();
    });
    println!();
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse_input(input);
    let rows = grid.len() as i64;
    let cols = grid[0].len() as i64;

    let from_left: Vec<(Pos, Dir)> = (0..rows).map(|y| ((-1, y), Dir::Right)).collect();
    let from_right: Vec<(Pos, Dir)> = (0..rows).map(|y| ((cols, y), Dir::Left)).collect();
    let from_above: Vec<(Pos, Dir)> = (0..cols).map(|x| ((x, -1), Dir::Down)).collect();
    let from_below: Vec<(Pos, Dir)> = (0..cols).map(|x| ((x, rows), Dir::Up)).collect();

    let all_starts: Vec<(Pos, Dir)> = [from_left, from_right, from_above, from_below]
        .iter()
        .flatten()
        .cloned()
        .collect();

    all_starts
        .par_iter()
        .map(|(start_pos, dir)| {
            let energy_map = energize_grid(&grid, *start_pos, *dir);
            energy_map.visited.len() as u32
        })
        .max()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
