use crate::days::Day;
use crate::util::parser::Parser;

pub const DAY2: Day = Day {
    puzzle1,
    puzzle2
};

fn puzzle1(input: &String) {
    let games = parse_input(input).unwrap();
    let bag = Bag { red: 12, green: 13, blue: 14 };

    let possible_games = filter_possible_games(games, &bag);
    let result = possible_games.iter().map(|g| g.id).sum::<isize>();

    println!("Sum of possible game IDs: {}", result);
}
fn puzzle2(input: &String) {
    let games = parse_input(input).unwrap();

    let minimum_bags: Vec<_> = games.iter().map(|g| get_smallest_bag_for_game(g).unwrap()).collect();
    let result: isize = minimum_bags.iter().map(|b| b.get_power()).sum();

    println!("Sum of power of minimum bags: {}", result);
}

#[derive(Clone, Eq, PartialEq, Default, Debug)]
struct Game {
    id: isize,
    pulls: Vec<Pull>
}

#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
struct Pull {
    red: isize,
    green: isize,
    blue: isize
}

#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
struct Bag {
    red: isize,
    green: isize,
    blue: isize
}

impl Bag {
    // The power of a set of cubes is equal to the numbers of red, green, and blue cubes multiplied together.
    pub fn get_power(&self) -> isize {
        self.red * self.green * self.blue
    }
}

fn parse_input(input: &str) -> Result<Vec<Game>, String> {
    input.lines().map(|l| parse_game(l)).collect()
}

fn parse_game(input: &str) -> Result<Game, String> {
    let mut parser: Parser = Parser::new(input);

    parser.literal("Game")?;
    let id = parser.isize()?;
    parser.literal(":")?;
    let mut pulls: Vec<Pull> = vec!();
    let mut pull = Pull::default();

    while !parser.is_exhausted() {
        let amount = parser.isize()?;
        match parser.one_of(vec!["red", "green", "blue"])? {
            "red" => pull.red += amount,
            "green" => pull.green += amount,
            "blue" => pull.blue += amount,
            other => return Err(format!("Wrong colour {}", other))
        }

        if parser.is_exhausted() {
            pulls.push(pull);
            break;
        }

        match parser.one_of(vec![",", ";"])? {
            "," => {},
            ";" => {
                pulls.push(pull);
                pull = Pull::default();
            },
            other => return Err(format!("Wrong separator {}", other))
        }
    }

    Ok(Game { id, pulls })
}

fn filter_possible_games(games: Vec<Game>, bag: &Bag) -> Vec<Game> {
    games.into_iter()
        .filter(|g| g.pulls.iter().all(|p| p.red <= bag.red && p.green <= bag.green && p.blue <= bag.blue))
        .collect()
}

fn get_smallest_bag_for_game(game: &Game) -> Option<Bag> {
    let red: isize = game.pulls.iter().map(|p| p.red).max()?;
    let green: isize = game.pulls.iter().map(|p| p.green).max()?;
    let blue: isize = game.pulls.iter().map(|p| p.blue).max()?;

    Some(Bag { red, green, blue })
}

#[cfg(test)]
mod tests {
    use crate::days::day02::{Bag, filter_possible_games, Game, get_smallest_bag_for_game, parse_game, parse_input, Pull};

    const TEST_INPUT: &str = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green\
    ";

    #[test]
    fn test_parse_game() {
        assert_eq!(parse_game("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"), Ok(Game {
            id: 1,
            pulls: vec![
                Pull { red: 4, green: 0, blue: 3 },
                Pull { red: 1, green: 2, blue: 6 },
                Pull { red: 0, green: 2, blue: 0 }
            ]
        }));
        assert_eq!(parse_game("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"), Ok(Game {
            id: 3,
            pulls: vec![
                Pull { red: 20, green: 8, blue: 6 },
                Pull { red: 4, green: 13, blue: 5 },
                Pull { red: 1, green: 5, blue: 0 }
            ]
        }));
    }

    #[test]
    fn test_parse_input() {
        let result = parse_input(TEST_INPUT);
        assert!(result.is_ok(), "Result was not OK: {}", result.err().unwrap());

        let games = result.unwrap();
        assert_eq!(games[0].id, 1);

        assert_eq!(games[3], Game {
            id: 4,
            pulls: vec![
                Pull { red: 3, green: 1, blue: 6 },
                Pull { red: 6, green: 3, blue: 0 },
                Pull { red: 14, green: 3, blue: 15 },
            ]
        })
    }

    #[test]
    fn test_filter_possible_games() {
        let games = parse_input(TEST_INPUT).unwrap();
        let filtered_games = filter_possible_games(games, &Bag { red: 12, green: 13, blue: 14 });

        assert_eq!(filtered_games.len(), 3);
        assert_eq!(filtered_games.iter().map(|g| g.id).collect::<Vec<_>>(), vec![1, 2, 5]);
    }

    #[test]
    fn test_get_smallest_bag_for_game() {
        let games = parse_input(TEST_INPUT).unwrap();

        assert_eq!(get_smallest_bag_for_game(&games[0]), Some(Bag { red: 4, green: 2, blue: 6 }));
        assert_eq!(get_smallest_bag_for_game(&games[1]), Some(Bag { red: 1, green: 3, blue: 4 }));
        assert_eq!(get_smallest_bag_for_game(&games[2]), Some(Bag { red: 20, green: 13, blue: 6 }));
        assert_eq!(get_smallest_bag_for_game(&games[3]), Some(Bag { red: 14, green: 3, blue: 15 }));
        assert_eq!(get_smallest_bag_for_game(&games[4]), Some(Bag { red: 6, green: 3, blue: 2 }));
    }

    #[test]
    fn test_bag_get_power() {
        // The power of the minimum set of cubes in game 1 is 48.
        // In games 2-5 it was 12, 1560, 630, and 36, respectively.
        assert_eq!((Bag { red: 4, green: 2, blue: 6 }).get_power(), 48);
        assert_eq!((Bag { red: 1, green: 3, blue: 4 }).get_power(), 12);
        assert_eq!((Bag { red: 20, green: 13, blue: 6 }).get_power(), 1560);
        assert_eq!((Bag { red: 14, green: 3, blue: 15 }).get_power(), 630);
        assert_eq!((Bag { red: 6, green: 3, blue: 2 }).get_power(), 36);
    }
}