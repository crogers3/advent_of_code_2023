use std::iter::zip;

struct Terrain {
    rows: Vec<String>,
    cols: Vec<String>,
}

impl Terrain {
    fn parse_input(input: &str) -> Terrain {
        let rows = input.lines().map(String::from).collect::<Vec<String>>();
        let cols = (0..rows[0].len())
            .map(|col_idx| {
                rows.iter()
                    .map(|row| row.as_bytes()[col_idx] as char)
                    .collect::<String>()
            })
            .collect::<Vec<String>>();
        Terrain { rows, cols }
    }

    fn find_reflection(&self) -> Reflection {
        find_1d_reflection(&self.rows).map_or_else(
            || {
                find_1d_reflection(&self.cols)
                    .map(|idx| Reflection::Col(idx))
                    .unwrap()
            },
            |idx| Reflection::Row(idx),
        )
    }
}

fn find_1d_reflection(dim: &Vec<String>) -> Option<usize> {
    (1..dim.len())
        .filter_map(|idx| {
            if dim[idx - 1] == dim[idx] {
                Some(idx - 1)
            } else {
                None
            }
        })
        .find_map(|idx| {
            if is_reflection_line(idx, dim) {
                Some(idx)
            } else {
                None
            }
        })
}

fn is_reflection_line(idx: usize, dim: &Vec<String>) -> bool {
    let mut iter_left = (0..=idx).rev();
    let mut iter_right = idx + 1..dim.len();

    while let (Some(left), Some(right)) = (iter_left.next(), iter_right.next()) {
        if dim[left] != dim[right] {
            return false;
        }
    }
    return true;
}

enum Reflection {
    Row(usize),
    Col(usize),
}

fn part_1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(Terrain::parse_input)
        .map(|t| t.find_reflection())
        .map(|r| match r {
            Reflection::Row(idx) => 100 * (idx + 1),
            Reflection::Col(idx) => idx + 1,
        })
        .sum()
}

fn hamming_distance(left: &str, right: &str) -> usize {
    zip(left.chars(), right.chars())
        .map(|(l, r)| if l == r { 0 } else { 1 })
        .sum()
}

fn is_reflection_line_with_smudge(idx: usize, dim: &Vec<String>) -> bool {
    let mut iter_left = (0..=idx).rev();
    let mut iter_right = idx + 1..dim.len();
    let mut total_hamming_distance = 0;

    while let (Some(left), Some(right)) = (iter_left.next(), iter_right.next()) {
        match hamming_distance(&dim[left], &dim[right]) {
            d @ 0 | d @ 1 => total_hamming_distance += d,
            _ => return false,
        }
    }
    return total_hamming_distance == 1;
}

fn find_reflection_1d_with_smudge(dim: &Vec<String>) -> Option<usize> {
    (1..dim.len())
        .filter_map(|idx| match hamming_distance(&dim[idx - 1], &dim[idx]) {
            0 | 1 => Some(idx - 1),
            _ => None,
        })
        .find_map(|idx| {
            if is_reflection_line_with_smudge(idx, dim) {
                Some(idx)
            } else {
                None
            }
        })
}

impl Terrain {
    fn find_reflection_with_smudge(&self) -> Reflection {
        find_reflection_1d_with_smudge(&self.rows).map_or_else(
            || {
                find_reflection_1d_with_smudge(&self.cols)
                    .map(|idx| Reflection::Col(idx))
                    .unwrap()
            },
            |idx| Reflection::Row(idx),
        )
    }
}

fn part_2(input: &str) -> usize {
    input
        .split("\n\n")
        .map(Terrain::parse_input)
        .map(|t| t.find_reflection_with_smudge())
        .map(|r| match r {
            Reflection::Row(idx) => 100 * (idx + 1),
            Reflection::Col(idx) => idx + 1,
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::aoc_2023::data::day_13::*;
    use crate::aoc_2023::day_13::*;

    #[test]
    fn part_1_sample() {
        let result = part_1(SAMPLE);
        assert_eq!(result, 405);
    }

    #[test]
    fn part_1_input() {
        let result = part_1(INPUT);
        assert_eq!(result, 34772);
    }

    #[test]
    fn part_2_sample() {
        let result = part_2(SAMPLE);
        assert_eq!(result, 400);
    }

    #[test]
    fn part_2_input() {
        let result = part_2(INPUT);
        assert_eq!(result, 35554);
    }
}
