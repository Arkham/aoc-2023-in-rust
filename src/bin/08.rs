advent_of_code::solution!(8);

use rayon::prelude::*;
use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while_m_n},
    character::complete::multispace1,
    combinator::map,
    multi::{many1, separated_list1},
    sequence::tuple,
    IResult,
};

#[derive(Debug, Clone)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Rule {
    name: String,
    values: (String, String),
}

fn is_alphanumeric(c: char) -> bool {
    c.is_alphanumeric()
}

fn parse_direction(input: &str) -> IResult<&str, Direction> {
    alt((
        map(tag("L"), |_| Direction::Left),
        map(tag("R"), |_| Direction::Right),
    ))(input)
}

fn parse_rule(input: &str) -> IResult<&str, Rule> {
    let (input, (name, _, _, (value1, _, value2), _)) = tuple((
        take_while_m_n(3, 3, is_alphanumeric),
        tag(" = "),
        tag("("),
        tuple((
            take_while_m_n(3, 3, is_alphanumeric),
            tag(", "),
            take_while_m_n(3, 3, is_alphanumeric),
        )),
        tag(")"),
    ))(input)?;

    Ok((
        input,
        Rule {
            name: name.to_string(),
            values: (value1.to_string(), value2.to_string()),
        },
    ))
}

fn parse_input(input: &str) -> IResult<&str, (Vec<Direction>, Vec<Rule>)> {
    let (input, (directions, _, rules)) = tuple((
        many1(parse_direction),
        multispace1,
        separated_list1(multispace1, parse_rule),
    ))(input)?;
    Ok((input, (directions, rules)))
}

pub fn part_one(input: &str) -> Option<u32> {
    match parse_input(input) {
        Ok((_, (directions, rules))) => {
            let rules_map: HashMap<String, (String, String)> = rules
                .into_iter()
                .map(|rule| (rule.name, rule.values))
                .collect();

            let directions = directions.into_iter().cycle();

            let mut curr = "AAA";
            let mut steps = 0;

            for dir in directions {
                match dir {
                    Direction::Left => {
                        curr = &rules_map.get(curr).unwrap().0;
                    }
                    Direction::Right => {
                        curr = &rules_map.get(curr).unwrap().1;
                    }
                }
                steps += 1;
                if curr == "ZZZ" {
                    break;
                }
            }

            Some(steps)
        }
        Err(err) => {
            panic!("Could not parse input: {}", err);
        }
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    match parse_input(input) {
        Ok((_, (directions, rules))) => {
            let rules_map: HashMap<String, (String, String)> = rules
                .into_iter()
                .map(|rule| (rule.name, rule.values))
                .collect();

            let tracked: Vec<String> = rules_map
                .keys()
                .filter(|k| k.ends_with('A'))
                .cloned()
                .collect();

            let tracked_loops: Vec<usize> = tracked
                .par_iter()
                .map(|start| {
                    let mut directions = directions.clone().into_iter().cycle();
                    let mut curr = start;
                    let mut steps = 0;

                    loop {
                        let dir = directions.next().unwrap();
                        match dir {
                            Direction::Left => {
                                curr = &rules_map.get(curr).unwrap().0;
                            }
                            Direction::Right => {
                                curr = &rules_map.get(curr).unwrap().1;
                            }
                        }

                        steps += 1;

                        if curr.ends_with('Z') {
                            break steps;
                        }
                    }
                })
                .collect();

            Some(lcm_vec(tracked_loops))
        }
        Err(err) => {
            panic!("Could not parse input: {}", err);
        }
    }
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

fn lcm_vec(numbers: Vec<usize>) -> usize {
    numbers.into_iter().fold(1, lcm)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(
            "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)",
        );
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(
            "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)",
        );
        assert_eq!(result, Some(6));
    }
}
