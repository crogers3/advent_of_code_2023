use std::num::Wrapping;

fn compute_hash(input: &str) -> u8 {
    input
        .as_bytes()
        .iter()
        .fold(Wrapping(0u8), |acc, b| (acc + Wrapping(*b)) * Wrapping(17))
        .0
}

fn part_1(input: &str) -> usize {
    input.split(',').map(|s| compute_hash(s) as usize).sum()
}

#[derive(Clone)]
struct Lens {
    label: String,
    focal: u8,
}

enum Operation {
    Add(Lens),
    Remove(String),
}

fn parse_operations(input: &str) -> Vec<Operation> {
    input
        .split(',')
        .map(|s| match s.chars().last().unwrap() {
            '-' => Operation::Remove(String::from(&s[..s.len() - 1])),
            _ => Operation::Add(Lens {
                label: String::from(&s[..s.len() - 2]),
                focal: String::from(s.chars().last().unwrap())
                    .parse::<u8>()
                    .unwrap(),
            }),
        })
        .collect::<Vec<Operation>>()
}

fn total_focus_power(boxes: &Vec<Vec<Lens>>) -> usize {
    boxes
        .iter()
        .enumerate()
        .map(|(box_idx, b)| {
            b.iter()
                .enumerate()
                .map(|(slot_idx, lens)| (box_idx + 1) * (slot_idx + 1) * (lens.focal as usize))
                .sum::<usize>()
        })
        .sum()
}

fn part_2(input: &str) -> usize {
    let mut boxes = (0..256).map(|_| Vec::new()).collect::<Vec<Vec<Lens>>>();

    parse_operations(input)
        .iter()
        .map(|op| match op {
            Operation::Add(Lens { label, focal: _ }) => (op, compute_hash(label)),
            Operation::Remove(label) => (op, compute_hash(label)),
        })
        .for_each(|(op, h)| match op {
            Operation::Add(lens) => {
                match boxes[h as usize]
                    .iter()
                    .enumerate()
                    .find(|(_, l)| l.label == lens.label)
                {
                    Some((idx, _)) => boxes[h as usize][idx] = lens.clone(),
                    None => boxes[h as usize].push(lens.clone()),
                }
            }
            Operation::Remove(label) => match boxes[h as usize]
                .iter()
                .enumerate()
                .find(|(_, l)| l.label == *label)
            {
                Some((idx, _)) => {
                    boxes[h as usize].remove(idx);
                }
                None => (),
            },
        });

    total_focus_power(&boxes)
}

#[cfg(test)]
mod tests {
    use crate::aoc_2023::data::day_15::*;
    use crate::aoc_2023::day_15::*;

    #[test]
    fn part_1_sample() {
        let result = part_1(SAMPLE);
        assert_eq!(result, 1320);
    }

    #[test]
    fn part_1_input() {
        let result = part_1(INPUT);
        assert_eq!(result, 494980);
    }

    #[test]
    fn part_2_sample() {
        let result = part_2(SAMPLE);
        assert_eq!(result, 145);
    }

    #[test]
    fn part_2_input() {
        let result = part_2(INPUT);
        assert_eq!(result, 247933);
    }
}
