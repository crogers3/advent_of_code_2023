use std::collections::HashSet;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn move_pos(&self, (row_idx, col_idx): (usize, usize)) -> (usize, usize) {
        match self {
            Dir::Up => (row_idx - 1, col_idx),
            Dir::Down => (row_idx + 1, col_idx),
            Dir::Left => (row_idx, col_idx - 1),
            Dir::Right => (row_idx, col_idx + 1),
        }
    }
}

#[derive(Copy, Clone)]
enum Tile {
    NS,
    WE,
    NE,
    NW,
    SW,
    SE,
    G,
    S,
}

impl Tile {
    fn from_char(c: char) -> Tile {
        match c {
            '|' => Tile::NS,
            '-' => Tile::WE,
            'L' => Tile::NE,
            'J' => Tile::NW,
            '7' => Tile::SW,
            'F' => Tile::SE,
            '.' => Tile::G,
            'S' => Tile::S,
            _ => panic!(),
        }
    }

    fn next_dir(&self, into_dir: &Dir) -> Dir {
        match into_dir {
            Dir::Up => match self {
                Tile::NS => Dir::Up,
                Tile::SW => Dir::Left,
                Tile::SE => Dir::Right,
                _ => panic!(),
            },
            Dir::Down => match self {
                Tile::NS => Dir::Down,
                Tile::NE => Dir::Right,
                Tile::NW => Dir::Left,
                _ => panic!(),
            },
            Dir::Left => match self {
                Tile::WE => Dir::Left,
                Tile::NE => Dir::Up,
                Tile::SE => Dir::Down,
                _ => panic!(),
            },
            Dir::Right => match self {
                Tile::WE => Dir::Right,
                Tile::NW => Dir::Up,
                Tile::SW => Dir::Down,
                _ => panic!(),
            },
        }
    }
}

struct Map {
    tiles: Vec<Vec<Tile>>,
}

impl Map {
    fn parse_input(input: &str) -> Map {
        Map {
            tiles: input
                .lines()
                .map(|line| line.chars().map(Tile::from_char).collect::<Vec<Tile>>())
                .collect::<Vec<Vec<Tile>>>(),
        }
    }

    fn get(&self, (row_idx, col_idx): (usize, usize)) -> &Tile {
        &self.tiles[row_idx][col_idx]
    }

    fn start(&self) -> ((usize, usize), Dir, Tile) {
        let pos = self
            .tiles
            .iter()
            .map(|row| {
                row.iter()
                    .enumerate()
                    .find(|(_, t)| matches!(**t, Tile::S))
                    .map(|(col_idx, _)| col_idx)
            })
            .enumerate()
            .find(|(_, opt_col_idx)| opt_col_idx.is_some())
            .map(|(row_idx, opt_col_idx)| (row_idx, opt_col_idx.unwrap()))
            .unwrap();

        let mut potential_dirs = HashSet::new();
        if pos.0 > 0
            && matches!(
                self.get(Dir::Up.move_pos(pos)),
                Tile::SE | Tile::SW | Tile::NS
            )
        {
            potential_dirs.insert(Dir::Up);
        }
        if let Tile::NS | Tile::NE | Tile::NW = self.get(Dir::Down.move_pos(pos)) {
            potential_dirs.insert(Dir::Down);
        }
        if let Tile::WE | Tile::NW | Tile::SW = self.get(Dir::Right.move_pos(pos)) {
            potential_dirs.insert(Dir::Right);
        }
        if pos.1 > 0
            && matches!(
                self.get(Dir::Left.move_pos(pos)),
                Tile::WE | Tile::NE | Tile::SE
            )
        {
            potential_dirs.insert(Dir::Left);
        }
        assert_eq!(potential_dirs.len(), 2);
        if potential_dirs.contains(&Dir::Up) {
            if potential_dirs.contains(&Dir::Left) {
                return (pos, Dir::Up, Tile::NW);
            } else if potential_dirs.contains(&Dir::Right) {
                return (pos, Dir::Up, Tile::NE);
            } else if potential_dirs.contains(&Dir::Down) {
                return (pos, Dir::Up, Tile::NS);
            }
        } else if potential_dirs.contains(&Dir::Down) {
            if potential_dirs.contains(&Dir::Left) {
                return (pos, Dir::Down, Tile::SW);
            } else if potential_dirs.contains(&Dir::Right) {
                return (pos, Dir::Down, Tile::SE);
            }
        }
        return (pos, Dir::Left, Tile::WE);
    }

