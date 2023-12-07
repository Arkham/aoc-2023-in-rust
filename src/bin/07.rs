advent_of_code::solution!(7);

use nom::{
    character::complete::{digit1, multispace0, one_of},
    combinator::map_res,
    multi::count,
    sequence::separated_pair,
    IResult,
};
use parse_display::{Display, FromStr};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Hash, Display, FromStr, PartialOrd, Ord, Clone)]
enum Card {
    #[display("2")]
    Two,
    #[display("3")]
    Three,
    #[display("4")]
    Four,
    #[display("5")]
    Five,
    #[display("6")]
    Six,
    #[display("7")]
    Seven,
    #[display("8")]
    Eight,
    #[display("9")]
    Nine,
    #[display("T")]
    T,
    #[display("J")]
    J,
    #[display("Q")]
    Q,
    #[display("K")]
    K,
    #[display("A")]
    A,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq)]
struct Hand {
    cards: Vec<Card>,
    hand_type: HandType,
}

#[derive(Debug, PartialEq)]
struct Bet {
    hand: Hand,
    amount: u32,
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    let (input, card) = one_of("AKQJT98765432")(input)?;
    let card = Card::from_str(&card.to_string()).unwrap();
    Ok((input, card))
}

fn parse_hand(input: &str) -> IResult<&str, Hand> {
    let (input, cards) = count(parse_card, 5)(input)?;
    let hand_type = determine_hand_type(&cards);
    Ok((input, Hand { cards, hand_type }))
}

fn determine_hand_type(cards: &[Card]) -> HandType {
    let mut counts = HashMap::new();
    for card in cards {
        *counts.entry(card).or_insert(0) += 1;
    }

    let mut pairs = 0;
    let mut threes = 0;
    let mut fours = 0;
    let mut fives = 0;

    for count in counts.values() {
        match count {
            2 => pairs += 1,
            3 => threes += 1,
            4 => fours += 1,
            5 => fives += 1,
            _ => (),
        }
    }

    match (fives, fours, threes, pairs) {
        (1, _, _, _) => HandType::FiveOfAKind,
        (_, 1, _, _) => HandType::FourOfAKind,
        (_, _, 1, 1) => HandType::FullHouse,
        (_, _, 1, _) => HandType::ThreeOfAKind,
        (_, _, _, 2) => HandType::TwoPair,
        (_, _, _, 1) => HandType::OnePair,
        _ => HandType::HighCard,
    }
}

fn parse_hand_with_jokers(input: &str) -> IResult<&str, Hand> {
    let (input, cards) = count(parse_card, 5)(input)?;
    let hand_type = determine_hand_type_with_jokers(&cards);
    Ok((input, Hand { cards, hand_type }))
}

fn determine_hand_type_with_jokers(cards: &[Card]) -> HandType {
    let mut counts = HashMap::new();
    for card in cards {
        *counts.entry(card).or_insert(0) += 1;
    }

    let jokers_count = counts.remove(&Card::J).unwrap_or(0);

    let mut pairs = 0;
    let mut threes = 0;
    let mut fours = 0;
    let mut fives = 0;

    let mut values: Vec<i32> = counts.values().cloned().collect();
    values.sort();

    if let Some(last) = values.last_mut() {
        *last += jokers_count;
    } else {
        fives = 1
    }

    for count in values {
        match count {
            0 => (),
            1 => (),
            2 => pairs += 1,
            3 => threes += 1,
            4 => fours += 1,
            _ => fives += 1,
        }
    }

    match (fives, fours, threes, pairs) {
        (1, _, _, _) => HandType::FiveOfAKind,
        (_, 1, _, _) => HandType::FourOfAKind,
        (_, _, 1, 1) => HandType::FullHouse,
        (_, _, 1, _) => HandType::ThreeOfAKind,
        (_, _, _, 2) => HandType::TwoPair,
        (_, _, _, 1) => HandType::OnePair,
        _ => HandType::HighCard,
    }
}

fn parse_bet(with_jokers: bool, input: &str) -> IResult<&str, Bet> {
    let (input, (hand, amount)) = separated_pair(
        if with_jokers {
            parse_hand_with_jokers
        } else {
            parse_hand
        },
        multispace0,
        map_res(digit1, |s: &str| s.parse()),
    )(input)?;
    Ok((input, Bet { hand, amount }))
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut bets: Vec<Bet> = input
        .trim()
        .lines()
        .map(|line| match parse_bet(false, line) {
            Ok((_, bet)) => bet,
            Err(err) => {
                panic!("Could not parse input: {}", err);
            }
        })
        .collect();

    bets.sort_by(|fst, snd| {
        let compare_hands = fst.hand.hand_type.cmp(&snd.hand.hand_type);
        if compare_hands == std::cmp::Ordering::Equal {
            match fst
                .hand
                .cards
                .iter()
                .zip(snd.hand.cards.clone())
                .map(|(x, y)| x.cmp(&y))
                .find(|el| *el != std::cmp::Ordering::Equal)
            {
                Some(ordering) => ordering,
                None => std::cmp::Ordering::Equal,
            }
        } else {
            compare_hands
        }
    });

    Some(
        bets.iter()
            .enumerate()
            .map(|(i, bet)| bet.amount * (i as u32 + 1))
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut bets: Vec<Bet> = input
        .trim()
        .lines()
        .map(|line| match parse_bet(true, line) {
            Ok((_, bet)) => bet,
            Err(err) => {
                panic!("Could not parse input: {}", err);
            }
        })
        .collect();

    bets.sort_by(|fst, snd| {
        let compare_hands = fst.hand.hand_type.cmp(&snd.hand.hand_type);
        if compare_hands == std::cmp::Ordering::Equal {
            match fst
                .hand
                .cards
                .iter()
                .zip(snd.hand.cards.clone())
                .map(|(x, y)| match (x, &y) {
                    (Card::J, Card::J) => std::cmp::Ordering::Equal,
                    (Card::J, _) => std::cmp::Ordering::Less,
                    (_, Card::J) => std::cmp::Ordering::Greater,
                    _ => x.cmp(&y),
                })
                .find(|el| *el != std::cmp::Ordering::Equal)
            {
                Some(ordering) => ordering,
                None => std::cmp::Ordering::Equal,
            }
        } else {
            compare_hands
        }
    });

    Some(
        bets.iter()
            .enumerate()
            .map(|(i, bet)| bet.amount * (i as u32 + 1))
            .sum::<u32>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        // let result = part_two("JJ23J 10");
        assert_eq!(result, Some(5905));
    }
}
