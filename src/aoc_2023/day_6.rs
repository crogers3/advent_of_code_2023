use roots::{find_roots_quadratic, Roots};
use std::iter::zip;

struct Race {
    time: usize,
    dist: usize,
}

fn parse_input(input: &str) -> Vec<Race> {
    let lines = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .skip(1)
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();
    return zip(lines[0].iter(), lines[1].iter())
        .map(|(time, dist)| Race {
            time: *time,
            dist: *dist,
        })
        .collect::<Vec<Race>>();
}

// x = time button held
// t = race time
// d = record distance
// x * (t-x) > d == winning conditions
// x^2 - tx + d < 0

fn num_ways(race: &Race) -> usize {
    let roots = find_roots_quadratic(1.0, race.time as f64 * -1.0, race.dist as f64);

    return match roots {
        Roots::Two([lower, upper]) => (upper.ceil() - lower.floor()) as usize - 1,
        _ => panic!(),
    };
}

fn part_1(input: &str) -> usize {
    parse_input(input)
        .iter()
        .map(num_ways)
        .fold(1, |x, y| x * y)
}

fn parse_for_part_2(input: &str) -> Race {
    let [time, dist] = input
        .lines()
        .map(str::split_whitespace)
        .map(|s| s.skip(1))
        .map(|nums| nums.collect::<String>())
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()[0..2]
    else {
        panic!()
    };
    return Race { time, dist };
}

fn part_2(input: &str) -> usize {
    num_ways(&parse_for_part_2(input))
}

#[cfg(test)]
mod tests {
    use crate::aoc_2023::data::day_6::*;
    use crate::aoc_2023::day_6::*;

    #[test]
    fn part_1_sample() {
        let result = part_1(SAMPLE);
        assert_eq!(result, 288);
    }

    #[test]
    fn part_1_input() {
        let result = part_1(INPUT);
        assert_eq!(result, 252000);
    }

    #[test]
    fn part_2_sample() {
        let result = part_2(SAMPLE);
        assert_eq!(result, 71503);
    }

    #[test]
    fn part_2_input() {
        let result = part_2(INPUT);
        assert_eq!(result, 36992486);
    }
}
