use std::collections::{HashSet, VecDeque};
use std::str::FromStr;
use crate::days::Day;
use crate::util::collection::CollectionExtension;
use crate::util::geometry::{Directions, Grid, Point};

pub const DAY16: Day = Day {
    puzzle1,
    puzzle2
};

fn puzzle1(input: &String) {
    let contraption = input.parse::<Contraption>().unwrap();
    println!("Number of energized tiles: {}", contraption.get_energized_tiles());
}

fn puzzle2(input: &String) {
    let contraption = input.parse::<Contraption>().unwrap();
    println!("Max number of energized tiles: {}", contraption.get_max_energized_tiles());
}

#[derive(Eq, PartialEq, Debug, Default, Copy, Clone)]
enum Tile {
    #[default]
    Empty,
    MirrorRight, // /
    MirrorLeft,  // \
    HorSplit,    // -
    VerSplit     // |
}

impl Tile {
    fn get_next_directions(&self, input: Directions) -> Vec<Directions> {
        match self {
            Self::Empty => vec![input],
            Self::HorSplit if Directions::Horizontal.has(input) => vec![input],
            Self::HorSplit => vec![Directions::Left, Directions::Right],
            Self::VerSplit if Directions::Vertical.has(input) => vec![input],
            Self::VerSplit => vec![Directions::Top, Directions::Bottom],
            // /
            Self::MirrorRight if input == Directions::Top => vec![Directions::Right],
            Self::MirrorRight if input == Directions::Right => vec![Directions::Top],
            Self::MirrorRight if input == Directions::Bottom => vec![Directions::Left],
            Self::MirrorRight if input == Directions::Left => vec![Directions::Bottom],
            // \
            Self::MirrorLeft if input == Directions::Top => vec![Directions::Left],
            Self::MirrorLeft if input == Directions::Right => vec![Directions::Bottom],
            Self::MirrorLeft if input == Directions::Bottom => vec![Directions::Right],
            Self::MirrorLeft if input == Directions::Left => vec![Directions::Top],
            _ => vec![]
        }
    }
}

type Contraption = Grid<Tile>;

impl Contraption {
    fn get_energized_tiles(&self) -> usize {
        // Start in top-left corner, going right
        self.get_energized_tiles_from(Point::from((0, 0)), Directions::Right)
    }

    fn get_energized_tiles_from(&self, start: Point, direction: Directions) -> usize {
        let mut energized_tiles: HashSet<(Point, Directions)> = HashSet::new();
        let mut queue: VecDeque<(Point, Directions)> = VecDeque::from([(start, direction)]);

        loop {
            if let Some((current_point, direction)) = queue.pop_front() {
                // Already visited going this direction?
                if !energized_tiles.insert((current_point, direction)) { continue; }

                // Get current tile:
                let tile = match self.get(&current_point) {
                    Some(tile) => tile,
                    None => continue
                };

                // Get next direction(s)
                for direction in tile.get_next_directions(direction) {
                    if let [point] = self.get_adjacent_points(&current_point, direction)[..] {
                        queue.push_back((point, direction));
                    }
                }
            } else {
                break;
            }
        }

        energized_tiles.iter().map(|(p, _)| p).collect::<Vec<_>>().deduplicate().len()
    }

    fn get_max_energized_tiles(&self) -> usize {
        // 'Dumb' solution, just try for each side and each column (4x110 starts)
        // Let's see how fast it is :joy:
        let mut result = 0;

        for row in self.bounds.y() {
            result = result.max(self.get_energized_tiles_from(Point::from((0, row)), Directions::Right));
            result = result.max(self.get_energized_tiles_from(Point::from((0, row)), Directions::Left));
        }
        for col in self.bounds.x() {
            result = result.max(self.get_energized_tiles_from(Point::from((col, 0)), Directions::Bottom));
            result = result.max(self.get_energized_tiles_from(Point::from((col, 0)), Directions::Top));
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day16::Contraption;

    #[test]
    fn test_get_energized_tiles() {
        let contraption = TEST_INPUT.parse::<Contraption>().unwrap();
        assert_eq!(contraption.get_energized_tiles(), 46);
    }

    #[test]
    fn test_get_max_energized_tiles() {
        let contraption = TEST_INPUT.parse::<Contraption>().unwrap();
        assert_eq!(contraption.get_max_energized_tiles(), 51);
    }

    const TEST_INPUT: &str = "\
        .|...\\....\n\
        |.-.\\.....\n\
        .....|-...\n\
        ........|.\n\
        ..........\n\
        .........\\\n\
        ..../.\\\\..\n\
        .-.-/..|..\n\
        .|....-|.\\\n\
        ..//.|....\
    ";
}

impl FromStr for Tile {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Tile::Empty),
            "|" => Ok(Tile::VerSplit),
            "-" => Ok(Tile::HorSplit),
            "/" => Ok(Tile::MirrorRight),
            "\\" => Ok(Tile::MirrorLeft),
            _ => Err(format!("Invalid tile '{}'", s))
        }
    }
}