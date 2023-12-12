use std::collections::HashSet;

type Point = (usize, usize);

struct Image {
    galaxies: Vec<Point>,
    empty_rows: Vec<usize>,
    empty_cols: Vec<usize>,
}

impl Image {
    fn parse_input(input: &str) -> Image {
        let galaxies = input
            .lines()
            .enumerate()
            .flat_map(|(row_idx, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(|(col_idx, c)| {
                        if matches!(c, '#') {
                            return Some((row_idx, col_idx));
                        }
                        None
                    })
                    .collect::<Vec<Point>>()
            })
            .collect::<Vec<Point>>();
        let galaxy_rows = galaxies.iter().map(|p| p.0).collect::<HashSet<usize>>();
        let galaxy_cols = galaxies.iter().map(|p| p.1).collect::<HashSet<usize>>();

        let empty_rows = (0..input.lines().count())
            .filter(|row| !galaxy_rows.contains(row))
            .collect::<Vec<usize>>();
        let empty_cols = (0..input.lines().next().unwrap().len())
            .filter(|col| !galaxy_cols.contains(col))
            .collect::<Vec<usize>>();

        Image {
            galaxies,
            empty_rows,
            empty_cols,
        }
    }
}

fn part_x(input: &str, dist: usize) -> usize {
    let image = Image::parse_input(input);
    let mut sum = 0;
    for (g1_idx, g1) in image.galaxies.iter().enumerate() {
        for (g2_idx, g2) in image.galaxies.iter().enumerate() {
            if g1_idx == g2_idx {
                continue;
            }
            let lowest_empty_row_idx = image.empty_rows.binary_search(&g1.0.min(g2.0)).unwrap_err();
            let highest_empty_row_idx =
                image.empty_rows.binary_search(&g1.0.max(g2.0)).unwrap_err();
            let num_empty_rows_between =
                (highest_empty_row_idx - lowest_empty_row_idx) * (dist - 1);
            let lowest_empty_col_idx = image.empty_cols.binary_search(&g1.1.min(g2.1)).unwrap_err();
            let highest_empty_col_idx =
                image.empty_cols.binary_search(&g1.1.max(g2.1)).unwrap_err();
            let num_empty_cols_between =
                (highest_empty_col_idx - lowest_empty_col_idx) * (dist - 1);

            sum += (g1.0.max(g2.0) - g1.0.min(g2.0) + num_empty_rows_between)
                + (g1.1.max(g2.1) - g1.1.min(g2.1) + num_empty_cols_between);
        }
    }

    return sum / 2;
}

#[cfg(test)]
mod tests {
    use crate::aoc_2023::data::day_11::*;
    use crate::aoc_2023::day_11::*;

    #[test]
    fn part_1_sample() {
        let result = part_x(SAMPLE, 2);
        assert_eq!(result, 374);
    }

    #[test]
    fn part_1_input() {
        let result = part_x(INPUT, 2);
        assert_eq!(result, 9608724);
    }

    #[test]
    fn part_2_sample_1() {
        let result = part_x(SAMPLE, 10);
        assert_eq!(result, 1030);
    }

    #[test]
    fn part_2_sample_2() {
        let result = part_x(SAMPLE, 100);
        assert_eq!(result, 8410);
    }

    #[test]
    fn part_2_input() {
        let result = part_x(INPUT, 1000000);
        assert_eq!(result, 904633799472);
    }
}