    fn traverse<F>(&self, mut f: F)
    where
        F: FnMut((usize, usize)),
    {
        let (start_pos, start_dir, _) = self.start();
        let (mut pos, mut into_dir) = (start_dir.move_pos(start_pos), start_dir);

        while pos != start_pos {
            f(pos);
            let tile = self.get(pos);
            let next_dir = tile.next_dir(&into_dir);
            let next_pos = next_dir.move_pos(pos);
            (pos, into_dir) = (next_pos, next_dir);
        }
    }
}

fn part_1(input: &str) -> usize {
    let map = Map::parse_input(input);
    let mut steps = 1;

    map.traverse(|_| steps += 1);

    return steps / 2;
}

enum State {
    Inside,
    InsideOnPipe(Dir),
    Outside,
    OutsideOnPipe(Dir),
}

fn part_2(input: &str) -> usize {
    let map = Map::parse_input(input);
    let (start_pos, _, start_tile) = map.start();

    // Mark loop
    let mut markings = Vec::new();
    for _ in 0..map.tiles.len() {
        let mut line = Vec::new();
        for _ in 0..map.tiles[0].len() {
            line.push(false);
        }
        markings.push(line);
    }
    markings[start_pos.0][start_pos.1] = true;
    map.traverse(|pos| markings[pos.0][pos.1] = true);

    // Find inner
    let mut inner_ground = 0;
    for (row_idx, line) in map.tiles.iter().enumerate() {
        let mut state = State::Outside;
        for (col_idx, tile) in line.iter().enumerate() {
            let tile = match tile {
                Tile::S => start_tile.clone(),
                _ => tile.clone(),
            };

            if markings[row_idx][col_idx] {
                state = match state {
                    State::Outside => match tile {
                        Tile::NE => State::OutsideOnPipe(Dir::Up),
                        Tile::SE => State::OutsideOnPipe(Dir::Down),
                        Tile::NS => State::Inside,
                        _ => panic!(),
                    },
                    State::OutsideOnPipe(d) => match tile {
                        Tile::WE => State::OutsideOnPipe(d),
                        Tile::SW => match d {
                            Dir::Up => State::Inside,
                            Dir::Down => State::Outside,
                            _ => panic!(),
                        },
                        Tile::NW => match d {
                            Dir::Up => State::Outside,
                            Dir::Down => State::Inside,
                            _ => panic!(),
                        },
                        _ => panic!(),
                    },
                    State::Inside => match tile {
                        Tile::NE => State::InsideOnPipe(Dir::Up),
                        Tile::SE => State::InsideOnPipe(Dir::Down),
                        Tile::NS => State::Outside,
                        _ => panic!(),
                    },
                    State::InsideOnPipe(d) => match tile {
                        Tile::WE => State::InsideOnPipe(d),
                        Tile::NW => match d {
                            Dir::Up => State::Inside,
                            Dir::Down => State::Outside,
                            _ => panic!(),
                        },
                        Tile::SW => match d {
                            Dir::Up => State::Outside,
                            Dir::Down => State::Inside,
                            _ => panic!(),
                        },
                        _ => panic!(),
                    },
                };
            } else {
                if matches!(state, State::Inside) {
                    inner_ground += 1;
                }
            }
        }
    }

    return inner_ground;
}

#[cfg(test)]
mod tests {
    use crate::aoc_2023::data::day_10::*;
    use crate::aoc_2023::day_10::*;

    #[test]
    fn part_1_sample_1() {
        let result = part_1(SAMPLE_1);
        assert_eq!(result, 4);
    }

    #[test]
    fn part_1_sample_2() {
        let result = part_1(SAMPLE_2);
        assert_eq!(result, 8);
    }

    #[test]
    fn part_1_input() {
        let result = part_1(INPUT);
        assert_eq!(result, 6947);
    }

    #[test]
    fn part_2_sample_3() {
        let result = part_2(SAMPLE_3);
        assert_eq!(result, 4);
    }

    #[test]
    fn part_2_sample_4() {
        let result = part_2(SAMPLE_4);
        assert_eq!(result, 8);
    }

    #[test]
    fn part_2_sample_5() {
        let result = part_2(SAMPLE_5);
        assert_eq!(result, 10);
    }

    #[test]
    fn part_2_input() {
        let result = part_2(INPUT);
        dbg!(result);
    }
}
