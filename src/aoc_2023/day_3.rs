#[derive(Copy, Clone)]
enum Character {
    Digit(char),
    Period,
    Symbol(char),
}

impl Character {
    fn from_char(c: char) -> Character {
        match c {
            '.' => Character::Period,
            x if x.is_ascii_digit() => Character::Digit(x),
            x => Character::Symbol(x),
        }
    }

    fn touches_symbol(schematic: &Vec<Vec<Character>>, line_idx: usize, c_idx: usize) -> bool {
        // Left
        if c_idx > 0 {
            if let Some(c) = schematic
                .get(line_idx)
                .map(|line| line.get(c_idx - 1))
                .flatten()
            {
                if let Character::Symbol(_) = c {
                    return true;
                }
            }
        }
        // Right
        if let Some(c) = schematic
            .get(line_idx)
            .map(|line| line.get(c_idx + 1))
            .flatten()
        {
            if let Character::Symbol(_) = c {
                return true;
            }
        }
        if line_idx > 0 {
            // Up Left
            if c_idx > 0 {
                if let Some(c) = schematic
                    .get(line_idx - 1)
                    .map(|line| line.get(c_idx - 1))
                    .flatten()
                {
                    if let Character::Symbol(_) = c {
                        return true;
                    }
                }
            }
            // Up
            if let Some(c) = schematic
                .get(line_idx - 1)
                .map(|line| line.get(c_idx))
                .flatten()
            {
                if let Character::Symbol(_) = c {
                    return true;
                }
            }
            // Up Right
            if let Some(c) = schematic
                .get(line_idx - 1)
                .map(|line| line.get(c_idx + 1))
                .flatten()
            {
                if let Character::Symbol(_) = c {
                    return true;
                }
            }
        }
        // Down Left
        if c_idx > 0 {
            if let Some(c) = schematic
                .get(line_idx + 1)
                .map(|line| line.get(c_idx - 1))
                .flatten()
            {
                if let Character::Symbol(_) = c {
                    return true;
                }
            }
        }
        // Down
        if let Some(c) = schematic
            .get(line_idx + 1)
            .map(|line| line.get(c_idx))
            .flatten()
        {
            if let Character::Symbol(_) = c {
                return true;
            }
        }
        // Down Right
        if let Some(c) = schematic
            .get(line_idx + 1)
            .map(|line| line.get(c_idx + 1))
            .flatten()
        {
            if let Character::Symbol(_) = c {
                return true;
            }
        }
        return false;
    }
}

fn parse_schematic(input: &str) -> Vec<Vec<Character>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(Character::from_char)
                .collect::<Vec<Character>>()
        })
        .collect::<Vec<Vec<Character>>>()
}

fn part_1(input: &str) -> usize {
    let mut sum = 0;
    let schematic = parse_schematic(input);

    for (line_idx, line) in schematic.iter().enumerate() {
        let mut part_num = String::new();
        let mut is_valid = false;
        for (c_idx, c) in line.iter().enumerate() {
            if let Character::Digit(d) = c {
                part_num.push(d.clone());
                if Character::touches_symbol(&schematic, line_idx, c_idx) {
                    is_valid = true;
                }
            } else {
                if is_valid {
                    sum += part_num.parse::<usize>().unwrap();
                }
                part_num.clear();
                is_valid = false;
            }
        }
        if is_valid {
            sum += part_num.parse::<usize>().unwrap();
        }
    }

    return sum;
}

fn get_star_idxs(schematic: &Vec<Vec<Character>>) -> Vec<(usize, usize)> {
    let mut star_idxs: Vec<(usize, usize)> = Vec::new();
    for (line_idx, line) in schematic.iter().enumerate() {
        for (c_idx, c) in line.iter().enumerate() {
            if let Character::Symbol('*') = c {
                star_idxs.push((line_idx, c_idx));
            }
        }
    }
    return star_idxs;
}

