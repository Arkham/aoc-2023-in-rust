advent_of_code::solution!(17);

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect()
        })
        .collect()
}

type Pos = (usize, usize);

#[derive(Clone, Copy, Eq, Hash, PartialEq, Ord, PartialOrd)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

fn dir_offset(dir: &Dir) -> (isize, isize) {
    match dir {
        Dir::Right => (1, 0),
        Dir::Up => (0, -1),
        Dir::Left => (-1, 0),
        Dir::Down => (0, 1),
    }
}

type DirWithCount = (Dir, u8);

#[derive(Clone, Copy, Eq, Hash, PartialEq, Ord, PartialOrd)]
struct State {
    pos: Pos,
    dir_with_count: DirWithCount,
}

fn get_cost(
    map: &Vec<Vec<u8>>,
    source: Pos,
    destination: Pos,
    min_length: u8,
    crucible_logic: fn(DirWithCount) -> Vec<DirWithCount>,
) -> Option<usize> {
    let mut cache: HashMap<State, usize> = HashMap::new();
    let mut heap: BinaryHeap<Reverse<(usize, State)>> = BinaryHeap::new();

    let start_state = State {
        pos: source,
        dir_with_count: (Dir::Right, 0),
    };

    cache.insert(start_state, 0);
    heap.push(Reverse((0, start_state)));

    while let Some(Reverse((curr_cost, curr_state))) = heap.pop() {
        if curr_state.pos == destination && curr_state.dir_with_count.1 >= min_length {
            return Some(curr_cost);
        }

        if cache.get(&curr_state).map_or(true, |c| *c < curr_cost) {
            continue;
        }

        let next_states = get_next(curr_state, map, crucible_logic);

        for next_state in next_states {
            let next_pos = next_state.pos;
            let next_cost = curr_cost + (map[next_pos.1][next_pos.0] as usize);
            if cache.get(&next_state).map_or(true, |c| *c > next_cost) {
                heap.push(Reverse((next_cost, next_state)));
                cache.insert(next_state, next_cost);
            }
        }
    }

    None
}

fn crucible_logic_part1((dir, count): DirWithCount) -> Vec<DirWithCount> {
    let mut turns = match dir {
        Dir::Down | Dir::Up => vec![(Dir::Right, 1), (Dir::Left, 1)],
        Dir::Left | Dir::Right => vec![(Dir::Down, 1), (Dir::Up, 1)],
    };

    if count < 3 {
        turns.push((dir, count + 1));
    }
    turns
}

fn crucible_logic_part2((dir, count): DirWithCount) -> Vec<DirWithCount> {
    let mut turns = vec![];

    if count >= 4 {
        match dir {
            Dir::Down | Dir::Up => turns.extend(vec![(Dir::Right, 1), (Dir::Left, 1)]),
            Dir::Left | Dir::Right => turns.extend(vec![(Dir::Down, 1), (Dir::Up, 1)]),
        }
    };

    if count < 10 {
        turns.push((dir, count + 1));
    }
    turns
}

fn get_next(
    state: State,
    map: &Vec<Vec<u8>>,
    crucible_logic: fn(DirWithCount) -> Vec<DirWithCount>,
) -> Vec<State> {
    let rows = map.len();
    let cols = map[0].len();
    let turns = crucible_logic(state.dir_with_count);

    let mut result = vec![];
    for (dir, count) in turns {
        let (dx, dy) = dir_offset(&dir);
        let new_x = state.pos.0 as isize + dx;
        let new_y = state.pos.1 as isize + dy;
        if new_x >= 0 && new_x < cols as isize && new_y >= 0 && new_y < rows as isize {
            result.push(State {
                pos: (new_x as usize, new_y as usize),
                dir_with_count: (dir, count),
            });
        }
    }
    result
}

pub fn part_one(input: &str) -> Option<usize> {
    let map: Vec<Vec<u8>> = parse_input(input);
    let rows = map.len();
    let cols = map[0].len();
    get_cost(&map, (0, 0), (cols - 1, rows - 1), 1, crucible_logic_part1)
}

pub fn part_two(input: &str) -> Option<usize> {
    let map: Vec<Vec<u8>> = parse_input(input);
    let rows = map.len();
    let cols = map[0].len();
    get_cost(&map, (0, 0), (cols - 1, rows - 1), 4, crucible_logic_part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_big_crucible() {
        let result = part_two(
            "111111111111
999999999991
999999999991
999999999991
999999999991",
        );
        assert_eq!(result, Some(71));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }
}
