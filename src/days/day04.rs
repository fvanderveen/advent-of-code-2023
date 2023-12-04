use std::collections::HashMap;
use std::str::FromStr;
use crate::days::Day;
use crate::util::parser::Parser;

pub const DAY4: Day = Day {
    puzzle1,
    puzzle2
};

fn puzzle1(input: &String) {
    let cards = input.lines().map(|l| l.parse::<ScratchCard>().unwrap());
    let total_points = cards.map(|c| c.points()).sum::<u32>();

    println!("Sum of card points: {}", total_points);
}
fn puzzle2(input: &String) {
    let cards = input.lines().map(|l| l.parse::<ScratchCard>()).collect::<Result<Vec<_>, _>>().unwrap();

    let total_cards = get_total_cards(cards);
    println!("Your cards resulted in a pile of {} cards.", total_cards);
}

#[derive(Eq, PartialEq, Clone, Debug, Default)]
struct ScratchCard {
    id: usize,
    winning_numbers: Vec<usize>,
    card_numbers: Vec<usize>
}

impl ScratchCard {
    fn matching_numbers(&self) -> usize {
        self.card_numbers.iter().filter(|n| self.winning_numbers.contains(n)).count()
    }

    // A card's points are computed by matching the numbers against the winning numbers.
    // The first match is worth 1 point, any subsequent match doubles the points.
    fn points(&self) -> u32 {
        // 1, 2, 4, 8, etc is 2^0, 2^1, ...
        // So 2^(matches - 1)
        match self.matching_numbers() {
            0 => 0u32,
            value => 2u32.pow((value - 1) as u32)
        }
    }
}

impl FromStr for ScratchCard {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parser = Parser::new(s);

        parser.literal("Card")?;
        let id = parser.usize()?;
        parser.literal(":")?;

        let mut winning_numbers: Vec<usize> = vec!();
        let mut card_numbers: Vec<usize> = vec!();

        loop {
            match parser.usize() {
                Ok(number) => winning_numbers.push(number),
                Err(..) => break
            }
        }
        parser.literal("|")?;
        loop {
            match parser.usize() {
                Ok(number) => card_numbers.push(number),
                Err(..) => break
            }
        }

        Ok(Self { id, winning_numbers, card_numbers })
    }
}

fn get_total_cards(initial_cards: Vec<ScratchCard>) -> usize {
    // Cards actually win (copies!) of other cards.
    // If card 1 has 4 matching numbers, it yields an extra 2, 3, 4, and 5 card.
    // We need to 'repeat' this until we no longer get any winnings,
    // and get the total amount of cards (including the originals)

    // We first make a mapping of each initial card and how much numbers match
    // We then use that mapping to determine how much copies should be added to the pile.
    // Since cards only win cards _after_ them, we can simplify the process a bit by handling them
    // from last to first.
    let cards: HashMap<usize, &ScratchCard> = HashMap::from_iter(initial_cards.iter().map(|c| (c.id, c)));

    let mut winnings_map: HashMap<usize, usize> = HashMap::new();
    let mut current_id = initial_cards.iter().max_by_key(|s| s.id).unwrap().id;

    loop {
        let card = cards.get(&current_id).unwrap();
        let matches = card.matching_numbers();
        let mut winnings = matches; // We win at least a single copy of each won card
        for offset in 1..=matches {
            // And whatever all our won cards would win us.
            winnings += winnings_map.get(&(current_id + offset)).unwrap_or(&0);
        }
        winnings_map.insert(current_id, winnings);
        if current_id == 1 { break; }
        current_id -= 1;
    }

    // We now know how much each copy of a card wins, so all we need is to add all totals
    // We start with a single copy of all cards, and we add all the winnings they accumulated
    initial_cards.len() + winnings_map.values().sum::<usize>()
}

#[cfg(test)]
mod tests {
    use crate::days::day04::{get_total_cards, ScratchCard};

    const TEST_INPUT: &str = "\
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11\n\
    ";

    #[test]
    fn test_scratch_card_from_str() {
        assert_eq!("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".parse::<ScratchCard>(), Ok(ScratchCard {
            id: 1,
            winning_numbers: vec![41, 48, 83, 86, 17],
            card_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53]
        }));
        assert_eq!("Card  42:  5  4 99 36 85 18 63 81 61 47 | 43 73 80 88 76 64 79  7 86 94 98 39 37 56 33 51 49 90 70  6 20  1 21 59 82".parse::<ScratchCard>(), Ok(ScratchCard {
            id: 42,
            winning_numbers: vec![5,4,99,36,85,18,63,81,61,47 ],
            card_numbers: vec![43,73,80,88,76,64,79,7,86,94,98,39,37,56,33,51,49,90,70,6,20,1,21,59,82]
        }))
    }

    #[test]
    fn test_scratch_card_get_points() {
        let scratch_cards = TEST_INPUT.lines().map(|l| l.parse::<ScratchCard>().unwrap()).collect::<Vec<_>>();
        assert_eq!(scratch_cards[0].points(), 8);
        assert_eq!(scratch_cards[1].points(), 2);
        assert_eq!(scratch_cards[2].points(), 2);
        assert_eq!(scratch_cards[3].points(), 1);
        assert_eq!(scratch_cards[4].points(), 0);
        assert_eq!(scratch_cards[5].points(), 0);
    }

    #[test]
    fn test_get_total_cards() {
        let cards = TEST_INPUT.lines().map(|l| l.parse::<ScratchCard>()).collect::<Result<Vec<_>, _>>().unwrap();

        assert_eq!(get_total_cards(cards), 30);
    }
}