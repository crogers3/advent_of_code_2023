use num::integer::lcm;
use regex::Regex;
use std::collections::HashMap;

type Graph = HashMap<String, (String, String)>;

enum Direction {
    R,
    L,
}

impl Direction {
    fn from_char(c: char) -> Direction {
        match c {
            'R' => Direction::R,
            'L' => Direction::L,
            _ => panic!(),
        }
    }
}

struct Map {
    directions: Vec<Direction>,
    graph: Graph,
}

impl Map {
    fn parse_input(input: &str) -> Map {
        let mut lines = input.lines();
        let directions = lines
            .next()
            .unwrap()
            .chars()
            .map(Direction::from_char)
            .collect::<Vec<Direction>>();
        lines.next();

        let regex = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();
        let graph = lines
            .map(|line| {
                let captures = regex.captures(line).unwrap();
                (
                    captures[1].to_string(),
                    (captures[2].to_string(), captures[3].to_string()),
                )
            })
            .collect::<Graph>();

        return Map { directions, graph };
    }
}

fn path_length(input: &str, starting_node: &str, ending_predicate: fn(&str) -> bool) -> usize {
    let map = Map::parse_input(input);
    let mut node = starting_node.to_string();
    let mut num_steps = 0;

    for direction in map.directions.iter().cycle() {
        if ending_predicate(&node) {
            break;
        }
        num_steps += 1;
        let children = map.graph.get(&node).unwrap();
        node = match direction {
            Direction::L => children.0.clone(),
            Direction::R => children.1.clone(),
        }
    }

    return num_steps;
}

fn part_1_brute(input: &str) -> usize {
    path_length(input, "AAA", |node| node == "ZZZ")
}

fn part_2(input: &str) -> usize {
    let map = Map::parse_input(input);

    return map
        .graph
        .keys()
        .filter(|node| node.ends_with('A'))
        .map(|node| path_length(input, node, |n| n.ends_with('Z')))
        .fold(1, |acc, g| lcm(acc, g));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc_2023::data::day_8::*;

    #[test]
    fn part_1_sample_1() {
        let result = part_1_brute(SAMPLE_1);
        assert_eq!(result, 2);
    }

    #[test]
    fn part_1_sample_2() {
        let result = part_1_brute(SAMPLE_2);
        assert_eq!(result, 6);
    }

    #[test]
    fn part_1_input() {
        let result = part_1_brute(INPUT);
        assert_eq!(result, 14429);
    }

    #[test]
    fn part_2_sample() {
        let result = part_2(SAMPLE_3);
        assert_eq!(result, 6);
    }

    #[test]
    fn part_2_input() {
        let result = part_2(INPUT);
        dbg!(result);
    }
}
