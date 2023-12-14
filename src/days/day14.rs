use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::days::Day;
use crate::util::geometry::{Directions, Grid, Point};

pub const DAY14: Day = Day {
    puzzle1,
    puzzle2,
};

fn puzzle1(input: &String) {
    let mut platform = input.parse::<Platform>().unwrap();
    platform.tilt(Directions::Top);

    println!("North beam load: {}", platform.get_north_beam_load());
}

fn puzzle2(input: &String) {
    let mut platform = input.parse::<Platform>().unwrap();

    let load_result = platform.run_spin_cycle();
    println!("North beam load after 1.000.000.000 spins: {}", load_result);
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Default)]
enum Tile {
    Boulder,
    Rock,
    #[default]
    Ground,
}

type Platform = Grid<Tile>;

impl Platform {
    fn tilt(&mut self, direction: Directions) {
        // 1. Take all boulders, sorted based on the direction
        //    - e.g. to top, sort based on lowest y value
        // 2. For each boulder, get everything in the direction
        // 3. Determine the last ground spot in the direction
        // 4. Move the boulder
        // ???
        // 5. Probably refactor for puzzle 2 :silly:

        // 5 was unfortunately true. It is too slow to run cycles with the above.
        // New plan: take the rows/columns, run over them (in the right direction) moving everything
        // Clear out all boulders from the grid
        // Insert them in the new positions based on the results.
        let lines = match direction {
            Directions::Top | Directions::Bottom => self.columns(),
            Directions::Left | Directions::Right => self.rows(),
            _ => return
        };

        let mut result = vec![];
        for line in lines {
            let mut new_line = vec![Tile::Ground; line.len()]; // Pre-allocate with all ground tiles.

            let is_reverse = direction == Directions::Bottom || direction == Directions::Right;
            let range = 0..line.len();

            let mut free_index = None;

            let indexes = if is_reverse { range.rev().collect::<Vec<_>>() } else { range.collect::<Vec<_>>() };
            for i in indexes {
                match line[i] {
                    Tile::Ground if free_index.is_none() => { free_index = Some(i) },
                    Tile::Ground => {} // More free ground~
                    Tile::Rock => free_index = None,
                    Tile::Boulder if free_index.is_some() => {
                        // Update index to be a boulder, move index (based on is_reverse)
                        let index = free_index.unwrap();
                        new_line[index] = Tile::Boulder;
                        free_index = Some(if is_reverse { index - 1 } else { index + 1 })
                    },
                    Tile::Boulder => {
                        // Just add the boulder at the same spot, cannot move:
                        new_line[i] = Tile::Boulder
                    },
                }
            }

            result.push(new_line);
        }

        // Remove boulders from the grid:
        self.entries().iter().filter(|(_, t)| Tile::Boulder.eq(t)).for_each(|(p, _)| self.set(*p, Tile::Ground));

        // Store result back into the grid
        for i1 in 0..result.len() {
            let line = &result[i1];
            for i2 in 0..line.len() {
                if line[i2] != Tile::Boulder { continue; }

                // Determine which is x, which is y based on direction:
                let (x, y) = match direction {
                    // lines are columns (x), each tile is a row (y)
                    Directions::Top | Directions::Bottom => (i1, i2),
                    Directions::Left | Directions::Right => (i2, i1),
                    _ => return
                };

                self.set(Point::try_from((x, y)).unwrap(), Tile::Boulder);
            }
        }
    }

    fn get_north_beam_load(&self) -> usize {
        // Each boulder causes a load depending on the row from the bottom.
        // Basically, take the height of this grid, and subtract the y position.
        self.entries().iter().filter(|(_, t)| Tile::Boulder.eq(t))
            .map(|(p, _)| self.bounds.height - p.y as usize)
            .sum()
    }

