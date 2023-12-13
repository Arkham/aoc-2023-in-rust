advent_of_code::solution!(12);

use cached::proc_macro::cached;

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .map(|line| {
                let (spring, counts) = line.split_once(' ').unwrap();
                let counts = counts
                    .split(',')
                    .map(|number| number.parse::<usize>().unwrap())
                    .collect::<Vec<_>>();
                solve(&spring.chars().collect::<Vec<_>>(), &counts, None)
            })
            .sum::<usize>(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .map(|line| {
                let (s, n) = line.split_once(' ').unwrap();
                let new_line = format!("{s}?{s}?{s}?{s}?{s} {n},{n},{n},{n},{n}");
                let (spring, counts) = new_line.split_once(' ').unwrap();
                let counts = counts
                    .split(',')
                    .map(|number| number.parse::<usize>().unwrap())
                    .collect::<Vec<_>>();
                solve(&spring.chars().collect::<Vec<_>>(), &counts, None)
            })
            .sum::<usize>(),
    )
}

#[cached(
    key = "String",
    convert = r#"{format!("{:?}{:?}{:?}", spring, groups, in_group)}"#
)]
fn solve(spring: &[char], groups: &[usize], in_group: Option<usize>) -> usize {
    if spring.is_empty() {
        return match in_group {
            Some(n) if groups == [n] => 1,
            None if groups.is_empty() => 1,
            _ => 0,
        };
    }
    match (in_group, spring[0], groups) {
        // we are in a group that's already satisfied, move to next group
        (Some(n), '.' | '?', [e, ..]) if n == *e => solve(&spring[1..], &groups[1..], None),
        // we are in a group that's not already satisfied, keep counting
        (Some(n), '#' | '?', [e, ..]) if n < *e => solve(&spring[1..], groups, Some(n + 1)),
        // we are not in a group (or there are no more groups), keep going
        (None, '.', _) | (None, '?', []) => solve(&spring[1..], groups, None),
        // we are not in a group, start a new group
        (None, '#', [_, ..]) => solve(&spring[1..], groups, Some(1)),
        // we are not in a group, we can either start a new group or not
        (None, '?', _) => solve(&spring[1..], groups, None) + solve(&spring[1..], groups, Some(1)),
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        // let result = part_one("?.? 1");
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