fn get_char_at(
    schematic: &Vec<Vec<Character>>,
    line_idx: usize,
    c_idx: usize,
) -> Option<Character> {
    schematic
        .get(line_idx)
        .map(|line| line.get(c_idx))
        .flatten()
        .map(Character::clone)
}

fn get_number_at(schematic: &Vec<Vec<Character>>, line_idx: usize, c_idx: usize) -> Option<usize> {
    let c = get_char_at(schematic, line_idx, c_idx);
    if let Some(Character::Symbol(_)) | Some(Character::Period) | None = c {
        return None;
    }

    if c_idx > 0 {
        if let Some(preceding_num) = get_number_at(schematic, line_idx, c_idx - 1) {
            return Some(preceding_num);
        }
    }

    let Some(Character::Digit(c)) = c else {
        panic!()
    };
    let mut num = String::from(c);
    let mut c_idx = c_idx + 1;
    while let Some(Character::Digit(next)) = get_char_at(schematic, line_idx, c_idx) {
        num.push(next);
        c_idx += 1;
    }

    return Some(num.parse::<usize>().unwrap());
}

fn get_surrounding_numbers(
    schematic: &Vec<Vec<Character>>,
    line_idx: usize,
    c_idx: usize,
) -> Vec<usize> {
    let mut surrounding_numbers = Vec::new();

    // Check left
    if c_idx > 0 {
        if let Some(number) = get_number_at(schematic, line_idx, c_idx - 1) {
            surrounding_numbers.push(number);
        }
    }
    // Check right
    if let Some(number) = get_number_at(schematic, line_idx, c_idx + 1) {
        surrounding_numbers.push(number);
    }

    // Check up
    if line_idx > 0 {
        if let Some(number) = get_number_at(schematic, line_idx - 1, c_idx) {
            surrounding_numbers.push(number);
        } else {
            // Check up left
            if c_idx > 0 {
                if let Some(number) = get_number_at(schematic, line_idx - 1, c_idx - 1) {
                    surrounding_numbers.push(number);
                }
            }
            // Check up right
            if let Some(number) = get_number_at(schematic, line_idx - 1, c_idx + 1) {
                surrounding_numbers.push(number);
            }
        }
    }

    // Check down
    if let Some(number) = get_number_at(schematic, line_idx + 1, c_idx) {
        surrounding_numbers.push(number);
    } else {
        // Check down left
        if c_idx > 0 {
            if let Some(number) = get_number_at(schematic, line_idx + 1, c_idx - 1) {
                surrounding_numbers.push(number);
            }
        }
        // Check down right
        if let Some(number) = get_number_at(schematic, line_idx + 1, c_idx + 1) {
            surrounding_numbers.push(number);
        }
    }

    return surrounding_numbers;
}

fn part_2(input: &str) -> usize {
    let mut sum = 0;

    let schematic = parse_schematic(input);
    let star_idxs = get_star_idxs(&schematic);
    for (line_idx, c_idx) in star_idxs.iter() {
        let surrounding_numbers = get_surrounding_numbers(&schematic, *line_idx, *c_idx);
        if surrounding_numbers.len() == 2 {
            sum += surrounding_numbers.iter().fold(1, |acc, e| acc * e);
        }
    }

    return sum;
}

#[cfg(test)]
mod tests {
    use super::super::data::day_3::*;
    use super::*;

    #[test]
    fn part_1_sample() {
        let result = part_1(SAMPLE);
        assert_eq!(result, 4361);
    }

    #[test]
    fn part_1_input() {
        let result = part_1(INPUT);
        assert_eq!(result, 544433);
    }

    #[test]
    fn part_2_sample() {
        let result = part_2(SAMPLE);
        assert_eq!(result, 467835);
    }

    #[test]
    fn part_2_input() {
        let result = part_2(INPUT);
        assert_eq!(result, 76314915);
    }
}
