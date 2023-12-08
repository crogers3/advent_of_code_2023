use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone)]
enum Type {
    HIGH,
    ONE,
    TWO,
    THREE,
    FULL,
    FOUR,
    FIVE,
}

impl Type {
    fn from_cards(cards: &[Card; 5]) -> Type {
        let mut freq: HashMap<&Card, usize> = HashMap::new();
        cards.iter().for_each(|c| *freq.entry(c).or_insert(0) += 1);

        match *freq.values().max().unwrap() {
            5 => Type::FIVE,
            4 => Type::FOUR,
            3 => match freq.into_values().find(|f| *f == 2) {
                Some(_) => Type::FULL,
                None => Type::THREE,
            },
            2 => match freq.into_values().filter(|f| *f == 2).count() {
                2 => Type::TWO,
                _ => Type::ONE,
            },
            _ => Type::HIGH,
        }
    }

    fn apply_jokers(self, n_jokers: usize) -> Type {
        if n_jokers == 0 {
            return self;
        }
        match self {
            Type::FIVE => match n_jokers {
                5 => Type::FIVE,
                _ => panic!(),
            },
            Type::FOUR => match n_jokers {
                4 | 1 => Type::FIVE,
                _ => panic!(),
            },
            Type::FULL => match n_jokers {
                3 | 2 => Type::FIVE,
                _ => panic!(),
            },
            Type::THREE => match n_jokers {
                3 | 1 => Type::FOUR,
                _ => panic!(),
            },
            Type::TWO => match n_jokers {
                2 => Type::FOUR,
                1 => Type::FULL,
                _ => panic!(),
            },
            Type::ONE => match n_jokers {
                2 | 1 => Type::THREE,
                _ => panic!(),
            },
            Type::HIGH => Type::ONE,
        }
    }

    fn from_cards_2(cards: &[Card2; 5]) -> Type {
        let cards_1: [Card; 5] = cards
            .into_iter()
            .map(|c| Card::from_card_2(*c))
            .collect::<Vec<Card>>()
            .try_into()
            .unwrap();
        let n_jokers = cards.iter().filter(|c| **c == Card2::J).count();
        return Type::from_cards(&cards_1).apply_jokers(n_jokers);
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
enum Card {
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    T,
    J,
    Q,
    K,
    A,
}

impl Card {
    fn from_char(c: char) -> Card {
        match c {
            '2' => Card::C2,
            '3' => Card::C3,
            '4' => Card::C4,
            '5' => Card::C5,
            '6' => Card::C6,
            '7' => Card::C7,
            '8' => Card::C8,
            '9' => Card::C9,
            'T' => Card::T,
            'J' => Card::J,
            'Q' => Card::Q,
            'K' => Card::K,
            'A' => Card::A,
            _ => panic!(),
        }
    }

    fn from_card_2(c: Card2) -> Card {
        match c {
            Card2::C2 => Card::C2,
            Card2::C3 => Card::C3,
            Card2::C4 => Card::C4,
            Card2::C5 => Card::C5,
            Card2::C6 => Card::C6,
            Card2::C7 => Card::C7,
            Card2::C8 => Card::C8,
            Card2::C9 => Card::C9,
            Card2::T => Card::T,
            Card2::J => Card::J,
            Card2::Q => Card::Q,
            Card2::K => Card::K,
            Card2::A => Card::A,
        }
    }
}

#[derive(Eq, Debug)]
struct Hand {
    hand_type: Type,
    cards: [Card; 5],
    bid: usize,
}

impl Hand {
    fn parse_input(input: &str) -> Hand {
        let (cards, bid) = input.split_once(' ').unwrap();
        let cards: [Card; 5] = cards
            .chars()
            .into_iter()
            .map(Card::from_char)
            .collect::<Vec<Card>>()
            .try_into()
            .unwrap();
        let hand_type = Type::from_cards(&cards);
        return Hand {
            hand_type,
            cards,
            bid: bid.parse::<usize>().unwrap(),
        };
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => self
                .cards
                .iter()
                .zip(other.cards.iter())
                .find_map(|(x, y)| match x.cmp(y) {
                    Ordering::Less => Some(Ordering::Less),
                    Ordering::Greater => Some(Ordering::Greater),
                    Ordering::Equal => None,
                })
                .unwrap(),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Hash, Copy, Clone)]
enum Card2 {
    J,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    T,
    Q,
    K,
    A,
}

impl Card2 {
    fn from_char(c: char) -> Card2 {
        match c {
            '2' => Card2::C2,
            '3' => Card2::C3,
            '4' => Card2::C4,
            '5' => Card2::C5,
            '6' => Card2::C6,
            '7' => Card2::C7,
            '8' => Card2::C8,
            '9' => Card2::C9,
            'T' => Card2::T,
            'J' => Card2::J,
            'Q' => Card2::Q,
            'K' => Card2::K,
            'A' => Card2::A,
            _ => panic!(),
        }
    }
}

#[derive(Eq, Debug)]
struct Hand2 {
    hand_type: Type,
    cards: [Card2; 5],
    bid: usize,
}

impl Hand2 {
    fn parse_input(input: &str) -> Hand2 {
        let (cards, bid) = input.split_once(' ').unwrap();
        let cards: [Card2; 5] = cards
            .chars()
            .into_iter()
            .map(Card2::from_char)
            .collect::<Vec<Card2>>()
            .try_into()
            .unwrap();
        let hand_type = Type::from_cards_2(&cards);
        return Hand2 {
            hand_type,
            cards,
            bid: bid.parse::<usize>().unwrap(),
        };
    }
}

impl Ord for Hand2 {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => self
                .cards
                .iter()
                .zip(other.cards.iter())
                .find_map(|(x, y)| match x.cmp(y) {
                    Ordering::Less => Some(Ordering::Less),
                    Ordering::Greater => Some(Ordering::Greater),
                    Ordering::Equal => None,
                })
                .unwrap(),
        }
    }
}

impl PartialOrd for Hand2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand2 {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

fn part_1(input: &str) -> usize {
    let mut hands = input.lines().map(Hand::parse_input).collect::<Vec<Hand>>();
    hands.sort();
    return hands
        .iter()
        .enumerate()
        .map(|(idx, hand)| hand.bid * (idx + 1))
        .sum();
}

fn part_2(input: &str) -> usize {
    let mut hands = input
        .lines()
        .map(Hand2::parse_input)
        .collect::<Vec<Hand2>>();
    hands.sort();
    return hands
        .iter()
        .enumerate()
        .map(|(idx, hand)| hand.bid * (idx + 1))
        .sum();
}

#[cfg(test)]
mod tests {
    use crate::aoc_2023::data::day_7::*;
    use crate::aoc_2023::day_7::*;

    #[test]
    fn part_1_sample() {
        let result = part_1(SAMPLE);
        assert_eq!(result, 6440);
    }

    #[test]
    fn part_1_input() {
        let result = part_1(INPUT);
        assert_eq!(result, 250347426);
    }

    #[test]
    fn part_2_sample() {
        let result = part_2(SAMPLE);
        assert_eq!(result, 5905);
    }

    #[test]
    fn part_2_sample_2() {
        let result = part_2(SAMPLE_2);
        assert_eq!(result, 6839);
    }

    #[test]
    fn part_2_input() {
        let result = part_2(INPUT);
        assert_eq!(result, 251224870);
    }
}
