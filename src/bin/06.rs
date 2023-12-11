advent_of_code::solution!(6);

use nom::{
    bytes::complete::{tag, take_while_m_n},
    character::complete::{digit1, multispace1},
    combinator::map_res,
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

fn from_input(input: &str) -> IResult<&str, Vec<Race>> {
    let (input, (_, times, _, distances)) = tuple((
        tuple((tag("Time:"), multispace1)),
        separated_list1(
            multispace1,
            take_while_m_n(1, 5, |c: char| c.is_ascii_digit()),
        ),
        tuple((multispace1, tag("Distance:"), multispace1)),
        separated_list1(
            multispace1,
            take_while_m_n(1, 5, |c: char| c.is_ascii_digit()),
        ),
    ))(input)?;

    let times = times.into_iter().map(|time| time.parse().unwrap());
    let distances = distances
        .into_iter()
        .map(|distance| distance.parse().unwrap());

    let races: Vec<Race> = times
        .into_iter()
        .zip(distances)
        .map(|(time, distance)| Race { time, distance })
        .collect();

    Ok((input, races))
}

pub fn part_one(input: &str) -> Option<u32> {
    match from_input(input) {
        Ok((_, races)) => Some(races.iter().map(find_winning_positions).product::<u32>()),
        Err(err) => {
            panic!("Could not parse input: {}", err);
        }
    }
}

fn parse_race(input: &str) -> IResult<&str, Race> {
    let (input, (time, _, distance)) = tuple((
        preceded(tag("Time:"), map_res(digit1, |s: &str| s.parse())),
        tag("\n"),
        preceded(tag("Distance:"), map_res(digit1, |s: &str| s.parse())),
    ))(input)?;

    Ok((input, Race { time, distance }))
}

pub fn part_two(input: &str) -> Option<u32> {
    match parse_race(&input.replace(' ', "")) {
        Ok((_, race)) => Some(find_winning_positions(&race)),
        Err(err) => panic!("Could not parse input: {}", err),
    }
}

fn find_winning_positions(race: &Race) -> u32 {
    let d = race.distance as f64;
    let t = race.time as f64;
    // we are solving the 'x * (t - x) > d' inequality
    // xt - x^2 > d
    // x^2 - xt + d < 0
    // x = (t +- sqrt(t^2 - 4d)) / 2
    let lower_x = (t - (t.powi(2) - (4.0 * d)).sqrt()) / 2.0;
    let upper_x = (t + (t.powi(2) - (4.0 * d)).sqrt()) / 2.0;
    count_integers_between_floats(lower_x, upper_x)
}

fn count_integers_between_floats(a: f64, b: f64) -> u32 {
    let lower = a.floor() as i32 + 1;
    let upper = b.ceil() as i32 - 1;
    (upper - lower + 1).unsigned_abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
