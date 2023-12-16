use std::collections::HashSet;

enum Space {
    Empty,
    MirrorFwd,
    MirrorBck,
    SplitterHor,
    SplitterVert,
}

impl Space {
    fn from_char(c: char) -> Space {
        match c {
            '.' => Space::Empty,
            '/' => Space::MirrorFwd,
            '\\' => Space::MirrorBck,
            '-' => Space::SplitterHor,
            '|' => Space::SplitterVert,
            _ => panic!(),
        }
    }
}

struct Layout {
    spaces: Vec<Vec<Space>>,
}

impl Layout {
    fn from_input(input: &str) -> Layout {
        Layout {
            spaces: input
                .lines()
                .map(|line| line.chars().map(Space::from_char).collect::<Vec<Space>>())
                .collect::<Vec<Vec<Space>>>(),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn step(&self, pos: &Pos, layout: &Layout) -> Option<Pos> {
        match self {
            Dir::Up => {
                if pos.0 > 0 {
                    Some((pos.0 - 1, pos.1, Dir::Up))
                } else {
                    None
                }
            }
            Dir::Down => {
                if pos.0 < layout.spaces.len() - 1 {
                    Some((pos.0 + 1, pos.1, Dir::Down))
                } else {
                    None
                }
            }
            Dir::Left => {
                if pos.1 > 0 {
                    Some((pos.0, pos.1 - 1, Dir::Left))
                } else {
                    None
                }
            }
            Dir::Right => {
                if pos.1 < layout.spaces[0].len() - 1 {
                    Some((pos.0, pos.1 + 1, Dir::Right))
                } else {
                    None
                }
            }
        }
    }
}

type Pos = (usize, usize, Dir);

fn step(start: &Pos, layout: &Layout) -> Vec<Pos> {
    let next_pos = match layout.spaces[start.0][start.1] {
        Space::Empty => vec![start.2.step(start, layout)],
        Space::MirrorFwd => match start.2 {
            Dir::Up => vec![Dir::Right.step(start, layout)],
            Dir::Down => vec![Dir::Left.step(start, layout)],
            Dir::Right => vec![Dir::Up.step(start, layout)],
            Dir::Left => vec![Dir::Down.step(start, layout)],
        },
        Space::MirrorBck => match start.2 {
            Dir::Up => vec![Dir::Left.step(start, layout)],
            Dir::Down => vec![Dir::Right.step(start, layout)],
            Dir::Right => vec![Dir::Down.step(start, layout)],
            Dir::Left => vec![Dir::Up.step(start, layout)],
        },
        Space::SplitterVert => match start.2 {
            Dir::Up | Dir::Down => vec![start.2.step(start, layout)],
            Dir::Right | Dir::Left => {
                vec![Dir::Up.step(start, layout), Dir::Down.step(start, layout)]
            }
        },
        Space::SplitterHor => match start.2 {
            Dir::Right | Dir::Left => vec![start.2.step(start, layout)],
            Dir::Up | Dir::Down => vec![
                Dir::Left.step(start, layout),
                Dir::Right.step(start, layout),
            ],
        },
    };
    return next_pos
        .into_iter()
        .filter_map(|pos| pos)
        .collect::<Vec<Pos>>();
}

fn num_tiles(layout: &Layout, start: &Pos) -> usize {
    let mut visited = HashSet::new();
    let mut next = vec![start.clone()];

    while !next.is_empty() {
        let pos = next.pop().unwrap();
        visited.insert(pos.clone());
        let next_pos = step(&pos, &layout);
        for p in next_pos.iter() {
            if !visited.contains(p) {
                next.push(p.clone());
            }
        }
    }

    return visited
        .iter()
        .map(|pos| (pos.0, pos.1))
        .collect::<HashSet<(usize, usize)>>()
        .len();
}

fn part_1(input: &str) -> usize {
    let layout = Layout::from_input(input);
    num_tiles(&layout, &(0, 0, Dir::Right))
}

fn part_2(input: &str) -> usize {
    let layout = Layout::from_input(input);
    let mut start = Vec::new();

    for row in 0..layout.spaces.len() {
        start.push((row, 0, Dir::Right));
        start.push((row, layout.spaces[0].len() - 1, Dir::Left));
    }

    for col in 0..layout.spaces[0].len() {
        start.push((0, col, Dir::Down));
        start.push((layout.spaces.len() - 1, col, Dir::Up));
    }

    start.iter().map(|s| num_tiles(&layout, s)).max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc_2023::data::day_16::*;

    #[test]
    fn part_1_sample() {
        let input = read_input(InputFile::SAMPLE);
        let result = part_1(&input);
        assert_eq!(result, 46);
    }

    #[test]
    fn part_1_input() {
        let input = read_input(InputFile::INPUT);
        let result = part_1(&input);
        dbg!(result);
    }

    #[test]
    fn part_2_sample() {
        let input = read_input(InputFile::SAMPLE);
        let result = part_2(&input);
        assert_eq!(result, 51);
    }

    #[test]
    fn part_2_input() {
        let input = read_input(InputFile::INPUT);
        let result = part_2(&input);
        assert_eq!(result, 8225);
    }
}
