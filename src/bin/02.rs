advent_of_code::solution!(2);

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1, multispace1},
    combinator::map_res,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Hash)]
enum Color {
    Red,
    Blue,
    Green,
}

impl FromStr for Color {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" => Ok(Color::Red),
            "blue" => Ok(Color::Blue),
            "green" => Ok(Color::Green),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<Vec<(Color, u32)>>,
}

fn parse_color(input: &str) -> IResult<&str, Color> {
    map_res(alpha1, FromStr::from_str)(input)
}

fn parse_count(input: &str) -> IResult<&str, u32> {
    map_res(digit1, FromStr::from_str)(input)
}

fn parse_color_count(input: &str) -> IResult<&str, (Color, u32)> {
    let (input, (count, color)) = separated_pair(parse_count, multispace1, parse_color)(input)?;
    Ok((input, (color, count)))
}

fn parse_round(input: &str) -> IResult<&str, Vec<(Color, u32)>> {
    separated_list1(tag(", "), parse_color_count)(input)
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, _) = tag("Game ")(input)?;
    let (input, id) = parse_count(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, rounds) = separated_list1(tag("; "), parse_round)(input)?;
    Ok((input, Game { id, rounds }))
}

pub fn part_one(input: &str) -> Option<u32> {
    let games = input
        .trim()
        .split('\n')
        .map(|line| parse_game(line).unwrap().1);

    let bag_contents: HashMap<_, _> = vec![(Color::Red, 12), (Color::Green, 13), (Color::Blue, 14)]
        .into_iter()
        .collect();

    let result = games.into_iter().filter(|game| {
        game.rounds.iter().all(|round| {
            round.iter().all(|(color, count)| {
                bag_contents
                    .get(color)
                    .map_or(false, |&bag_count| bag_count >= *count)
            })
        })
    });

    Some(result.map(|game| game.id).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let games = input
        .trim()
        .split('\n')
        .map(|line| parse_game(line).unwrap().1);

    Some(
        games
            .into_iter()
            .map(|game| {
                let mut bag_contents: HashMap<_, _> =
                    vec![(Color::Red, 0), (Color::Green, 0), (Color::Blue, 0)]
                        .into_iter()
                        .collect();

                for round in &game.rounds {
                    for (color, count) in round {
                        let bag_count = bag_contents.get_mut(color).unwrap();
                        *bag_count = (*bag_count).max(*count);
                    }
                }
                bag_contents.values().product::<u32>()
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
