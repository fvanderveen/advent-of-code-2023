use std::collections::{HashMap, VecDeque};
use std::str::FromStr;
use crate::days::Day;
use crate::util::geometry::{Directions, Grid, Point};

pub const DAY21: Day = Day {
    puzzle1,
    puzzle2
};

fn puzzle1(input: &String) {
    let garden: Garden = input.parse().unwrap();
    println!("There are {} tiles reachable with 64 steps", garden.get_tiles_within(64));
}
fn puzzle2(input: &String) {
    todo!("Implement puzzle 2");
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, Default)]
enum Tile {
    Start,
    Garden,
    #[default]
    Rock
}

type Garden = Grid<Tile>;

impl Garden {
    fn get_tiles_within(&self, num_steps: usize) -> usize {
        // Find the start point, and queue it.
        // Keep a map of point -> lowest value to reach it (similar to Dijkstra)
        // Get the next point from the queue (we don't order it here, we need to find all spots at `num_steps` distance)
        // If we already got there quicker, ignore it
        // If we are at `num_steps` distance, stop
        // Otherwise, get possible next tiles (Tile::Garden) and enqueue

        // Note: the answer wants all tiles that can be reached, however, this included already visited tiles.
        // However², any tile visited can be visited again in another step. This means that if `num_steps` is even, we
        // want to ensure we store an even number in the distance map (and override any odd number if there is)

        let start = self.entries().iter().find(|(_, t)| Tile::Start.eq(t)).unwrap().0;
        let target_even = (num_steps % 2) == 0;

        let mut queue: VecDeque<(Point, usize)> = VecDeque::from([(start, 0)]);
        let mut distances: HashMap<Point, usize> = HashMap::new();

        while queue.len() > 0 {
            let (point, current_steps) = queue.pop_front().unwrap();

            if let Some(distance) = distances.get(&point) {
                if *distance <= current_steps { continue } // Already been here earlier
            }

            distances.insert(point, current_steps);

            if current_steps == num_steps { continue; } // No more steps to take

            // Get surrounding tiles, part 2 mentions that this garden actually infinitely loops; so if we get a point outside our bounds, we need to wrap it.
            for next_point in point.get_points_around(Directions::NonDiagonal) {
                let width = self.bounds.width as isize;
                let height = self.bounds.height as isize;

                // Remap point to be inside map domain
                let remapped_point = Point {
                    x: ((next_point.x % width) + width) % width,
                    y: ((next_point.y % height) + height) % height
                };

                if let Some(tile) = self.get(&remapped_point) {
                    if tile != Tile::Rock {
                        queue.push_back((next_point, current_steps + 1))
                    }
                }
            }
            // Note: we need something smarter to extrapolate. Even 5000 steps is already taking way too long, let alone
            // the 22 million we need for part 2.
            // Since we repeat the map, everything kinda repeats. Just like we know any same even/odd step tile can
            // become a target...
            // Filling 5 tiles (start, and NEWS of it) of map should be doable, and should give us the results we need (I think).
            // Either the odd/evenness of tiles is the same, or it flips between a repeat. If it's the same, we know that
            // every next map is the same. If it flips, we know that happens for every other map as well.
            // Just need to figure out how to compute the steps...
        }

        // We want to determine all distances that match the even-ness of the target
        distances.values().filter(|l| ((*l % 2) == 0) == target_even).count()
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day21::Garden;

    #[test]
    fn test_get_tiles_within() {
        let garden: Garden = TEST_INPUT.parse().unwrap();

        assert_eq!(garden.get_tiles_within(6), 16);
        assert_eq!(garden.get_tiles_within(10), 50);
        assert_eq!(garden.get_tiles_within(50), 1594);
        assert_eq!(garden.get_tiles_within(100), 6536);
        // assert_eq!(garden.get_tiles_within(5000), 16733044);
    }

    const TEST_INPUT: &str = "\
        ...........\n\
        .....###.#.\n\
        .###.##..#.\n\
        ..#.#...#..\n\
        ....#.#....\n\
        .##..S####.\n\
        .##..#...#.\n\
        .......##..\n\
        .##.#.####.\n\
        .##..##.##.\n\
        ...........\
    ";
}

impl FromStr for Tile {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Self::Garden),
            "#" => Ok(Self::Rock),
            "S" => Ok(Self::Start),
            _ => Err(format!("Invalid tile: '{}'", s))
        }
    }
}