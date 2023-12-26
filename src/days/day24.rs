use std::ops::RangeInclusive;
use std::str::FromStr;
use crate::days::Day;
use crate::util::geometry::{Line, Point3D};

pub const DAY24: Day = Day {
    puzzle1,
    puzzle2
};

fn puzzle1(input: &String) {
    let stones = parse_input(input).unwrap();
    let area = 200_000_000_000_000f64..=400_000_000_000_000f64;

    println!("Number of intersections in target area: {}", Hailstone::count_2d_intersections(&stones, &area));
}
fn puzzle2(input: &String) {
    todo!("Implement puzzle 2");
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
struct Hailstone {
    position: Point3D,
    velocity: Point3D
}

impl Hailstone {
    fn future_line_2d(&self) -> Line {
        Line {
            start: (self.position.x, self.position.y).into(),
            // Note: line needs to be long enough for the real data to properly compute intersections...
            end: (self.position.x + (self.velocity.x * 1000), self.position.y + (self.velocity.y * 1000)).into()
        }
    }

    fn intersects_2d(&self, other: &Self, area: &RangeInclusive<f64>) -> bool {
        fn in_future(stone: &Hailstone, x: f64, y: f64) -> bool {
            let sx = stone.position.x as f64;
            let sy = stone.position.y as f64;

            // x/y is in the future if:
            // x/y > start and dx/dy > 0
            // x/y < start and dx/dy < 0
            let x_future = x == sx || if x > sx { stone.velocity.x > 0 } else { stone.velocity.x < 0 };
            let y_future = y == sy || if y > sy { stone.velocity.y > 0 } else { stone.velocity.y < 0 };
            x_future && y_future
        }

        // Puzzle 1 wants to find all hailstones that intersect on the x,y line in the future and inside the area.
        // We'll find the x/y intersection first (if any), and if it's in the area, validate it's actually in the future.
        if let Some((x, y)) = self.future_line_2d().intersection(&other.future_line_2d()) {
            area.contains(&x) && area.contains(&y) && in_future(self, x, y) && in_future(other, x, y)
        } else {
            false
        }
    }

    fn count_2d_intersections(stones: &Vec<Self>, area: &RangeInclusive<f64>) -> usize {
        let mut count = 0;
        for a in 0..stones.len() {
            for b in a+1..stones.len() {
                if stones[a].intersects_2d(&stones[b], area) { count+=1; }
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day24::{Hailstone, parse_input};

    #[test]
    fn test_intersects_2d() {
        let stones = parse_input(TEST_INPUT).unwrap();
        let test_range = 7f64..=27f64;

        assert_eq!(stones[0].intersects_2d(&stones[1], &test_range), true);
        assert_eq!(stones[0].intersects_2d(&stones[2], &test_range), true);
        assert_eq!(stones[0].intersects_2d(&stones[3], &test_range), false);
    }

    #[test]
    fn test_count_2d_intersections() {
        let stones = parse_input(TEST_INPUT).unwrap();
        let test_range = 7f64..=27f64;

        assert_eq!(Hailstone::count_2d_intersections(&stones, &test_range), 2);
    }

    const TEST_INPUT: &str = "\
        19, 13, 30 @ -2,  1, -2\n\
        18, 19, 22 @ -1, -1, -2\n\
        20, 25, 34 @ -2, -2, -4\n\
        12, 31, 28 @ -1, -2, -1\n\
        20, 19, 15 @  1, -5, -3\n\
    ";
}

fn parse_input(input: &str) -> Result<Vec<Hailstone>, String> {
    input.lines().map(|l| l.parse()).collect()
}

impl FromStr for Hailstone {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [pos_str, vel_str] = match s.split('@').collect::<Vec<_>>()[..] {
            [pos_str, vel_str] => Ok([pos_str, vel_str]),
            _ => Err(format!("Could not parse hailstone '{}'", s))
        }?;

        Ok(Self {
            position: pos_str.parse()?,
            velocity: vel_str.parse()?,
        })
    }
}