advent_of_code::solution!(18);

use parse_display::{Display, FromStr};

#[derive(Debug, Display, FromStr, PartialEq, Eq, Clone, Copy)]
enum Dir {
    #[display("L")]
    Left,
    #[display("R")]
    Right,
    #[display("U")]
    Up,
    #[display("D")]
    Down,
}

#[derive(Debug, Display, FromStr)]
#[display("{dir} {count} ({hex_code})")]
struct Instruction {
    dir: Dir,
    count: i64,
    hex_code: String,
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .trim()
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let instructions = parse_input(input);
    Some(shoelace(
        instructions
            .iter()
            .map(|ins| (ins.dir, ins.count))
            .collect::<Vec<_>>(),
    ))
}

fn shoelace(instructions: Vec<(Dir, i64)>) -> u64 {
    let mut pos = (0, 0);
    let mut points = vec![(0, 0)];

    for (dir, count) in &instructions {
        let (dx, dy) = match dir {
            Dir::Right => (*count, 0),
            Dir::Down => (0, *count),
            Dir::Left => (-*count, 0),
            Dir::Up => (0, -*count),
        };
        pos = (pos.0 + dx, pos.1 + dy);
        points.push(pos);
    }

    let mut area: i64 = instructions.iter().map(|(_, count)| count).sum();
    for i in 1..points.len() {
        let (x1, y1) = points[i - 1];
        let (x2, y2) = points[i];
        area += (x1 * y2) - (x2 * y1);
    }

    (area / 2).unsigned_abs() + 1
}

pub fn part_two(input: &str) -> Option<u64> {
    let instructions = parse_input(input);
    Some(shoelace(
        instructions
            .iter()
            .map(|ins| decode_hex(&ins.hex_code))
            .collect::<Vec<_>>(),
    ))
}

fn decode_hex(hex: &str) -> (Dir, i64) {
    let hex = hex.trim_start_matches('#');
    let (distance, direction) = hex.split_at(5);

    let distance = i64::from_str_radix(distance, 16).unwrap();
    let direction = match i32::from_str_radix(direction, 16).unwrap() {
        0 => Dir::Right,
        1 => Dir::Down,
        2 => Dir::Left,
        3 => Dir::Up,
        _ => panic!("Invalid direction"),
    };

    (direction, distance)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(952408144115));
    }
}
