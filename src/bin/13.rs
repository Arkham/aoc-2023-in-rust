advent_of_code::solution!(13);

type Grid = Vec<Vec<char>>;

fn parse_input(input: &str) -> Vec<Grid> {
    let grids_str = input.split("\n\n").collect::<Vec<&str>>();
    let mut grids = Vec::new();

    for grid_str in grids_str {
        let grid = grid_str
            .lines()
            .map(|line| line.chars().collect())
            .collect();
        grids.push(grid);
    }

    grids
}

pub fn part_one(input: &str) -> Option<u32> {
    let grids = parse_input(input);

    let mut total = 0;
    for grid in grids {
        // print_grid(&grid);
        if let Some(idx) = find_horizontal_reflection(&grid) {
            // println!("found horizontal reflection at {}!", idx);
            total += idx * 100;
        } else {
            let transposed = transpose(&grid);
            if let Some(idx) = find_horizontal_reflection(&transposed) {
                // println!("found vertical reflection at {}!", idx);
                total += idx;
            }
        }
        // println!();
    }

    Some(total)
}

fn find_horizontal_reflection(grid: &Grid) -> Option<u32> {
    for i in 1..grid.len() {
        let reflection_span = i.min(grid.len() - i);
        let mut reflection_found = true;
        for j in 1..=reflection_span {
            if grid[i - j] != grid[i + j - 1] {
                reflection_found = false;
                break;
            }
        }
        if reflection_found {
            return Some(i as u32);
        }
    }
    None
}

fn find_reflection_candidates(grid: &Grid) -> Vec<(usize, usize)> {
    let mut results = vec![];
    for i in 1..grid.len() {
        let reflection_span = i.min(grid.len() - i);
        let mut diff_total = 0;
        for j in 1..=reflection_span {
            diff_total += diff_vecs(&grid[i - j], &grid[i + j - 1]);
        }
        results.push((i, diff_total));
    }
    results.sort_by(|(_, a), (_, b)| a.cmp(b));
    results
}

fn diff_vecs<T: PartialEq>(vec1: &[T], vec2: &[T]) -> usize {
    vec1.iter().zip(vec2).filter(|&(a, b)| a != b).count()
}

fn transpose(grid: &Grid) -> Grid {
    let mut transposed = vec![vec![' '; grid.len()]; grid[0].len()];

    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            transposed[j][i] = cell;
        }
    }

    transposed
}

#[allow(dead_code)]
fn print_grid(grid: &Grid) {
    for row in grid {
        for col in row {
            print!("{}", col);
        }
        println!();
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    let grids = parse_input(input);

    let mut total = 0;
    for grid in grids {
        // print_grid(&grid);
        match find_reflection_candidates(&grid)[..] {
            [(_, 0), (idx, 1), ..] | [(idx, 1), ..] => {
                // println!("found horizontal reflection at {}!", idx);
                total += idx * 100;
            }
            _ => {
                let transposed = transpose(&grid);
                match find_reflection_candidates(&transposed)[..] {
                    [(_, 0), (idx, 1), ..] | [(idx, 1), ..] => {
                        // println!("found vertical reflection at {}!", idx);
                        total += idx;
                    }
                    _ => {
                        panic!("no reflection found!")
                    }
                }
            }
        }
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_bug1() {
        let result = part_one(
            "#.##...
####...
.##.#.#
.###..#
#..#.##
.###..#
.###..#",
        );
        assert_eq!(result, Some(600));
    }

    #[test]
    fn test_simple_part_two() {
        let result = part_one(
            "#.##...
####...
.##.#.#
.###..#
#..#.##
.###..#
.###..#",
        );
        assert_eq!(result, Some(600));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
