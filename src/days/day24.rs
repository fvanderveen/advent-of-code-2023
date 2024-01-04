use std::ops::RangeInclusive;
use std::str::FromStr;
use crate::days::Day;
use crate::util::geometry::Point3D;

pub const DAY24: Day = Day {
    puzzle1,
    puzzle2,
};

fn puzzle1(input: &String) {
    let stones = parse_input(input).unwrap();
    let area = 200_000_000_000_000f64..=400_000_000_000_000f64;

    println!("Number of intersections in target area: {}", Hailstone::count_2d_intersections(&stones, &area));
}

fn puzzle2(input: &String) {
    let stones = parse_input(input).unwrap();

    let stone = Hailstone::find_stone_hitting_all(&stones).unwrap();
    println!("Stone hitting all hailstones: {:?}, result: {}", stone, stone.position.x + stone.position.y + stone.position.z);
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
struct Hailstone {
    position: Point3D,
    velocity: Point3D,
}

impl Hailstone {
    fn dydx_slope(&self) -> f64 {
        if self.velocity.x == 0 {
            usize::MAX as f64
        } else {
            self.velocity.y as f64 / self.velocity.x as f64
        }
    }

    fn c_xy(&self) -> f64 {
        self.position.y as f64 - self.dydx_slope() * self.position.x as f64
    }

    fn dzdx_slope(&self) -> f64 {
        if self.velocity.x == 0 {
            usize::MAX as f64
        } else {
            self.velocity.z as f64 / self.velocity.x as f64
        }
    }

    fn c_xz(&self) -> f64 {
        self.position.z as f64 - self.dzdx_slope() * self.position.x as f64
    }

    fn intersection_xy(&self, other: &Self) -> Option<(f64, f64)> {
        let m1 = self.dydx_slope();
        let c1 = self.c_xy();
        let m2 = other.dydx_slope();
        let c2 = other.c_xy();

        if m1 == m2 { return None; } // parallel

        let x = (c2 - c1) / (m1 - m2);
        let y = m1 * x + c1;

        let t_self = (x - self.position.x as f64) / self.velocity.x as f64;
        let t_other = (x - other.position.x as f64) / other.velocity.x as f64;

        if t_self < 0f64 || t_other < 0f64 {
            None // Intersection in past
        } else {
            Some((x, y))
        }
    }

    fn intersection_xz(&self, other: &Self) -> Option<(f64, f64)> {
        let m1 = self.dzdx_slope();
        let c1 = self.c_xz();
        let m2 = other.dzdx_slope();
        let c2 = other.c_xz();

        if m1 == m2 { return None; } // parallel

        let x = (c2 - c1) / (m1 - m2);
        let y = m1 * x + c1;

        let t_self = (x - self.position.x as f64) / self.velocity.x as f64;
        let t_other = (x - other.position.x as f64) / other.velocity.x as f64;

        if t_self < 0f64 || t_other < 0f64 {
            None // Intersection in past
        } else {
            Some((x, y))
        }
    }

    fn intersects_2d(&self, other: &Self, area: &RangeInclusive<f64>) -> bool {
        // Puzzle 1 wants to find all hailstones that intersect on the x,y line in the future and inside the area.
        // We'll find the x/y intersection first (if any), and if it's in the area, validate it's actually in the future.
        if let Some((x, y)) = self.intersection_xy(other) {
            area.contains(&x) && area.contains(&y)
        } else {
            false
        }
    }

    fn count_2d_intersections(stones: &Vec<Self>, area: &RangeInclusive<f64>) -> usize {
        let mut count = 0;
        for a in 0..stones.len() {
            for b in a + 1..stones.len() {
                if stones[a].intersects_2d(&stones[b], area) { count += 1; }
            }
        }
        count
    }

    fn find_stone_hitting_all(stones: &Vec<Self>) -> Option<Hailstone> {
        // To find the stone offset and velocity, we simply brute force all velocities.
        // To eliminate the time factor, we map all (or a subset, at least) hailstones to subtract the test velocity (x,y).
        // If we can find a point where the mapped stones hit each other, we have a candidate. From there, we do the
        // same by adding the z velocity, and seeing if we can find an intersection for all stones there as well.
        // The intersection point (x,y,z) is the offset we need our stone to be at.

        fn find_intersection(stones: &Vec<Hailstone>, intersect: impl (Fn(&Hailstone, &Hailstone) -> Option<(f64, f64)>)) -> Option<(f64, f64)> {
            // We need an integer position, and as such, an integer intersection.
            let main = stones[0];
            let mut current_point: Option<(f64, f64)> = None;

            for stone in &stones[1..] {
                let (a, b) = intersect(&main, stone)?;

                if let Some((cur_a, cur_b)) = current_point {
                    if (cur_a - a).abs() > 1f64 || (cur_b - b).abs() > 1f64 {
                        // Not all lines match in one point
                        return None;
                    }
                } else {
                    current_point = Some((a, b))
                }
            }

            current_point
        }

        const USE_STONES: usize = 10; // Number of stones to validate
        const MAX_Z: isize = 10000; // Bind the maximum Z value to search, in case we have a x/y match with no Z match
        // Note: based on velocities in the input, which seem < 1000, so I don't expect a very large Z necessary.

        for i in 0..isize::MAX {
            if i % 100 == 0 { println!("{}...", i); }

            for j in 0..=i {
                for [x, y] in [[i, j], [j, i]] {
                    for [sx, sy] in [[1, 1], [1, -1], [-1, 1], [-1, -1]] {
                        let rock_dx = sx * x;
                        let rock_dy = sy * y;

                        // println!("Testing dx/dy {}/{}", rock_dx, rock_dy);

                        // Remap set of hailstones to subtract the test velocity
                        let xy_stones = stones.iter()
                            .take(USE_STONES)
                            .map(|s| Hailstone {
                                position: s.position,
                                velocity: Point3D { x: s.velocity.x - rock_dx, y: s.velocity.y - rock_dy, z: s.velocity.z },
                            }).collect::<Vec<_>>();

                        let (x, y) = match find_intersection(&xy_stones, |a, b| a.intersection_xy(b)) {
                            Some(p) => p,
                            None => continue, // No match, continue
                        };

                        println!("Found hit for {},{}", x, y);

                        for z in 0..MAX_Z {
                            for sz in [-1, 1] {
                                let rock_dz = sz * z;

                                let xz_stones = stones.iter()
                                    .take(USE_STONES)
                                    .map(|s| Hailstone {
                                        position: s.position,
                                        velocity: Point3D { x: s.velocity.x - rock_dx, y: s.velocity.y, z: s.velocity.z - rock_dz },
                                    })
                                    .collect::<Vec<_>>();

                                let (_, z) = match find_intersection(&xz_stones, |a, b| a.intersection_xz(b)) {
                                    Some(p) => p,
                                    None => continue, // No match, continue
                                };

                                println!("Found z {}", z);

                                // We got all data:
                                return Some(Hailstone {
                                    position: Point3D { x: x as isize, y: y as isize, z: z as isize },
                                    velocity: Point3D { x: rock_dx, y: rock_dy, z: rock_dz },
                                });
                            }
                        }

                        println!("No hit for z <= 5000?");
                    }
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day24::{Hailstone, parse_input};
    use crate::util::geometry::Point3D;

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

    #[test]
    fn test_find_stone_hitting_all() {
        let stones = parse_input(TEST_INPUT).unwrap();

        assert_eq!(Hailstone::find_stone_hitting_all(&stones), Some(Hailstone {
            position: Point3D { x: 24, y: 13, z: 10 },
            velocity: Point3D { x: -3, y: 1, z: 2 },
        }))
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