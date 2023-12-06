use std::str::FromStr;
use crate::days::Day;
use crate::util::number::parse_usize;
use crate::util::parser::Parser;

pub const DAY6: Day = Day {
    puzzle1,
    puzzle2,
};

fn puzzle1(input: &String) {
    let puzzle = input.parse::<Puzzle>().unwrap();

    let result = puzzle.races.iter().map(|r| r.get_ways_to_win()).reduce(|l,r| l*r).unwrap();

    println!("Puzzle 1 result: {}", result);
}

fn puzzle2(input: &String) {
    let race = input.parse::<Race>().unwrap();

    println!("Puzzle 2 result: {}", race.get_ways_to_win());
}

#[derive(Eq, PartialEq, Debug, Default, Clone)]
struct Puzzle {
    races: Vec<Race>,
}

impl FromStr for Puzzle {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut times = vec![];
        let mut records = vec![];

        let mut parser = Parser::new(s);
        parser.literal("Time:")?;
        loop {
            match parser.usize() {
                Ok(time) => times.push(time),
                Err(_) => break
            }
        }

        parser.literal("Distance:")?;
        loop {
            match parser.usize() {
                Ok(record) => records.push(record),
                Err(_) => break
            }
        }

        if times.len() != records.len() {
            Err(format!("Expected the same amount of times '{}' and distances '{}'", times.len(), records.len()))
        } else {
            Ok(Puzzle {
                races: times.iter().zip(records.iter()).map(|(duration, record)| Race { duration: *duration, record: *record }).collect::<Vec<_>>()
            })
        }
    }
}

#[derive(Eq, PartialEq, Debug, Default, Copy, Clone)]
struct Race {
    duration: usize, // ms
    record: usize,
}

impl Race {
    fn get_ways_to_win(&self) -> usize {
        // We can hold a button; for every ms pressed, the boat will go 1mm/ms
        // e.g. hold 1ms => 1mm/ms, hold 3ms => 3mm/ms
        // We can quite simply find the distance here by (T - ht) * ht (T = race time, ht = hold time)
        // e.g. for example 1 (RT=7) (7-ht)*ht, we need to solve that for results larger than the record.
        // e.g. for example 1 (7-ht)*ht > 9
        // Since there aren't any very large time numbers (at least for part 1...) we'll just compute this
        // We can also know that the result moves parabolic, as such we can stop once the ht causes the result to go <= record.
        (0..self.duration)
            .map(|ht| (self.duration - ht) * ht)
            .skip_while(|dist| self.record.ge(dist)) // skip while dist <= record
            .take_while(|dist| self.record.lt(dist)) // take while dist > record
            .count()
    }
}

impl FromStr for Race {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Puzzle 2 gets to the point where the race is actually one with bad kerning
        // So when parsing the input to a single Race, we'll combine all the time/distance characters first
        let lines: Vec<_> = s.lines().filter(|l| !l.trim().is_empty()).collect();
        if lines.len() != 2 { return Err(format!("Expected 2 lines of input but got {}", lines.len())) }
        if !lines[0].starts_with("Time:") { return Err(format!("Line 1 does not start with Time:")) }
        if !lines[1].starts_with("Distance:") { return Err(format!("Line 2 does not start with Distance:")) }

        // Parse
        let duration = parse_usize(&lines[0]["Time:".len()..].replace(" ", ""))?;
        let record = parse_usize(&lines[1]["Distance:".len()..].replace(" ", ""))?;

        Ok(Race { duration, record })
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day06::{Puzzle, Race};

    #[test]
    fn test_puzzle_from_str() {
        let result = TEST_INPUT.parse::<Puzzle>();
        assert!(result.is_ok(), "Expected Ok, got Err({})", result.err().unwrap());

        let puzzle = result.unwrap();
        assert_eq!(puzzle.races, vec![
            Race { duration: 7, record: 9 },
            Race { duration: 15, record: 40 },
            Race { duration: 30, record: 200 },
        ])
    }

    #[test]
    fn test_race_ways_to_win() {
        let puzzle = TEST_INPUT.parse::<Puzzle>().unwrap();

        assert_eq!(puzzle.races[0].get_ways_to_win(), 4);
        assert_eq!(puzzle.races[1].get_ways_to_win(), 8);
        assert_eq!(puzzle.races[2].get_ways_to_win(), 9);
    }

    #[test]
    fn test_race_ways_to_win_p2() {
        let race = TEST_INPUT.parse::<Race>().unwrap();

        assert_eq!(race.get_ways_to_win(), 71503);
    }

    const TEST_INPUT: &str = "\
        Time:      7  15   30\n\
        Distance:  9  40  200\n\
    ";
}