    fn run_spin_cycle(&mut self) -> usize {
        // We need to run 1.000.000.000 cycles. A cycle is a tilt top => left => bottom => right.
        // Obviously, running that real-time is _probably_ going to take too long.
        // However, knowing AoC, at some point this process will stabilize and start looping at some point.
        // As such, once we find the loop, we can just figure out where in the loop we'll end.

        // To find the loop, we run the cycles storing the boulder locations after each cycle.
        // Once we find a state we've already seen, we know the loop size and offset, and the rest will be simple.

        #[derive(Eq, PartialEq, Hash)]
        struct State { boulders: Vec<Point> }

        let mut states: Vec<State> = vec![];

        loop {
            let boulders = self.entries().iter().filter(|(_, t)| Tile::Boulder.eq(t)).map(|(p, _)| *p).collect();
            let state = State { boulders };

            if let Some(offset) = states.iter().position(|s| state.eq(s)) {
                let loop_len = states.len() - offset;
                println!("Found loop from {} of len {}", offset, loop_len);
                // Compute the 1.000.000.000 cycle state, and get the weight from that.

                let target_index = offset + ((1_000_000_000 - offset) % loop_len);
                return states[target_index].boulders.iter().map(|p| self.bounds.height - p.y as usize).sum()
            }

            states.push(state);

            // Do a cycle:
            self.tilt(Directions::Top);    // North
            self.tilt(Directions::Left);   // West
            self.tilt(Directions::Bottom); // South
            self.tilt(Directions::Right);  // East
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day14::Platform;
    use crate::util::geometry::Directions;

    #[test]
    fn test_get_north_beam_load() {
        let mut grid = TEST_INPUT.parse::<Platform>().unwrap();
        grid.tilt(Directions::Top);

        assert_eq!(grid.get_north_beam_load(), 136);
    }

    #[test]
    fn test_run_spin_cycle() {
        let mut grid = TEST_INPUT.parse::<Platform>().unwrap();

        assert_eq!(grid.run_spin_cycle(), 64);
    }

    #[test]
    fn test_tilt() {
        let mut grid = TEST_INPUT.parse::<Platform>().unwrap();

        grid.tilt(Directions::Top);
        assert_eq!(format!("{}", grid), "\
            OOOO.#.O..\n\
            OO..#....#\n\
            OO..O##..O\n\
            O..#.OO...\n\
            ........#.\n\
            ..#....#.#\n\
            ..O..#.O.O\n\
            ..O.......\n\
            #....###..\n\
            #....#....\
        ");

        grid.tilt(Directions::Right);
        assert_eq!(format!("{}", grid), "\
            .OOOO#...O\n\
            ..OO#....#\n\
            ..OOO##..O\n\
            ..O#....OO\n\
            ........#.\n\
            ..#....#.#\n\
            ....O#..OO\n\
            .........O\n\
            #....###..\n\
            #....#....\
        ");

        grid.tilt(Directions::Right);
        assert_eq!(format!("{}", grid), "\
            .OOOO#...O\n\
            ..OO#....#\n\
            ..OOO##..O\n\
            ..O#....OO\n\
            ........#.\n\
            ..#....#.#\n\
            ....O#..OO\n\
            .........O\n\
            #....###..\n\
            #....#....\
        ");

        grid.tilt(Directions::Bottom);
        assert_eq!(format!("{}", grid), "\
            ...OO#...O\n\
            ..OO#....#\n\
            ..OO.##...\n\
            ..O#....OO\n\
            ..O.....#O\n\
            ..#....#.#\n\
            .....#....\n\
            ..........\n\
            #...O###.O\n\
            #O..O#..OO\
        ");

        grid.tilt(Directions::Left);
        assert_eq!(format!("{}", grid), "\
            OO...#O...\n\
            OO..#....#\n\
            OO...##...\n\
            O..#OO....\n\
            O.......#O\n\
            ..#....#.#\n\
            .....#....\n\
            ..........\n\
            #O...###O.\n\
            #OO..#OO..\
        ");

        grid.tilt(Directions::Top);
        assert_eq!(format!("{}", grid), "\
            OO...#O...\n\
            OO..#....#\n\
            OO..O##..O\n\
            OO.#.O....\n\
            OO......#.\n\
            ..#....#O#\n\
            ..O..#....\n\
            ..........\n\
            #....###..\n\
            #....#OO..\
        ");
    }

    const TEST_INPUT: &str = "\
        O....#....\n\
        O.OO#....#\n\
        .....##...\n\
        OO.#O....O\n\
        .O.....O#.\n\
        O.#..O.#.#\n\
        ..O..#O..O\n\
        .......O..\n\
        #....###..\n\
        #OO..#....\
    ";
}

impl FromStr for Tile {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "O" => Ok(Tile::Boulder),
            "#" => Ok(Tile::Rock),
            "." => Ok(Tile::Ground),
            _ => Err(format!("Invalid tile '{}'", s))
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Boulder => write!(f, "O"),
            Tile::Rock => write!(f, "#"),
            Tile::Ground => write!(f, ".")
        }
    }
}