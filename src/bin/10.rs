advent_of_code::solution!(10);

use colored::Colorize;
use nom::{
    branch::alt,
    character::complete::{char, multispace1},
    combinator::map,
    multi::{many1, separated_list1},
    IResult,
};
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Clone)]
enum Tile {
    Vertical,
    Horizontal,
    BendNE,
    BendNW,
    BendSW,
    BendSE,
    Ground,
    Start,
}

fn parse_tile(input: &str) -> IResult<&str, Tile> {
    alt((
        map(char('|'), |_| Tile::Vertical),
        map(char('-'), |_| Tile::Horizontal),
        map(char('L'), |_| Tile::BendNE),
        map(char('J'), |_| Tile::BendNW),
        map(char('7'), |_| Tile::BendSW),
        map(char('F'), |_| Tile::BendSE),
        map(char('.'), |_| Tile::Ground),
        map(char('S'), |_| Tile::Start),
    ))(input)
}

fn parse_grid(input: &str) -> IResult<&str, Vec<Vec<Tile>>> {
    separated_list1(multispace1, many1(parse_tile))(input)
}

type Pos = (i64, i64);

fn grid_to_map(grid: Vec<Vec<Tile>>) -> (HashMap<Pos, Tile>, Pos) {
    let mut map = HashMap::new();
    let mut start_pos = (0, 0);

    for (y, row) in grid.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if tile == &Tile::Start {
                start_pos = (x as i64, y as i64);
            }
            map.insert((x as i64, y as i64), tile.clone());
        }
    }
    (map, start_pos)
}

pub fn part_one(input: &str) -> Option<u32> {
    match parse_grid(input) {
        Ok((_, grid)) => {
            let rows = grid.len();
            let cols = grid[0].len();
            let (mut map, start_pos) = grid_to_map(grid);
            let visited = explore_and_mark(&mut map, start_pos, rows, cols);
            Some(*visited.values().max().unwrap() as u32)
        }
        Err(e) => panic!("{}", e),
    }
}

fn explore_and_mark(
    map: &mut HashMap<Pos, Tile>,
    start_pos: Pos,
    rows: usize,
    cols: usize,
) -> HashMap<Pos, i32> {
    let debug = false;

    let mut visited = HashMap::new();
    let mut tracked = vec![start_pos];
    visited.insert(start_pos, 0);

    loop {
        let mut new_tracked = vec![];
        for curr in &tracked {
            let curr_tile = map.get(curr).unwrap();
            let curr_count = *visited.get(curr).unwrap();

            let above = (curr.0, curr.1 - 1);
            if let (
                None,
                Tile::Start | Tile::Vertical | Tile::BendNE | Tile::BendNW,
                Some(Tile::Vertical) | Some(Tile::BendSE) | Some(Tile::BendSW),
            ) = (visited.get(&above), curr_tile, map.get(&above))
            {
                new_tracked.push(above);
                visited.insert(above, curr_count + 1);
            }

            let below = (curr.0, curr.1 + 1);
            if let (
                None,
                Tile::Start | Tile::Vertical | Tile::BendSE | Tile::BendSW,
                Some(Tile::Vertical) | Some(Tile::BendNE) | Some(Tile::BendNW),
            ) = (visited.get(&below), curr_tile, map.get(&below))
            {
                new_tracked.push(below);
                visited.insert(below, curr_count + 1);
            }

            let left = (curr.0 - 1, curr.1);
            if let (
                None,
                Tile::Start | Tile::Horizontal | Tile::BendSW | Tile::BendNW,
                Some(Tile::Horizontal) | Some(Tile::BendSE) | Some(Tile::BendNE),
            ) = (visited.get(&left), curr_tile, map.get(&left))
            {
                new_tracked.push(left);
                visited.insert(left, curr_count + 1);
            }

            let right = (curr.0 + 1, curr.1);
            if let (
                None,
                Tile::Start | Tile::Horizontal | Tile::BendSE | Tile::BendNE,
                Some(Tile::Horizontal) | Some(Tile::BendSW) | Some(Tile::BendNW),
            ) = (visited.get(&right), curr_tile, map.get(&right))
            {
                new_tracked.push(right);
                visited.insert(right, curr_count + 1);
            }

            if curr_tile == &Tile::Start {
                if new_tracked == [above, below] {
                    map.insert(start_pos, Tile::Vertical)
                } else if new_tracked == [above, left] {
                    map.insert(start_pos, Tile::BendNE)
                } else if new_tracked == [above, right] {
                    map.insert(start_pos, Tile::BendNW)
                } else if new_tracked == [below, left] {
                    map.insert(start_pos, Tile::BendSE)
                } else if new_tracked == [below, right] {
                    map.insert(start_pos, Tile::BendSW)
                } else if new_tracked == [left, right] {
                    map.insert(start_pos, Tile::Horizontal)
                } else {
                    panic!("Should not happen: {:?}", new_tracked)
                };
            }
        }

        if new_tracked.is_empty() {
            break;
        }
        tracked = new_tracked;
    }

    if debug {
        for y in 0..rows {
            for x in 0..cols {
                print!(
                    "{}",
                    match visited.get(&(x as i64, y as i64)) {
                        Some(steps) => format!("{}", steps),
                        None => ".".to_string(),
                    }
                );
            }
            println!();
        }
    }

    visited
}

