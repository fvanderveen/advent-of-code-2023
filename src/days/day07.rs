use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::days::Day;
use crate::util::number::parse_u8;
use crate::util::parser::Parser;

pub const DAY7: Day = Day {
    puzzle1,
    puzzle2,
};

fn puzzle1(input: &String) {
    let hands = input.lines().map(|l| l.parse::<Hand>()).collect::<Result<Vec<_>, _>>().unwrap();

    println!("Winnings in puzzle 1: {}", get_winnings(&hands));
}

fn puzzle2(input: &String) {
    let hands = input.lines().map(|l| l.parse::<Hand2>()).collect::<Result<Vec<_>, _>>().unwrap();

    println!("Winnings in puzzle 2: {}", get_winnings2(&hands));
}

fn get_winnings(hands: &Vec<Hand>) -> usize {
    let mut winnings = 0;
    let mut sorted = hands.clone();
    sorted.sort();

    for i in 0..sorted.len() {
        winnings += sorted[i].bid * (i + 1);
    }

    winnings
}

fn get_winnings2(hands: &Vec<Hand2>) -> usize {
    let mut winnings = 0;
    let mut sorted = hands.clone();
    sorted.sort();

    for i in 0..sorted.len() {
        winnings += sorted[i].bid * (i + 1);
    }

    winnings
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
struct Hand {
    cards: [u8; 5],
    bid: usize,
}

impl Hand {
    fn get_kind(&self) -> HandKind {
        let mut map: HashMap<u8, usize> = HashMap::new();
        for card in self.cards {
            let current = map.get(&card).unwrap_or(&0);
            map.insert(card, current + 1);
        }

        // Check number of entries on map
        Self::get_kind_from_map(&mut map)
    }

    fn get_kind_from_map(map: &HashMap<u8, usize>) -> HandKind {
        match map.len() {
            1 => HandKind::FiveOfAKind, // Can only be five of the same
            2 => {
                if let [a, b] = map.values().collect::<Vec<_>>()[..2] {
                    match (a, b) {
                        (4, 1) | (1, 4) => HandKind::FourOfAKind,
                        (2, 3) | (3, 2) => HandKind::FullHouse,
                        _ => panic!("Invalid combo ({},{})", a, b)
                    }
                } else {
                    panic!("Should not happen?!");
                }
            }
            3 => {
                if let [a, b, c] = map.values().collect::<Vec<_>>()[..3] {
                    match (a, b, c) {
                        (1, 1, 3) | (1, 3, 1) | (3, 1, 1) => HandKind::ThreeOfAKind,
                        (1, 2, 2) | (2, 1, 2) | (2, 2, 1) => HandKind::TwoPair,
                        _ => panic!("Invalid combo ({}, {}, {})", a, b, c)
                    }
                } else {
                    panic!("Should not happen?!");
                }
            }
            4 => HandKind::Pair, // Can only be a single pair
            _ => HandKind::Garbage
        }
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
struct Hand2 {
    cards: [u8; 5],
    bid: usize,
}

impl Hand2 {
    fn get_kind(&self) -> HandKind {
        // Need a smarter way, as a joker (card value 1) can fit any slot.
        // First know the amount of jokers (that opens or closes a lot of info)
        // Partition other numbers into the map as before
        let (jokers, cards): (Vec<_>, Vec<_>) = self.cards.into_iter().partition(|c| 1.eq(c));
        let mut map: HashMap<u8, usize> = HashMap::new();
        for card in cards {
            map.insert(card, map.get(&card).unwrap_or(&0) + 1);
        }

        match jokers.len() {
            5 | 4 => HandKind::FiveOfAKind, // 5 jokers, or 4 jokers + whatever
            3 if map.len() == 1 => HandKind::FiveOfAKind, // 3 jokers + 2 of the same card
            2 if map.len() == 1 => HandKind::FiveOfAKind, // 2 jokers + 3 same cards
            1 if map.len() == 1 => HandKind::FiveOfAKind, // 1 joker + 4 same cards
            3 => HandKind::FourOfAKind, // 3 jokers and 2 random cards, four of a kind is the highest score.
            2 if map.len() == 2 => HandKind::FourOfAKind, // 2 jokers, 2 same cards, and a random card
            2 if map.len() == 3 => HandKind::ThreeOfAKind, // 2 jokers and 3 random cards
            1 if map.len() == 2 => {
                // Need to check:
                if let Some(value) = map.values().next() {
                    match value {
                        // - J + XX / YY => Full House
                        2 => HandKind::FullHouse,
                        // - J + XXX / Y => Four of a Kind
                        1 | 3 => HandKind::FourOfAKind,
                        _ => panic!("Wrong number of cards {}", value)
                    }
                } else {
                    panic!("Should not happen?!");
                }
            }
            1 if map.len() == 3 => HandKind::ThreeOfAKind, // 1 joker, with 2 cards, and 2 random cards (3 > 2+2)
            1 if map.len() == 4 => HandKind::Pair, // 1 joker, and four random cards
            0 => Hand::get_kind_from_map(&map), // 0 jokers, fall back to normal behaviour
            wrong => panic!("Cannot have {} jokers?!", wrong)
        }
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum HandKind {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    Pair,
    Garbage,
}

#[cfg(test)]
mod tests {
    use crate::days::day07::{get_winnings, Hand, Hand2, HandKind};

    #[test]
    fn test_hand_from_str() {
        assert_eq!("32T3K 765".parse::<Hand>(), Ok(Hand {
            cards: [3, 2, 10, 3, 13],
            bid: 765,
        }))
    }

    #[test]
    fn test_hand_display() {
        assert_eq!(format!("{}", Hand { cards: [2, 4, 10, 13, 14], bid: 42 }), "24TKA 42".to_string())
    }

    #[test]
    fn test_hand_get_kind() {
        assert_eq!(Hand { cards: [3, 3, 3, 3, 3], bid: 0 }.get_kind(), HandKind::FiveOfAKind);
        assert_eq!(Hand { cards: [3, 3, 2, 3, 3], bid: 0 }.get_kind(), HandKind::FourOfAKind);
        assert_eq!(Hand { cards: [3, 4, 3, 3, 4], bid: 0 }.get_kind(), HandKind::FullHouse);
        assert_eq!(Hand { cards: [4, 4, 2, 3, 4], bid: 0 }.get_kind(), HandKind::ThreeOfAKind);
        assert_eq!(Hand { cards: [4, 4, 2, 3, 2], bid: 0 }.get_kind(), HandKind::TwoPair);
        assert_eq!(Hand { cards: [4, 4, 6, 3, 2], bid: 0 }.get_kind(), HandKind::Pair);
        assert_eq!(Hand { cards: [4, 8, 6, 3, 2], bid: 0 }.get_kind(), HandKind::Garbage);
    }

    #[test]
    fn test_hand2_get_kind() {
        // Without any jokers:
        assert_eq!(Hand2 { cards: [3, 3, 3, 3, 3], bid: 0 }.get_kind(), HandKind::FiveOfAKind);
        assert_eq!(Hand2 { cards: [3, 3, 2, 3, 3], bid: 0 }.get_kind(), HandKind::FourOfAKind);
        assert_eq!(Hand2 { cards: [3, 4, 3, 3, 4], bid: 0 }.get_kind(), HandKind::FullHouse);
        assert_eq!(Hand2 { cards: [4, 4, 2, 3, 4], bid: 0 }.get_kind(), HandKind::ThreeOfAKind);
        assert_eq!(Hand2 { cards: [4, 4, 2, 3, 2], bid: 0 }.get_kind(), HandKind::TwoPair);
        assert_eq!(Hand2 { cards: [4, 4, 6, 3, 2], bid: 0 }.get_kind(), HandKind::Pair);
        assert_eq!(Hand2 { cards: [4, 8, 6, 3, 2], bid: 0 }.get_kind(), HandKind::Garbage);

        // With jokers:
        assert_eq!(Hand2 { cards: [1, 1, 1, 1, 1], bid: 0 }.get_kind(), HandKind::FiveOfAKind);
        assert_eq!(Hand2 { cards: [1, 1, 1, 1, 2], bid: 0 }.get_kind(), HandKind::FiveOfAKind);
        assert_eq!(Hand2 { cards: [1, 1, 1, 2, 2], bid: 0 }.get_kind(), HandKind::FiveOfAKind);
        assert_eq!(Hand2 { cards: [1, 1, 2, 2, 2], bid: 0 }.get_kind(), HandKind::FiveOfAKind);
        assert_eq!(Hand2 { cards: [1, 2, 2, 2, 2], bid: 0 }.get_kind(), HandKind::FiveOfAKind);
        assert_eq!(Hand2 { cards: [1, 1, 1, 2, 3], bid: 0 }.get_kind(), HandKind::FourOfAKind);
        assert_eq!(Hand2 { cards: [1, 1, 2, 2, 3], bid: 0 }.get_kind(), HandKind::FourOfAKind);
        assert_eq!(Hand2 { cards: [1, 1, 4, 2, 3], bid: 0 }.get_kind(), HandKind::ThreeOfAKind);
        assert_eq!(Hand2 { cards: [1, 4, 4, 2, 3], bid: 0 }.get_kind(), HandKind::ThreeOfAKind);
        assert_eq!(Hand2 { cards: [1, 4, 4, 3, 3], bid: 0 }.get_kind(), HandKind::FullHouse);
        assert_eq!(Hand2 { cards: [1, 6, 4, 2, 3], bid: 0 }.get_kind(), HandKind::Pair);
    }

    #[test]
    fn test_sort_test_input() {
        let hands = TEST_INPUT.lines().map(|l| l.parse::<Hand>()).collect::<Result<Vec<_>, _>>().unwrap();

        let mut sorted = hands.clone();
        sorted.sort();

        assert_eq!(sorted, vec![
            hands[0], // 32T3K - Pair
            hands[3], // KTJJT - Two pair [K, T]
            hands[2], // KK677 - Two pair [K, K]
            hands[1], // T55J5 - Three of a kind [T]
            hands[4],  // QQQJA - Three of a kind [Q]
        ])
    }

    #[test]
    fn test_sort_test_input2() {
        let hands = TEST_INPUT.lines().map(|l| l.parse::<Hand2>()).collect::<Result<Vec<_>, _>>().unwrap();

        let mut sorted = hands.clone();
        sorted.sort();

        assert_eq!(sorted, vec![
            hands[0], // 32T3K - Pair
            hands[2], // KK677 - Two pair
            hands[1], // T55J5 - Three of a kind [T]
            hands[4], // QQQJA - Three of a kind [Q]
            hands[3], // KTJJT - Three of a kind [K]
        ])
    }

    #[test]
    fn test_get_winnings() {
        let hands = TEST_INPUT.lines().map(|l| l.parse::<Hand>()).collect::<Result<Vec<_>, _>>().unwrap();

        assert_eq!(get_winnings(&hands), 6440);
    }

    const TEST_INPUT: &str = "\
        32T3K 765\n\
        T55J5 684\n\
        KK677 28\n\
        KTJJT 220\n\
        QQQJA 483\
    ";
}

// # std trait implementations
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.get_kind().cmp(&other.get_kind()) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => {
                // Compare card values per position
                for i in 0..5 {
                    if self.cards[i] < other.cards[i] {
                        return Ordering::Less;
                    }
                    if self.cards[i] > other.cards[i] {
                        return Ordering::Greater;
                    }
                }
                return Ordering::Equal;
            }
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand2 {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.get_kind().cmp(&other.get_kind()) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => {
                // Compare card values per position
                for i in 0..5 {
                    if self.cards[i] < other.cards[i] {
                        return Ordering::Less;
                    }
                    if self.cards[i] > other.cards[i] {
                        return Ordering::Greater;
                    }
                }
                return Ordering::Equal;
            }
        }
    }
}

impl PartialOrd for Hand2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HandKind {
    fn cmp(&self, other: &Self) -> Ordering {
        if self == other { return Ordering::Equal; }

        match (self, other) {
            (HandKind::FiveOfAKind, _) => Ordering::Greater,
            (HandKind::FourOfAKind, HandKind::FiveOfAKind) => Ordering::Less,
            (HandKind::FourOfAKind, _) => Ordering::Greater,
            (HandKind::FullHouse, HandKind::FiveOfAKind | HandKind::FourOfAKind) => Ordering::Less,
            (HandKind::FullHouse, _) => Ordering::Greater,
            (HandKind::ThreeOfAKind, HandKind::FiveOfAKind | HandKind::FourOfAKind | HandKind::FullHouse) => Ordering::Less,
            (HandKind::ThreeOfAKind, _) => Ordering::Greater,
            (HandKind::TwoPair, HandKind::Pair | HandKind::Garbage) => Ordering::Greater,
            (HandKind::TwoPair, _) => Ordering::Less,
            (HandKind::Pair, HandKind::Garbage) => Ordering::Greater,
            (HandKind::Pair, _) => Ordering::Less,
            (HandKind::Garbage, _) => Ordering::Less
        }
    }
}

impl PartialOrd for HandKind {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for Hand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parser = Parser::new(s);
        let mut cards: [u8; 5] = [0; 5];

        for i in 0..5 {
            cards[i] = match parser.one_of(vec!["2", "3", "4", "5", "6", "7", "8", "9", "T", "J", "Q", "K", "A"])? {
                val @ ("2" | "3" | "4" | "5" | "6" | "7" | "8" | "9") => parse_u8(val)?,
                "T" => 10,
                "J" => 11,
                "Q" => 12,
                "K" => 13,
                "A" => 14,
                inv => return Err(format!("Invalid char '{}'", inv))
            }
        }

        let bid = parser.usize()?;
        parser.ensure_exhausted()?;

        Ok(Hand {
            cards,
            bid,
        })
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        fn get_card_display(card: u8) -> char {
            match card {
                2..=9 => (('2' as u8) + (card - 2)) as char,
                10 => 'T',
                11 => 'J',
                12 => 'Q',
                13 => 'K',
                14 => 'A',
                _ => panic!("Invalid card value: {}", card)
            }
        }

        for card in self.cards {
            write!(f, "{}", get_card_display(card))?;
        }

        write!(f, " {}", self.bid)
    }
}

impl FromStr for Hand2 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parser = Parser::new(s);
        let mut cards: [u8; 5] = [0; 5];

        for i in 0..5 {
            cards[i] = match parser.one_of(vec!["2", "3", "4", "5", "6", "7", "8", "9", "T", "J", "Q", "K", "A"])? {
                "J" => 1,
                val @ ("2" | "3" | "4" | "5" | "6" | "7" | "8" | "9") => parse_u8(val)?,
                "T" => 10,
                "Q" => 11,
                "K" => 12,
                "A" => 13,
                inv => return Err(format!("Invalid char '{}'", inv))
            }
        }

        let bid = parser.usize()?;
        parser.ensure_exhausted()?;

        Ok(Hand2 {
            cards,
            bid,
        })
    }
}

impl Display for Hand2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        fn get_card_display(card: u8) -> char {
            match card {
                1 => 'J',
                2..=9 => (('2' as u8) + (card - 2)) as char,
                10 => 'T',
                11 => 'Q',
                12 => 'K',
                13 => 'A',
                _ => panic!("Invalid card value: {}", card)
            }
        }

        for card in self.cards {
            write!(f, "{}", get_card_display(card))?;
        }

        write!(f, " {}", self.bid)
    }
}
