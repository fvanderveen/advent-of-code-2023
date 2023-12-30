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
    let garden: Garden = input.parse().unwrap();
    println!("There are {} tiles reachable with 26501365 steps", garden.get_tiles_within(26501365));
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
    fn get_tiles_from(&self, start: Point, num_steps: usize, odd_tiles: bool, overflow: bool) -> usize {
        // Note: we don't wrap, if num_steps is big enough, will just count all tiles from the start point.
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
                    x: if overflow { ((next_point.x % width) + width) % width } else { next_point.x },
                    y: if overflow { ((next_point.y % height) + height) % height } else { next_point.y },
                };

                if let Some(tile) = self.get(&remapped_point) {
                    if tile != Tile::Rock {
                        queue.push_back((next_point, current_steps + 1))
                    }
                }
            }
        }

        // We want to determine all distances that match the even-ness of the target
        distances.values().filter(|l| ((*l % 2) == 0) != odd_tiles).count()
    }

    fn get_tiles_within(&self, num_steps: usize) -> usize {
        // Tiles probably differ odd/even, so we need to get a full odd and full even tile count,
        // then (if num_steps > map size) we need to compute the top, left, right, bottom, and corner tile counts
        // (a manhattan distance circle is a square, so all NE corners will be the same, etc)
        // Also, the route from S to the edges is free in the (real) map, as well as all edges, simplifying the problem.

        // While this won't work for the test input, it's easier to deal with for the real map.
        // Assumptions:
        // - Square map
        // - Empty borders
        // - Start in the middle
        // - Free road from start to edges (shortest path == manhattan distance)
        // Plan:
        // - Determine how many full maps fit in the steps
        //   - Of these, compute how many will be odd vs even, and figure the amount of tiles on each.
        // - Compute the north, east, south and west most maps based on the remainder of the steps, from the middle entry point
        // - Figure out the corner maps in a similar way (entering from the closest corner)

        // Real map stats:
        // 131 x 131, 65 tiles from start to edge, 130 tiles from start to corner
        // 26501365 steps = (202300 * 131) + 65 (exactly goes to the top of a map even...)
        // This (also) means one kind of corner map per side.

        let start = self.entries().iter().find(|(_, t)| Tile::Start.eq(t)).unwrap().0;
        let corner_distance = start.manhattan_distance(&(0, 0).into()) as usize; // Assumptions: square map and start in middle
        let map_length = self.bounds.width;
        let steps_odd = (num_steps % 2) != 0;

        if num_steps < corner_distance {
            // Puzzle 1
            return self.get_tiles_from(start, num_steps, steps_odd, true)
        }

        // Puzzle 2
        let odd_count = self.get_tiles_from(start, usize::MAX, true, false);
        let even_count = self.get_tiles_from(start, usize::MAX, false, false);

        // Number of full maps (left/right/top/bottom)
        let full_maps = (num_steps - corner_distance) / map_length;

        println!("We can fit {} full maps in any direction, even maps have {} tiles, odd maps {}", full_maps, odd_count, even_count);

        // The initial map should be the same odd/even as the number of steps, the second the other, etc.
        // Full maps form a square:
        // ...E...
        // ..EOE..
        // .EOEOE.
        // EOEOEOE
        // .EOEOE.
        // ..EOE..
        // ...E...
        // 1st full map = 1, 2nd = 4, 3rd = 8, 4th = 12 (4*N, N = full map steps (first = 0))
        // Similarly, the number of edge maps on each edge is N-1 (as the outermost N/E/S/W maps are different from the edges)

        let mut main_squares: usize = 1; // Number of squares where we need the same oddity for
        let mut alt_squares = 0; // Number of squares where the oddity is flipped

        let mut j = 0;
        for i in 0..=full_maps {
            if i % 2 == 0 { main_squares += j } else { alt_squares += j }
            j += 4
        }

        let main_tiles = if steps_odd { odd_count } else { even_count };
        let alt_tiles = if steps_odd { even_count } else {odd_count};

        let mut total_tiles = main_squares * main_tiles + alt_squares * alt_tiles;

        // For the left/right/top/bottom most maps, we compute the amount of tiles using the left-over steps from the center point
        // of the inner edge.
        let end_step = full_maps + 1;
        let steps_left = num_steps - (((end_step - 1) * map_length) + start.x as usize);
        println!("There are {} steps left for end-points", steps_left);
        let end_odd = (steps_odd && (end_step % 2) == 0) || (!steps_odd && (end_step % 2) == 1);

        total_tiles += self.get_tiles_from((start.x, map_length as isize - 1).into(), steps_left, !end_odd, false); // Top
        total_tiles += self.get_tiles_from((map_length as isize - 1, start.y).into(), steps_left, !end_odd, false); // Right
        total_tiles += self.get_tiles_from((start.x, 0).into(), steps_left, !end_odd, false); // Bottom
        total_tiles += self.get_tiles_from((0, start.y).into(), steps_left, !end_odd, false); // Left

        // For the NE/NW/SW/SE maps, we compute using the left-over steps from the innermost corner point
        // We need two different corner kinds, one to fill the row between two ends, and one to fill the outer edge
        // (We end up at the north-most map with 131 steps from the bottom-middle, which means we can still venture
        //  into the map right of that with 131 - 65 steps (=66) steps. (but not the map above))
        let corner_count = end_step - 1; // N sides, but we already covered one
        let tiles_left_large = num_steps - (corner_distance + ((end_step - 2) * map_length)) - 2;
        let tiles_left_small = num_steps - (corner_distance + ((end_step - 1) * map_length)) - 2;

        println!("There are {}/{} steps left for corners, and {}/{} corners", tiles_left_large, tiles_left_small, corner_count, corner_count + 1);

        total_tiles += corner_count * self.get_tiles_from((0, map_length as isize - 1).into(), tiles_left_large, end_odd, false); // NE
        total_tiles += (corner_count + 1) * self.get_tiles_from((0, map_length as isize - 1).into(), tiles_left_small, !end_odd, false); // NE
        total_tiles += corner_count * self.get_tiles_from((0, 0).into(), tiles_left_large, end_odd, false); // SE
        total_tiles += (corner_count + 1) * self.get_tiles_from((0, 0).into(), tiles_left_small, !end_odd, false); // SE
        total_tiles += corner_count * self.get_tiles_from((map_length as isize - 1, 0).into(), tiles_left_large, end_odd, false); // SW
        total_tiles += (corner_count + 1) * self.get_tiles_from((map_length as isize - 1, 0).into(), tiles_left_small, !end_odd, false); // SW
        total_tiles += corner_count * self.get_tiles_from((map_length as isize - 1, map_length as isize - 1).into(), tiles_left_large, end_odd, false); // NW
        total_tiles += (corner_count + 1) * self.get_tiles_from((map_length as isize - 1, map_length as isize - 1).into(), tiles_left_small, !end_odd, false); // NW

        total_tiles
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day21::Garden;

    #[test]
    fn test_get_tiles_within() {
        let garden: Garden = TEST_INPUT.parse().unwrap();

        assert_eq!(garden.get_tiles_within(6), 16);
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