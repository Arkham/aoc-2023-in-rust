advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<usize> {
    Some(input.trim().split(',').map(magic_hash).sum())
}

fn magic_hash(input: &str) -> usize {
    let mut current_value: usize = 0;

    for character in input.chars() {
        let ascii_code = character as u8;
        current_value = ((current_value + ascii_code as usize) * 17) % 256;
    }

    current_value
}

#[derive(Debug, Clone)]
struct Lens {
    label: String,
    focal_length: u8,
}

pub fn part_two(input: &str) -> Option<usize> {
    let steps: Vec<&str> = input.trim().split(',').collect();

    let mut boxes: Vec<Vec<Lens>> = vec![Vec::new(); 256];

    for step in steps {
        let (label, operation, focal_length) = if let Some(index) = step.find('=') {
            let (label, rest) = step.split_at(index);
            let focal_length: u8 = rest[1..].parse().unwrap();
            (label, "=", focal_length)
        } else {
            let label = step.trim_end_matches('-');
            (label, "-", 0)
        };

        let box_id = magic_hash(label);

        match operation {
            "=" => {
                let lens = Lens {
                    label: label.to_string(),
                    focal_length,
                };

                if let Some(index) = boxes[box_id].iter().position(|l| l.label == label) {
                    boxes[box_id][index] = lens;
                } else {
                    boxes[box_id].push(lens);
                }
            }
            "-" => {
                boxes[box_id].retain(|l| l.label != label);
            }
            _ => (),
        }
    }

    let mut total_focusing_power: usize = 0;

    for (box_id, box_lenses) in boxes.iter().enumerate() {
        for (slot_number, lens) in box_lenses.iter().enumerate() {
            total_focusing_power += (1 + box_id) * (1 + slot_number) * (lens.focal_length as usize);
        }
    }

    Some(total_focusing_power)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
