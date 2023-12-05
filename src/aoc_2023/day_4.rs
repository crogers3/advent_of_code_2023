use std::collections::HashSet;

struct Card {
    winning_numbers: HashSet<usize>,
    numbers: HashSet<usize>,
    num_copies: usize,
}

impl Card {
    fn from_line(line: &str) -> Card {
        let (_, remainder) = line
            .strip_prefix("Card")
            .map(str::trim)
            .map(|s| s.split_once(':'))
            .flatten()
            .unwrap();
        let (winning_numbers, numbers) = remainder.split_once('|').unwrap();

        return Card {
            winning_numbers: winning_numbers
                .split_whitespace()
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<HashSet<usize>>(),
            numbers: numbers
                .split_whitespace()
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<HashSet<usize>>(),
            num_copies: 1,
        };
    }

    fn num_matches(&self) -> usize {
        self.winning_numbers.intersection(&self.numbers).count()
    }

    fn point_value(&self) -> usize {
        return match self.num_matches() {
            0 => 0,
            1 => 1,
            n => 2 << (n - 2),
        };
    }
}

fn parse_cards(input: &str) -> Vec<Card> {
    input.lines().map(Card::from_line).collect::<Vec<Card>>()
}

fn part_1(input: &str) -> usize {
    parse_cards(input).iter().map(Card::point_value).sum()
}

fn part_2(input: &str) -> usize {
    let mut cards = parse_cards(input);

    for i in 0..cards.len() {
        let num_matches = cards[i].num_matches();
        let num_copies = cards[i].num_copies;

        for j in i + 1..=i + num_matches {
            cards[j].num_copies += num_copies;
        }
    }

    return cards.iter().map(|c| c.num_copies).sum();
}

#[cfg(test)]
mod tests {
    use super::super::data::day_4::*;
    use super::*;

    #[test]
    fn part_1_sample() {
        let result = part_1(SAMPLE);
        assert_eq!(result, 13);
    }

    #[test]
    fn part_1_input() {
        let result = part_1(INPUT);
        assert_eq!(result, 23678);
    }

    #[test]
    fn part_2_sample() {
        let result = part_2(SAMPLE);
        assert_eq!(result, 30);
    }

    #[test]
    fn part_2_input() {
        let result = part_2(INPUT);
        assert_eq!(result, 15455663);
    }
}
