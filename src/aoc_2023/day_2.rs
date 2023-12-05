use std::cmp::max;

#[derive(Eq, PartialEq, Debug)]
struct Game {
    number: usize,
    hands: Vec<Hand>,
}

impl Game {
    fn new(number: usize) -> Self {
        Game {
            number,
            hands: Vec::new(),
        }
    }

    fn all_hands_possible(&self, max: &Hand) -> bool {
        self.hands.iter().all(|hand| hand.is_possible(max))
    }

    fn minimal_hand(&self) -> Hand {
        let mut minimal_hand = Hand::new();
        for hand in self.hands.iter() {
            minimal_hand.red = max(minimal_hand.red, hand.red);
            minimal_hand.green = max(minimal_hand.green, hand.green);
            minimal_hand.blue = max(minimal_hand.blue, hand.blue);
        }
        return minimal_hand;
    }
}

#[derive(Eq, PartialEq, Debug)]
struct Hand {
    red: usize,
    green: usize,
    blue: usize,
}

impl Hand {
    fn new() -> Self {
        Hand {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    fn is_possible(&self, max: &Hand) -> bool {
        self.red <= max.red && self.blue <= max.blue && self.green <= max.green
    }

    fn power(&self) -> usize {
        self.red * self.green * self.blue
    }
}

fn parse_line(line: &str) -> Game {
    let (game, hands) = line.split_once(": ").unwrap();
    let number = game.split_once(" ").unwrap().1.parse::<usize>().unwrap();
    let mut game = Game::new(number);

    let hands = hands.split("; ");
    for hand in hands {
        let cubes = hand.split(", ");
        let mut hand = Hand::new();
        for cube in cubes {
            let (count, color) = cube.split_once(" ").unwrap();
            let count = count.parse::<usize>().unwrap();
            match color {
                "blue" => hand.blue += count,
                "red" => hand.red += count,
                "green" => hand.green += count,
                _ => panic!(),
            }
        }
        game.hands.push(hand);
    }

    return game;
}

const MAX: Hand = Hand {
    red: 12,
    green: 13,
    blue: 14,
};

fn part_1(input: &str) -> usize {
    input
        .lines()
        .map(parse_line)
        .filter(|game| game.all_hands_possible(&MAX))
        .map(|game| game.number)
        .sum()
}

fn part_2(input: &str) -> usize {
    input
        .lines()
        .map(parse_line)
        .map(|game| game.minimal_hand())
        .map(|hand| hand.power())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::super::data::day_2::*;
    use super::*;

    #[test]
    fn parse() {
        let line = SAMPLE_1.lines().next().unwrap();
        let game = parse_line(line);
        assert_eq!(
            game,
            Game {
                number: 1,
                hands: vec![
                    Hand {
                        blue: 3,
                        red: 4,
                        green: 0
                    },
                    Hand {
                        red: 1,
                        green: 2,
                        blue: 6
                    },
                    Hand {
                        green: 2,
                        red: 0,
                        blue: 0
                    }
                ]
            }
        );
    }

    #[test]
    fn minimal_hand() {
        let minimal_hands = SAMPLE_1
            .lines()
            .map(parse_line)
            .map(|game| game.minimal_hand())
            .collect::<Vec<Hand>>();

        assert_eq!(
            minimal_hands,
            vec![
                Hand {
                    red: 4,
                    green: 2,
                    blue: 6
                },
                Hand {
                    red: 1,
                    green: 3,
                    blue: 4
                },
                Hand {
                    red: 20,
                    green: 13,
                    blue: 6
                },
                Hand {
                    red: 14,
                    green: 3,
                    blue: 15
                },
                Hand {
                    red: 6,
                    green: 3,
                    blue: 2
                },
            ]
        );
    }

    #[test]
    fn power() {
        let powers = SAMPLE_1
            .lines()
            .map(parse_line)
            .map(|g| g.minimal_hand())
            .map(|h| h.power())
            .collect::<Vec<usize>>();
        assert_eq!(powers, vec![48, 12, 1560, 630, 36]);
    }

    #[test]
    fn part_1_sample() {
        let result = part_1(SAMPLE_1);
        assert_eq!(result, 8);
    }

    #[test]
    fn part_1_input() {
        let result = part_1(INPUT);
        assert_eq!(result, 2164);
    }

    #[test]
    fn part_2_sample() {
        let result = part_2(SAMPLE_1);
        assert_eq!(result, 2286);
    }

    #[test]
    fn part_2_input() {
        let result = part_2(INPUT);
        assert_eq!(result, 69929);
    }
}