fn tile_to_string(tile: &Tile) -> String {
    match tile {
        Tile::Vertical => "|".to_string(),
        Tile::Horizontal => "-".to_string(),
        Tile::BendNE => "L".to_string(),
        Tile::BendNW => "J".to_string(),
        Tile::BendSW => "7".to_string(),
        Tile::BendSE => "F".to_string(),
        Tile::Ground => ".".to_string(),
        Tile::Start => "S".to_string(),
    }
}

#[allow(dead_code)]
fn print_map(map: &HashMap<Pos, Tile>, rows: usize, cols: usize) {
    for y in 0..rows {
        for x in 0..cols {
            print!(
                "{}",
                match map.get(&(x as i64, y as i64)) {
                    Some(tile) => tile_to_string(tile),
                    None => " ".to_string(),
                }
            );
        }
        println!();
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    match parse_grid(input) {
        Ok((_, grid)) => {
            let rows = grid.len();
            let cols = grid[0].len();
            let (mut map, start_pos) = grid_to_map(grid);
            let visited = explore_and_mark(&mut map, start_pos, rows, cols);
            let mut inner_section: HashSet<Pos> = HashSet::new();
            let mut outer_section: HashSet<Pos> = HashSet::new();

            for y in 0..rows {
                let mut count_north_crossings = 0;
                for x in 0..cols {
                    let pos = (x as i64, y as i64);
                    if let (Some(_), Some(Tile::Vertical | Tile::BendNE | Tile::BendNW)) =
                        (visited.get(&pos), map.get(&pos))
                    {
                        count_north_crossings += 1;
                    }

                    if visited.get(&pos).is_none() {
                        if count_north_crossings % 2 == 1 {
                            inner_section.insert(pos);
                        } else {
                            outer_section.insert(pos);
                        }
                    }
                }
            }

            let debug = false;
            if debug {
                for y in 0..rows {
                    for x in 0..cols {
                        let pos = (x as i64, y as i64);
                        let value = match (
                            visited.get(&pos),
                            outer_section.contains(&pos),
                            inner_section.contains(&pos),
                        ) {
                            (Some(_), _, _) => tile_to_string(map.get(&pos).unwrap()),
                            (_, true, _) => "O".red().to_string(),
                            (_, _, true) => "I".green().to_string(),
                            _ => panic!("Should not happen: {:?}", pos),
                        };
                        print!("{}", value);
                    }
                    println!();
                }
            }

            Some(inner_section.len() as u32)
        }
        Err(e) => panic!("{}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(
            "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L",
        );
        assert_eq!(result, Some(10));
    }

    #[test]
    fn test_inner_loop() {
        let result = part_two(
            "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........",
        );
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_unused_loop() {
        let result = part_two(
            "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........",
        );
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_large() {
        let result = part_two(
            ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...",
        );

        assert_eq!(result, Some(8));
    }
}
