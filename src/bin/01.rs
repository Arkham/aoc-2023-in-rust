advent_of_code::solution!(1);

use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    Some(parse(input, false).iter().sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    Some((parse(input, true).iter()).sum())
}

fn parse(input: &str, look_at_strings: bool) -> Vec<u32> {
    input
        .trim()
        .split('\n')
        .map(|line| {
            let first_digit = find_first_digit(line, look_at_strings);
            let last_digit = find_last_digit(line, look_at_strings);
            match (first_digit, last_digit) {
                (Some(f), Some(l)) => format!("{}{}", f, l).parse::<u32>().unwrap(),
                (_, _) => panic!("Could not find digits in line '{}'", line),
            }
        })
        .collect()
}

fn find_first_digit(s: &str, look_at_strings: bool) -> Option<char> {
    let re = if look_at_strings {
        Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|\d)").unwrap()
    } else {
        Regex::new(r"\d").unwrap()
    };
    match re.find(s) {
        Some(matched) => matched
            .as_str()
            .replace("one", "1")
            .replace("two", "2")
            .replace("three", "3")
            .replace("four", "4")
            .replace("five", "5")
            .replace("six", "6")
            .replace("seven", "7")
            .replace("eight", "8")
            .replace("nine", "9")
            .chars()
            .next(),
        None => None,
    }
}

fn find_last_digit(s: &str, look_at_strings: bool) -> Option<char> {
    let re = if look_at_strings {
        Regex::new(r"(eno|owt|eerht|ruof|evif|xis|neves|thgie|enin|\d)").unwrap()
    } else {
        Regex::new(r"\d").unwrap()
    };
    let reversed = s.chars().rev().collect::<String>();
    match re.find(&reversed) {
        Some(matched) => matched
            .as_str()
            .replace("eno", "1")
            .replace("owt", "2")
            .replace("eerht", "3")
            .replace("ruof", "4")
            .replace("evif", "5")
            .replace("xis", "6")
            .replace("neves", "7")
            .replace("thgie", "8")
            .replace("enin", "9")
            .chars()
            .next(),
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(
            "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet",
        );
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
