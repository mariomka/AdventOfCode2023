use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap};

const CARD_STRENGTH: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

const CARD_STRENGTH_WITH_JOKER: [char; 13] = [
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
];

#[derive(PartialEq, PartialOrd, Debug)]
enum HandType {
    HighCard = 1,
    OnePair = 2,
    TwoPair = 3,
    ThreeOfAKind = 4,
    FullHouse = 5,
    FourOfAKind = 6,
    FiveOfAKind = 7,
}
impl HandType {
    fn from_cards(cards: &Vec<char>, with_joker: bool) -> HandType {
        let mut card_counts = HashMap::new();

        for card in cards {
            card_counts
                .entry(*card)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        let joker_count = if with_joker {
            card_counts.remove(&'J').unwrap_or(0)
        } else {
            0
        };

        let mut pair_count = 0;
        let mut three_of_a_kind = false;
        let mut four_of_a_kind = false;
        let mut five_of_a_kind = false;

        for (_, &count) in card_counts.iter() {
            if count == 2 {
                pair_count += 1;
                continue;
            }

            if count == 3 {
                three_of_a_kind = true;
                continue;
            }

            if count == 4 {
                four_of_a_kind = true;
                break;
            }

            if count == 5 {
                five_of_a_kind = true;
                break;
            }
        }

        if five_of_a_kind {
            return HandType::FiveOfAKind;
        }

        if four_of_a_kind {
            if joker_count == 1 {
                return HandType::FiveOfAKind;
            }
            return HandType::FourOfAKind;
        }

        if three_of_a_kind {
            if pair_count == 1 {
                return HandType::FullHouse;
            }

            if joker_count == 1 {
                return HandType::FourOfAKind;
            }

            if joker_count == 2 {
                return HandType::FiveOfAKind;
            }
        }

        if three_of_a_kind {
            if joker_count == 1 {
                return HandType::FourOfAKind;
            }

            if joker_count == 2 {
                return HandType::FiveOfAKind;
            }

            return HandType::ThreeOfAKind;
        }

        if pair_count == 2 {
            if joker_count == 1 {
                return HandType::FullHouse;
            }

            return HandType::TwoPair;
        }

        if pair_count == 1 {
            if joker_count == 1 {
                return HandType::ThreeOfAKind;
            }

            if joker_count == 2 {
                return HandType::FourOfAKind;
            }

            if joker_count == 3 {
                return HandType::FiveOfAKind;
            }

            return HandType::OnePair;
        }

        if joker_count == 1 {
            return HandType::OnePair;
        }

        if joker_count == 2 {
            return HandType::ThreeOfAKind;
        }

        if joker_count == 3 {
            return HandType::FourOfAKind;
        }

        if joker_count == 4 || joker_count == 5 {
            return HandType::FiveOfAKind;
        }

        HandType::HighCard
    }
}

#[derive(Debug)]
struct Hand {
    cards: Vec<char>,
    hand_type: HandType,
    bid: usize,
    with_joker: bool,
}

impl Hand {
    fn new(cards: Vec<char>, bid: usize, with_joker: bool) -> Hand {
        Hand {
            cards: cards.clone(),
            hand_type: HandType::from_cards(&cards, with_joker),
            bid,
            with_joker,
        }
    }
}

impl Eq for Hand {}

impl PartialEq<Self> for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.hand_type > other.hand_type {
            return Some(Ordering::Greater);
        }

        if self.hand_type < other.hand_type {
            return Some(Ordering::Less);
        }

        for (card, other_card) in self.cards.iter().zip(other.cards.iter()) {
            let cards = if self.with_joker {
                CARD_STRENGTH_WITH_JOKER
            } else {
                CARD_STRENGTH
            };

            let card_position = cards.iter().position(|&c| c == *card).unwrap();
            let other_card_position = cards.iter().position(|&c| c == *other_card).unwrap();

            if card_position > other_card_position {
                return Some(Ordering::Greater);
            }

            if card_position < other_card_position {
                return Some(Ordering::Less);
            }
        }

        Some(Ordering::Equal)
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn calc_winnings(input: &Vec<&str>, with_joker: bool) -> usize {
    input
        .iter()
        .map(|&line| {
            let (hand, bid) = line.split_once(' ').unwrap();

            Hand::new(hand.chars().collect(), bid.parse().unwrap(), with_joker)
        })
        .collect::<BTreeSet<Hand>>()
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.bid)
        .sum()
}

pub fn part1(input: &Vec<&str>) -> usize {
    calc_winnings(input, false)
}

pub fn part2(input: &Vec<&str>) -> usize {
    calc_winnings(input, true)
}

#[cfg(test)]
mod tests {
    use helpers::input_lines;

    use super::*;

    fn input<'a>() -> Vec<&'a str> {
        let input = "
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        input_lines(input)
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            HandType::FiveOfAKind.partial_cmp(&HandType::FourOfAKind),
            Some(Ordering::Greater)
        );

        assert_eq!(part1(&input()), 6440)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 5905)
    }
}
