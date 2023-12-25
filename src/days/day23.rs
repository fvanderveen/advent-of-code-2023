use std::collections::{HashMap, VecDeque};
use std::str::FromStr;
use crate::days::Day;
use crate::util::geometry::{Directions, Grid, Point};

pub const DAY23: Day = Day {
    puzzle1,
    puzzle2,
};

fn puzzle1(input: &String) {
    let map: Map = input.parse().unwrap();

    println!("Longest hike path: {} steps", map.longest_hike_path(false).unwrap());
}

fn puzzle2(input: &String) {
    let map: Map = input.parse().unwrap();

    println!("Longest non-slippery hike path: {} steps", map.longest_hike_path(true).unwrap());
}

#[derive(Eq, PartialEq, Debug, Default, Copy, Clone)]
enum Tile {
    #[default]
    Forest,
    Path,
    SlopeNorth,
    SlopeEast,
    SlopeSouth,
    SlopeWest,
}

type Map = Grid<Tile>;

impl Map {
    fn start(&self) -> Point {
        self.get_row(0).iter().position(|t| Tile::Path.eq(t)).map(|x| Point { x: x as isize, y: 0 })
            .ok_or("Could not find a start point?!")
            .unwrap()
    }

    fn end(&self) -> Point {
        let y = self.bounds.bottom();
        self.get_row(y).iter().position(|t| Tile::Path.eq(t)).map(|x| Point { x: x as isize, y })
            .ok_or("Could not find an end point?!")
            .unwrap()
    }

    fn longest_hike_path(&self, ignore_slopes: bool) -> Option<usize> {
        // Dijkstra really works for shortest only (always picking the longest current path will not switch after all)
        // Since we also need to make sure we don't step on the same tile twice, we need some history per path.
        // Most simplest implementation would be running depth-first. Let's see how that fares.
        let mut queue: VecDeque<(Vec<Point>, usize, Option<Point>, Point)> = VecDeque::new();

        let start = self.start();
        let end = self.end();

        #[derive(Eq, PartialEq, Hash)]
        struct DistanceKey {
            from: Option<Point>,
            to: Point,
        }

        let mut distances: HashMap<DistanceKey, usize> = HashMap::new();
        let mut longest = None;

        queue.push_back((vec![], 0, None, start));
        while !queue.is_empty() {
            let (trail, current_steps, previous_point, current_point) = queue.pop_front()?;

            let key = DistanceKey { from: previous_point, to: current_point };
            if let Some(current_max) = distances.get(&key) {
                // got here already with a longer (or same) trail (going in the same direction)?
                if current_steps.lt(current_max) {
                    continue;
                }
            }
            distances.insert(key, current_steps);

            if current_point.eq(&end) {
                let so_far = longest.unwrap_or(0);
                longest = Some(so_far.max(current_steps));
                continue;
            }

            let current_tile = self.get(&current_point)?;

            // If we ended up in the forest, we're fucked
            // If the current_tile is a slope, we need to follow it down
            // Otherwise, given current_point, get points around and see where we can go
            // - Forest cannot be entered
            // - Point already in trail we don't want to
            // - Slope needs to go down to follow
            let directions = match current_tile {
                Tile::Forest => None,
                Tile::SlopeNorth if !ignore_slopes => Some(Directions::Top),
                Tile::SlopeEast if !ignore_slopes => Some(Directions::Right),
                Tile::SlopeSouth if !ignore_slopes => Some(Directions::Bottom),
                Tile::SlopeWest if !ignore_slopes => Some(Directions::Left),
                _ => Some(Directions::NonDiagonal)
            }?;

            // Note: only junctions are interesting to check. We might follow a path back, but at least the list of
            // points is not growing too fast (which might also be an issue...)

            // A tile is a junction if there are more than 2 non-forest tiles.
            let is_junction = self.get_adjacent(&current_point, Directions::NonDiagonal).iter().filter(|t| Tile::Forest.ne(t)).count() > 2;
            let next_trail = if is_junction {
                let mut t = trail.clone();
                t.push(current_point);
                t
            } else {
                trail.clone()
            };

            for (point, tile) in self.get_adjacent_entries(&current_point, directions) {
                if trail.contains(&point) { // Avoid going to a junction already visited
                    continue;
                }
                if previous_point.is_some_and(|pp| pp == point) { // Don't go back
                    continue;
                }

                match tile {
                    Tile::Forest => continue, // Cannot enter
                    _ if ignore_slopes => {} // Allow continuing wherever
                    Tile::SlopeNorth if point.y > current_point.y => continue, // Cannot climb up
                    Tile::SlopeEast if point.x < current_point.x => continue,
                    Tile::SlopeSouth if point.y < current_point.y => continue,
                    Tile::SlopeWest if point.x > current_point.x => continue,
                    _ => {}
                };

                queue.push_back((next_trail.clone(), current_steps + 1, Some(current_point), point));
            }
        }

        longest
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day23::Map;

    #[test]
    fn test_longest_hike_path() {
        let map: Map = TEST_INPUT.parse().unwrap();

        assert_eq!(map.longest_hike_path(false), Some(94));
        assert_eq!(map.longest_hike_path(true), Some(154));
    }

    const TEST_INPUT: &str = "\
        #.#####################\n\
        #.......#########...###\n\
        #######.#########.#.###\n\
        ###.....#.>.>.###.#.###\n\
        ###v#####.#v#.###.#.###\n\
        ###.>...#.#.#.....#...#\n\
        ###v###.#.#.#########.#\n\
        ###...#.#.#.......#...#\n\
        #####.#.#.#######.#.###\n\
        #.....#.#.#.......#...#\n\
        #.#####.#.#.#########v#\n\
        #.#...#...#...###...>.#\n\
        #.#.#v#######v###.###v#\n\
        #...#.>.#...>.>.#.###.#\n\
        #####v#.#.###v#.#.###.#\n\
        #.....#...#...#.#.#...#\n\
        #.#########.###.#.#.###\n\
        #...###...#...#...#.###\n\
        ###.###.#.###v#####v###\n\
        #...#...#.#.>.>.#.>.###\n\
        #.###.###.#.###.#.#v###\n\
        #.....###...###...#...#\n\
        #####################.#\
    ";
}

impl FromStr for Tile {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "#" => Ok(Self::Forest),
            "." => Ok(Self::Path),
            "^" => Ok(Self::SlopeNorth),
            ">" => Ok(Self::SlopeEast),
            "v" => Ok(Self::SlopeSouth),
            "<" => Ok(Self::SlopeWest),
            _ => Err(format!("Invalid tile '{}'", s))
        }
    }
}