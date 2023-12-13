use std::collections::HashMap;
use std::hash::Hash;
use std::iter;

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

impl Condition {
    fn parse_char(c: char) -> Condition {
        match c {
            '#' => Condition::Damaged,
            '.' => Condition::Operational,
            '?' => Condition::Unknown,
            _ => panic!(),
        }
    }
}

#[derive(Hash, Clone, Eq, PartialEq)]
struct Row {
    springs: Vec<Condition>,
    damaged_groups: Vec<usize>,
}

impl Row {
    fn parse_line(line: &str) -> Row {
        let (springs, damaged_groups) = line.split_once(' ').unwrap();
        Row {
            springs: springs.chars().map(Condition::parse_char).collect(),
            damaged_groups: damaged_groups
                .split(',')
                .map(|c| c.parse::<usize>().unwrap())
                .collect(),
        }
    }

    fn parse_line_part_2(line: &str) -> Row {
        let (springs, damaged_groups) = line.split_once(' ').unwrap();
        let springs = iter::repeat(springs)
            .take(5)
            .map(String::from)
            .collect::<Vec<String>>()
            .join("?");
        let damaged_groups = iter::repeat(damaged_groups)
            .take(5)
            .map(String::from)
            .collect::<Vec<String>>()
            .join(",");
        Row::parse_line(&format!("{springs} {damaged_groups}"))
    }

    fn possibilities_r(&self, memo: &mut HashMap<Row, usize>) -> usize {
        if memo.contains_key(self) {
            return *memo.get(self).unwrap();
        }

        if self.damaged_groups.is_empty() {
            let possibilities = if self.springs.iter().any(|c| *c == Condition::Damaged) {
                0
            } else {
                1
            };
            memo.insert(self.clone(), possibilities);
            return possibilities;
        }

        if self.springs.is_empty() {
            memo.insert(self.clone(), 0);
            return 0;
        }

        if self.springs[0] == Condition::Operational {
            let possibilites = (Row {
                springs: self.springs[1..].to_vec(),
                damaged_groups: self.damaged_groups.clone(),
            })
            .possibilities_r(memo);
            memo.insert(self.clone(), possibilites);
            return possibilites;
        }

        if self.springs[0] == Condition::Damaged {
            let next_group = self.damaged_groups[0];
            if self.springs.len() >= next_group
                && self.springs[0..next_group]
                    .iter()
                    .all(|c| *c != Condition::Operational)
                && !matches!(self.springs.get(next_group), Some(Condition::Damaged))
            {
                let possibilities = (Row {
                    springs: self.springs[self.springs.len().min(next_group + 1)..].to_vec(),
                    damaged_groups: self.damaged_groups[1..].to_vec(),
                })
                .possibilities_r(memo);
                memo.insert(self.clone(), possibilities);
                return possibilities;
            }
            memo.insert(self.clone(), 0);
            return 0;
        }

        let possibilities = (Row {
            springs: [vec![Condition::Damaged], self.springs[1..].to_vec()].concat(),
            damaged_groups: self.damaged_groups.clone(),
        })
        .possibilities_r(memo)
            + (Row {
                springs: [vec![Condition::Operational], self.springs[1..].to_vec()].concat(),
                damaged_groups: self.damaged_groups.clone(),
            })
            .possibilities_r(memo);
        memo.insert(self.clone(), possibilities);
        return possibilities;
    }
}

fn part_1(input: &str) -> usize {
    input
        .lines()
        .map(Row::parse_line)
        .map(|r| r.possibilities_r(&mut HashMap::new()))
        .sum()
}

fn part_2(input: &str) -> usize {
    input
        .lines()
        .map(Row::parse_line_part_2)
        .map(|r| r.possibilities_r(&mut HashMap::new()))
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::aoc_2023::data::day_12::*;
    use crate::aoc_2023::day_12::*;

    #[test]
    fn part_1_sample() {
        let result = part_1(SAMPLE);
        assert_eq!(result, 21);
    }

    #[test]
    fn part_1_input() {
        let result = part_1(INPUT);
        assert_eq!(result, 6852);
    }

    #[test]
    fn part_2_sample() {
        let result = part_2(SAMPLE);
        assert_eq!(result, 525152);
    }

    #[test]
    fn part_2_input() {
        let result = part_2(INPUT);
        assert_eq!(result, 8475948826693);
    }
}
