#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
enum Space {
    Round,
    Empty,
    Cube,
}

impl Space {
    fn parse_char(c: char) -> Space {
        match c {
            'O' => Space::Round,
            '#' => Space::Cube,
            '.' => Space::Empty,
            _ => panic!(),
        }
    }
}

fn parse_cols(input: &str) -> Vec<Vec<Space>> {
    let mut cols = Vec::new();
    for _ in 0..input.lines().next().unwrap().len() {
        cols.push(Vec::new());
    }

    input.lines().for_each(|line| {
        line.chars()
            .enumerate()
            .for_each(|(col_idx, c)| cols[col_idx].push(Space::parse_char(c)))
    });

    return cols;
}

fn compress(col: &Vec<Space>, reverse: bool) -> Vec<Space> {
    col.split(|s| matches!(s, Space::Cube))
        .map(|spaces| {
            let mut sorted = Vec::from(spaces);
            sorted.sort();
            if reverse {
                sorted.reverse();
            }
            return sorted;
        })
        .collect::<Vec<Vec<Space>>>()
        .join(&Space::Cube)
}

fn calc_load(spaces: &Vec<Vec<Space>>) -> usize {
    let num_rows = spaces.len();
    transpose(spaces)
        .iter()
        .map(|col| {
            col.iter().enumerate().fold(0, |acc, (row, space)| {
                if matches!(space, Space::Round) {
                    acc + num_rows - row
                } else {
                    acc
                }
            })
        })
        .sum()
}

fn part_1(input: &str) -> usize {
    let spaces = parse_spaces(input);
    calc_load(&compress_north(&spaces))
}

fn parse_spaces(input: &str) -> Vec<Vec<Space>> {
    input
        .lines()
        .map(|line| line.chars().map(Space::parse_char).collect::<Vec<Space>>())
        .collect::<Vec<Vec<Space>>>()
}

fn transpose(spaces: &Vec<Vec<Space>>) -> Vec<Vec<Space>> {
    let mut transposed = Vec::new();
    for _ in 0..spaces.len() {
        transposed.push(Vec::new());
    }
    spaces.iter().for_each(|line| {
        line.iter()
            .enumerate()
            .for_each(|(idx, space)| transposed[idx].push(space.clone()))
    });
    return transposed;
}

fn compress_north(spaces: &Vec<Vec<Space>>) -> Vec<Vec<Space>> {
    transpose(
        &transpose(spaces)
            .iter()
            .map(|col| compress(col, false))
            .collect::<Vec<Vec<Space>>>(),
    )
}

fn compress_south(spaces: &Vec<Vec<Space>>) -> Vec<Vec<Space>> {
    transpose(
        &transpose(spaces)
            .iter()
            .map(|col| compress(col, true))
            .collect::<Vec<Vec<Space>>>(),
    )
}

fn compress_west(spaces: &Vec<Vec<Space>>) -> Vec<Vec<Space>> {
    spaces
        .iter()
        .map(|col| compress(col, false))
        .collect::<Vec<Vec<Space>>>()
}

fn compress_east(spaces: &Vec<Vec<Space>>) -> Vec<Vec<Space>> {
    spaces
        .iter()
        .map(|col| compress(col, true))
        .collect::<Vec<Vec<Space>>>()
}

fn spin(spaces: &Vec<Vec<Space>>) -> Vec<Vec<Space>> {
    compress_east(&compress_south(&compress_west(&compress_north(&spaces))))
}

fn part_2(input: &str) -> usize {
    let mut spaces = parse_spaces(input);

    for _ in 0..1000 {
        spaces = spin(&spaces);
    }

    calc_load(&spaces)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc_2023::data::day_14::*;

    #[test]
    fn part_1_sample() {
        let result = part_1(SAMPLE);
        assert_eq!(result, 136);
    }

    #[test]
    fn part_1_input() {
        let result = part_1(INPUT);
        assert_eq!(result, 109466);
    }

    #[test]
    fn part_2_sample() {
        let result = part_2(SAMPLE);
        assert_eq!(result, 64);
    }

    #[test]
    fn part_2_input() {
        let result = part_2(INPUT);
        dbg!(result);
    }
}
