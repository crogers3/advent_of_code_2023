fn part_1(input: &str) -> usize {
    let mut sum = 0;
    for line in input.lines() {
        sum += parse_line(&line);
    }
    return sum;
}

fn parse_line(line: &str) -> usize {
    let nums = line
        .chars()
        .map(|c| c.to_digit(10))
        .filter(Option::is_some)
        .map(Option::unwrap)
        .map(|n| n as usize)
        .collect::<Vec<usize>>();
    return (nums.first().unwrap() * 10) + nums.last().unwrap();
}

fn part_2(input: &str) -> usize {
    let mut sum = 0;
    for line in input.lines() {
        let first = get_first(&line);
        let last = get_last(&line);
        let number = (first * 10) + last;
        sum += number;
    }
    return sum;
}

fn get_first(line: &str) -> usize {
    for i in 0..line.len() {
        match start_digit(&line[i..]) {
            Some(x) => return x,
            None => continue,
        }
    }
    panic!()
}

fn get_last(line: &str) -> usize {
    for i in (0..line.len()).rev() {
        match start_digit(&line[i..]) {
            Some(x) => return x,
            None => continue,
        }
    }
    panic!()
}

fn start_digit(line: &str) -> Option<usize> {
    match line {
        x if x.starts_with("1") || x.starts_with("one") => Some(1),
        x if x.starts_with("2") || x.starts_with("two") => Some(2),
        x if x.starts_with("3") || x.starts_with("three") => Some(3),
        x if x.starts_with("4") || x.starts_with("four") => Some(4),
        x if x.starts_with("5") || x.starts_with("five") => Some(5),
        x if x.starts_with("6") || x.starts_with("six") => Some(6),
        x if x.starts_with("7") || x.starts_with("seven") => Some(7),
        x if x.starts_with("8") || x.starts_with("eight") => Some(8),
        x if x.starts_with("9") || x.starts_with("nine") => Some(9),
        x if x.starts_with("0") || x.starts_with("zero") => Some(0),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::super::data::day_1::*;
    use super::*;

    #[test]
    fn part_1_sample() {
        let result = part_1(SAMPLE);
        assert_eq!(result, 142);
    }

    #[test]
    fn part_1_input() {
        let result = part_1(INPUT);
        assert_eq!(result, 54450);
    }

    #[test]
    fn part_2_sample() {
        let result = part_2(SAMPLE_2);
        assert_eq!(result, 281)
    }

    #[test]
    fn part_2_input() {
        let result = part_2(INPUT);
        assert_eq!(result, 54265);
    }
}
