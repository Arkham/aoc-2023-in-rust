advent_of_code::solution!(5);

extern crate rayon;
use rayon::prelude::*;

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace1},
    sequence::{preceded, tuple},
    IResult,
};

type Mapping = Vec<(u32, u32, u32)>;

fn parse_number(input: &str) -> IResult<&str, u32> {
    let (input, number) = digit1(input)?;
    Ok((input, number.parse().unwrap()))
}

fn parse_mapping(input: &str) -> IResult<&str, Mapping> {
    let (input, _) = multispace1(input)?;
    let (input, map) = nom::multi::many1(tuple((
        parse_number,
        multispace1,
        parse_number,
        multispace1,
        parse_number,
        multispace1,
    )))(input)?;
    let map = map
        .into_iter()
        .map(|(n1, _, n2, _, n3, _)| (n1, n2, n3))
        .collect();
    Ok((input, map))
}

fn parse_input(input: &str) -> IResult<&str, (Vec<u32>, Vec<Mapping>)> {
    let (input, _) = tag("seeds: ")(input)?;
    let (input, seeds) = nom::multi::separated_list1(multispace1, parse_number)(input)?;

    let (input, _) = preceded(multispace1, tag("seed-to-soil map:"))(input)?;
    let (input, seed_to_soil) = parse_mapping(input)?;

    let (input, _) = tag("soil-to-fertilizer map:")(input)?;
    let (input, soil_to_fertilizer) = parse_mapping(input)?;

    let (input, _) = tag("fertilizer-to-water map:")(input)?;
    let (input, fertilizer_to_water) = parse_mapping(input)?;

    let (input, _) = tag("water-to-light map:")(input)?;
    let (input, water_to_light) = parse_mapping(input)?;

    let (input, _) = tag("light-to-temperature map:")(input)?;
    let (input, light_to_temperature) = parse_mapping(input)?;

    let (input, _) = tag("temperature-to-humidity map:")(input)?;
    let (input, temperature_to_humidity) = parse_mapping(input)?;

    let (input, _) = tag("humidity-to-location map:")(input)?;
    let (input, humidity_to_location) = parse_mapping(input)?;

    Ok((
        input,
        (
            seeds,
            vec![
                seed_to_soil,
                soil_to_fertilizer,
                fertilizer_to_water,
                water_to_light,
                light_to_temperature,
                temperature_to_humidity,
                humidity_to_location,
            ],
        ),
    ))
}

pub fn part_one(input: &str) -> Option<u32> {
    match parse_input(input) {
        Ok((_, (seeds, mappings))) => seeds
            .iter()
            .map(|n| {
                let mut curr = *n;

                for map in &mappings {
                    for (dest, src, range) in map {
                        if curr >= *src && curr <= *src + *range {
                            curr = *dest + (curr - *src);
                            break;
                        }
                    }
                }

                curr
            })
            .min(),
        Err(err) => panic!("Could not parse input: {}", err),
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    match parse_input(input) {
        Ok((_, (seed_ranges, mappings))) => {
            let mut seeds = Vec::new();
            for n in seed_ranges[0]..(seed_ranges[0] + seed_ranges[1]) {
                seeds.push(n);
            }
            for n in seed_ranges[2]..(seed_ranges[2] + seed_ranges[3]) {
                seeds.push(n);
            }

            seeds
                .par_iter()
                .map(|n| {
                    let mut curr = *n;

                    for map in &mappings {
                        for (dest, src, range) in map {
                            if curr >= *src && curr <= *src + *range {
                                curr = *dest + (curr - *src);
                                break;
                            }
                        }
                    }

                    curr
                })
                .min()
        }
        Err(err) => panic!("Could not parse input: {}", err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
