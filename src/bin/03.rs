advent_of_code::solution!(3);

pub fn process_map<F>(input: &str, condition: F) -> Vec<Vec<u32>>
where
    F: Fn(char) -> bool,
{
    let map = parse_map(input);

    let mut visited: Vec<Vec<bool>> = map
        .iter()
        .map(|inner_vec| inner_vec.iter().map(|_| false).collect())
        .collect();

    let directions = vec![
        (-1, 0),  // up
        (1, 0),   // down
        (0, -1),  // left
        (0, 1),   // right
        (-1, -1), // up-left
        (-1, 1),  // up-right
        (1, -1),  // down-left
        (1, 1),   // down-right
    ];

    let mut results = Vec::new();

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if condition(map[i][j]) {
                let mut numbers = Vec::new();
                for dir in &directions {
                    let ni = (i as i32) + dir.0;
                    let nj = (j as i32) + dir.1;

                    if ni >= 0
                        && ni < map.len() as i32
                        && nj >= 0
                        && nj < map[ni as usize].len() as i32
                        && map[ni as usize][nj as usize].is_ascii_digit()
                        && !visited[ni as usize][nj as usize]
                    {
                        let mut before = String::new();
                        let mut before_nj = nj;
                        while before_nj >= 0
                            && map[ni as usize][before_nj as usize].is_ascii_digit()
                        {
                            before.push(map[ni as usize][before_nj as usize]);
                            visited[ni as usize][before_nj as usize] = true;
                            before_nj -= 1;
                        }

                        let mut result = before.chars().rev().collect::<String>();
                        let mut after_nj = nj + 1;
                        while after_nj < map[ni as usize].len() as i32
                            && map[ni as usize][after_nj as usize].is_ascii_digit()
                        {
                            result.push(map[ni as usize][after_nj as usize]);
                            visited[ni as usize][after_nj as usize] = true;
                            after_nj += 1;
                        }

                        if !result.is_empty() {
                            numbers.push(result.parse::<u32>().unwrap());
                        }
                    }
                }
                results.push(numbers);
            }
        }
    }

    results
}

pub fn part_one(input: &str) -> Option<u32> {
    let results = process_map(input, |c| c.is_ascii_punctuation() && c != '.');
    Some(results.iter().flatten().sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let results = process_map(input, |c| c == '*');
    Some(
        results
            .iter()
            .filter(|v| v.len() == 2)
            .map(|v| v[0] * v[1])
            .sum(),
    )
}
fn parse_map(map_str: &str) -> Vec<Vec<char>> {
    map_str.lines().map(|line| line.chars().collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
