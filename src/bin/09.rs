advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i64> {
    Some(
        parse_input(input)
            .iter()
            .map(|numbers| predict_next(numbers))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<i64> {
    Some(
        parse_input(input)
            .iter()
            .map(|numbers| predict_previous(numbers))
            .sum(),
    )
}

fn parse_input(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse().expect("parse error"))
                .collect()
        })
        .collect()
}

fn predict_next(numbers: &[i64]) -> i64 {
    let last_number = numbers.last().unwrap();
    let differences: Vec<i64> = numbers.windows(2).map(|w| w[1] - w[0]).collect();

    if differences.windows(2).all(|w| w[0] == w[1]) {
        last_number + differences[0]
    } else {
        last_number + predict_next(&differences)
    }
}

fn predict_previous(numbers: &[i64]) -> i64 {
    let first_number = numbers[0];
    let differences: Vec<i64> = numbers.windows(2).map(|w| w[1] - w[0]).collect();

    if differences.windows(2).all(|w| w[0] == w[1]) {
        first_number - differences[0]
    } else {
        first_number - predict_previous(&differences)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
