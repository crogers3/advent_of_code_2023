enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn move_pos(&self, (col_idx, row_idx): (usize, usize)) -> (usize, usize) {
        match self {
            Dir::Up => (col_idx - 1, row_idx),
            Dir::Down => (col_idx + 1, row_idx),
            Dir::Left => (col_idx, row_idx - 1),
            Dir::Right => (col_idx, row_idx + 1),
        }
    }
}

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

    fn next_dir(&self, into_dir: Dir) -> Dir {
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

    fn get(&self, (col_idx, row_idx): (usize, usize)) -> &Tile {
        &self.tiles[col_idx][row_idx]
    }

    fn start(&self) -> ((usize, usize), Dir) {
        let pos = self.tiles
            .iter()
            .map(|row| {
                row.iter()
                    .enumerate()
                    .find(|(_, t)| matches!(**t, Tile::S))
                    .map(|(idx, _)| idx)
            })
            .enumerate()
            .find(|(_, opt_row_idx)| opt_row_idx.is_some())
            .map(|(col_idx, opt_row_idx)| (col_idx, opt_row_idx.unwrap()))
            .unwrap();

        if let Tile::SE | Tile::SW | Tile::NS = self.get(Dir::Up.move_pos(pos)) {
            return (pos, Dir::Up);
        }
        if let Tile::NS | Tile::NE | Tile::NW = self.get(Dir::Down.move_pos(pos)) {
            return (pos, Dir::Down);
        }
        if let Tile::WE | Tile::NW | Tile::SW = self.get(Dir::Right.move_pos(pos)) {
            return (pos, Dir::Right);
        }
        if let Tile::WE | Tile::NE | Tile::SE = self.get(Dir::Left.move_pos(pos)) {
            return (pos, Dir::Left);
        }
        panic!()
    }

    fn step(&self, (col_idx, row_idx): (usize, usize), into_dir: Dir) -> usize {
        let tile = &self.tiles[col_idx][row_idx];
        if matches!(tile, Tile::S) {
            return 0;
        }

        let next_dir = tile.next_dir(into_dir);
        let next_pos = next_dir.move_pos((col_idx, row_idx));

        return 1 + self.step(next_pos, next_dir);
    }
}

fn part_1(input: &str) -> usize {
    let map = Map::parse_input(input);
    let (start_pos, start_dir) = map.start();

    return map.step(start_dir.move_pos(start_pos), start_dir) / 2;
}

fn part_2(input: &str) -> usize {
    todo!();
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
        dbg!(result);
    }

    #[test]
    fn part_2_sample_1() {
        let result = part_2(SAMPLE_1);
        assert_eq!(result, todo!());
    }

    #[test]
    fn part_2_sample_2() {
        let result = part_2(SAMPLE_2);
        assert_eq!(result, todo!());
    }

    #[test]
    fn part_2_input() {
        let result = part_2(INPUT);
        dbg!(result);
    }
}
