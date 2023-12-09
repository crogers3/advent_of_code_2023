fn next_point(input: &Vec<isize>) -> isize {
    if input.iter().all(|p| *p == 0) {
        return 0;
    }
    let mut diffs = Vec::new();
    for i in 1..input.len() {
        diffs.push(input[i] - input[i - 1]);
    }
    let next_diff = next_point(&diffs);
    return input.last().unwrap() + next_diff;
}

fn prev_point(input: &Vec<isize>) -> isize {
    if input.iter().all(|p| *p == 0) {
        return 0;
    }
    let mut diffs = Vec::new();
    for i in 1..input.len() {
        diffs.push(input[i] - input[i - 1]);
    }
    let prev_diff = prev_point(&diffs);
    return input[0] - prev_diff;
}

fn part_1(input: &str) -> isize {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|p| p.parse::<isize>().unwrap())
                .collect::<Vec<isize>>()
        })
        .map(|line| next_point(&line))
        .sum()
}

fn part_2(input: &str) -> isize {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|p| p.parse::<isize>().unwrap())
                .collect::<Vec<isize>>()
        })
        .map(|line| prev_point(&line))
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::aoc_2023::data::day_9::*;
    use crate::aoc_2023::day_9::*;

    #[test]
    fn part_1_sample() {
        let result = part_1(SAMPLE);
        assert_eq!(result, 114);
    }

    #[test]
    fn part_1_input() {
        let result = part_1(INPUT);
        assert_eq!(result, 1969958987);
    }

    #[test]
    fn part_2_sample() {
        let result = part_2(SAMPLE);
        assert_eq!(result, 2);
    }

    #[test]
    fn part_2_input() {
        let result = part_2(INPUT);
        dbg!(result);
    }
}
