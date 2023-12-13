use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::days::Day;
use crate::days::day13::Mirror::{Horizontal, Vertical};
use crate::util::geometry::Grid;

pub const DAY13: Day = Day {
    puzzle1,
    puzzle2,
};

fn puzzle1(input: &String) {
    let maps = parse_input(input).unwrap();

    let result: usize = maps.iter()
        .map(|m| m.get_mirror().unwrap())
        .map(|m| m.get_value())
        .sum();
    println!("Sum of summarized data: {}", result);
}

fn puzzle2(input: &String) {
    let maps = parse_input(input).unwrap();

    let result: usize = maps.iter()
        .map(|m| m.get_mirror_v2().unwrap())
        .map(|m| m.get_value())
        .sum();
    println!("Sum of fixed summarized data: {}", result);
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Default)]
enum Tile {
    #[default]
    Ground,
    Rock,
}

type Map = Grid<Tile>;

/**
 * Mirror line, usize param is the amount of columns/rows _before_ the mirror line.
 */
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Mirror {
    Horizontal(usize),
    Vertical(usize)
}

impl Map {
    fn get_mirror(&self) -> Option<Mirror> {
        // Try finding the line after which the map mirrors (either horizontal or vertical)
        // At least for part 1 (I'm already afraid for 2...) it should be easy enough to just loop over all columns/rows
        // We can start at index 1, as we need at least _a_ line above/below the mirrors.

        fn find_mirror_index(lines: Vec<Vec<Tile>>) -> Option<usize> {
            for i in 1..lines.len() {
                if lines[i-1] != lines[i] { continue };
                let (first, second) = lines.split_at(i);
                let len = first.len().min(second.len());
                let first_lines: Vec<_> = first.iter().rev().take(len).collect();
                let second_lines: Vec<_> = second.iter().take(len).collect();
                if first_lines == second_lines {
                    return Some(i);
                }
            }
            None
        }

        let rows: Vec<_> = self.bounds.y().map(|y| self.get_row(y)).collect();
        let cols: Vec<_> = self.bounds.x().map(|x| self.get_column(x)).collect();
        if let Some(index) = find_mirror_index(rows) {
            Some(Horizontal(index))
        } else if let Some(index) = find_mirror_index(cols) {
            Some(Vertical(index))
        } else {
            None
        }
    }

    fn get_mirror_v2(&self) -> Option<Mirror> {
        // Try finding the line after which the map mirrors when changing _exactly_ one tile to the other type (either horizontal or vertical)
        // Idea: loop over all possible mirror lines, and compute the differences in this mirror.
        // If the difference is exactly one, it is the new valid mirror line.
        fn get_differences_in_line(left: &Vec<Tile>, right: &Vec<Tile>) -> usize {
            let mut differences = 0;
            for i in 0..left.len() {
                if left[i] != right[i] { differences += 1 }
            }
            differences
        }

        fn get_differences_in_mirror(lines: &Vec<Vec<Tile>>, index: usize) -> usize {
            let (first, second) = lines.split_at(index);
            let len = first.len().min(second.len());
            let first_lines: Vec<_> = first.iter().rev().take(len).collect();
            let second_lines: Vec<_> = second.iter().take(len).collect();

            let mut differences = 0;
            for i in 0..len {
                differences += get_differences_in_line(first_lines[i], second_lines[i])
            }
            differences
        }

        fn find_mirror_index(lines: Vec<Vec<Tile>>) -> Option<usize> {
            for i in 1..lines.len() {
                if get_differences_in_mirror(&lines, i) == 1 { return Some(i) }
            }
            None
        }

        let rows: Vec<_> = self.bounds.y().map(|y| self.get_row(y)).collect();
        let cols: Vec<_> = self.bounds.x().map(|x| self.get_column(x)).collect();
        if let Some(index) = find_mirror_index(rows) {
            Some(Horizontal(index))
        } else if let Some(index) = find_mirror_index(cols) {
            Some(Vertical(index))
        } else {
            None
        }
    }
}

impl Mirror {
    fn get_value(&self) -> usize {
        match self {
            Vertical(value) => *value,
            Horizontal(value) => 100 * value,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day13::{Map, parse_input, Tile};
    use crate::days::day13::Mirror::{Horizontal, Vertical};
    use crate::util::geometry::Bounds;

    #[test]
    fn test_parse_input() {
        let result = parse_input(TEST_INPUT);
        assert!(result.is_ok(), "Expected Ok, but got Err({})", result.err().unwrap());

        let maps = result.unwrap();
        assert_eq!(maps.len(), 2);
        assert_eq!(maps[0].bounds, Bounds::from_size(9, 7));
        assert_eq!(maps[1].bounds, Bounds::from_size(9, 7));

        assert_eq!(maps[0].get_row(0), vec![Tile::Rock, Tile::Ground, Tile::Rock, Tile::Rock, Tile::Ground, Tile::Ground, Tile::Rock, Tile::Rock, Tile::Ground])
    }

    #[test]
    fn test_map_get_mirror() {
        let maps = parse_input(TEST_INPUT).unwrap();

        assert_eq!(maps[0].get_mirror(), Some(Vertical(5)));
        assert_eq!(maps[1].get_mirror(), Some(Horizontal(4)));

        let map = FAILING_CASE.parse::<Map>().unwrap();
        assert_eq!(map.get_mirror(), Some(Horizontal(10)))
    }

    #[test]
    fn test_map_get_mirror_v2() {
        let maps = parse_input(TEST_INPUT).unwrap();

        assert_eq!(maps[0].get_mirror_v2(), Some(Horizontal(3)));
        assert_eq!(maps[1].get_mirror_v2(), Some(Horizontal(1)));
    }

    #[test]
    fn test_mirror_get_value() {
        assert_eq!(Vertical(5).get_value(), 5);
        assert_eq!(Horizontal(4).get_value(), 400);
    }

    const TEST_INPUT: &str = "\
        #.##..##.\n\
        ..#.##.#.\n\
        ##......#\n\
        ##......#\n\
        ..#.##.#.\n\
        ..##..##.\n\
        #.#.##.#.\n\
        \n\
        #...##..#\n\
        #....#..#\n\
        ..##..###\n\
        #####.##.\n\
        #####.##.\n\
        ..##..###\n\
        #....#..#\
    ";

    const FAILING_CASE: &str = "\
        #.##.#..#...#..\n\
        .#...#.....#...\n\
        #.#..#.#.##.###\n\
        .#####.#.#..#.#\n\
        .#####.#.#..#.#\n\
        #.#..#.#.##.###\n\
        .#...#.#...#...\n\
        #.##.#..#...#..\n\
        #.#.#.###.####.\n\
        #..###....###..\n\
        #..###....###..\
    ";
}

fn parse_input(input: &str) -> Result<Vec<Map>, String> {
    let mut result = vec![];
    let mut current_lines = vec![];

    // Note: we add an empty line at the end to ensure we've pushed any last result
    for line in input.lines().chain(vec![""]) {
        if line.is_empty() {
            if current_lines.len() > 0 {
                result.push(current_lines.join("\n").parse()?);
            }
            current_lines.clear();
        } else {
            current_lines.push(line);
        }
    }

    Ok(result)
}

impl FromStr for Tile {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "#" => Ok(Self::Rock),
            "." => Ok(Self::Ground),
            _ => Err(format!("Invalid tile: '{}'", s))
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Ground => write!(f, "."),
            Tile::Rock => write!(f, "#")
        }
    }
}