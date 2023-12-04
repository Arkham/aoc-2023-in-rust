advent_of_code::solution!(4);

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace1},
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug)]
struct Card {
    #[allow(dead_code)]
    id: i32,
    winning_numbers: HashSet<i32>,
    given_numbers: HashSet<i32>,
}

fn parse_number(input: &str) -> IResult<&str, i32> {
    let (input, number) = digit1(input)?;
    let number = i32::from_str(number).unwrap();
    Ok((input, number))
}

fn parse_numbers(input: &str) -> IResult<&str, HashSet<i32>> {
    let (input, numbers) = separated_list1(multispace1, parse_number)(input)?;
    Ok((input, numbers.into_iter().collect()))
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    let (input, (id, winning_numbers, given_numbers)) = tuple((
        preceded(tag("Card"), preceded(multispace1, parse_number)),
        preceded(tag(":"), preceded(multispace1, parse_numbers)),
        preceded(tag(" |"), preceded(multispace1, parse_numbers)),
    ))(input)?;

    Ok((
        input,
        Card {
            id,
            winning_numbers,
            given_numbers,
        },
    ))
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .trim()
            .lines()
            .map(|line| match parse_card(line) {
                Ok((_, card)) => {
                    let mut result = 0;
                    for given_number in card.given_numbers {
                        if card.winning_numbers.contains(&given_number) {
                            if result == 0 {
                                result = 1;
                            } else {
                                result *= 2;
                            }
                        }
                    }
                    result
                }
                Err(err) => panic!("Could not parse card {}, {}", line, err),
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let cards = input
        .trim()
        .lines()
        .filter_map(|line| match parse_card(line) {
            Ok((_, card)) => Some(card),
            Err(_) => None,
        });

    let mut counts: Vec<u32> = cards.clone().map(|_| 1).collect();

    for (i, card) in cards.enumerate() {
        let mut won_this_round = card
            .winning_numbers
            .intersection(&card.given_numbers)
            .count();
        while won_this_round > 0 {
            counts[i + won_this_round] += counts[i];
            won_this_round -= 1;
        }
    }

    Some(counts.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
