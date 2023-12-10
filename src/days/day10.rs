use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::days::Day;
use crate::util::geometry::{Directions, Grid, Point};

pub const DAY10: Day = Day {
    puzzle1,
    puzzle2
};

fn puzzle1(input: &String) {
    let grid = input.parse::<PipeGrid>().unwrap();

    let result = get_steps_to_furthest_point(&grid).unwrap();
    println!("It takes {} steps to the furthest point in the loop.", result);
}
fn puzzle2(input: &String) {
    let grid = input.parse::<PipeGrid>().unwrap();

    let result = get_tiles_enclosed_by_loop(&grid).unwrap();
    println!("Grid contains {} tiles enclosed in the loop.", result);
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, Default)]
enum Pipe {
    #[default]
    None,       // .
    TopBottom,  // |
    LeftRight,  // -
    LeftTop,    // J
    LeftBottom, // 7
    RightTop,   // L
    RightBottom,// F
    Start,      // S
}

impl Pipe {
    fn can_enter(&self, towards: Directions) -> bool {
        match self {
            Self::None => false,
            Self::Start => true,
            Self::TopBottom => towards == Directions::Bottom || towards == Directions::Top,
            Self::LeftRight => towards == Directions::Left || towards == Directions::Right,
            Self::LeftTop => towards == Directions::Right || towards == Directions::Bottom,
            Self::LeftBottom => towards == Directions::Right || towards == Directions::Top,
            Self::RightTop => towards == Directions::Left || towards == Directions::Bottom,
            Self::RightBottom => towards == Directions::Left || towards == Directions::Top,
        }
    }

    fn get_next_direction(&self, towards: Directions) -> Option<Directions> {
        if !self.can_enter(towards) { return None }
        match self {
            Self::None => None,
            Self::Start => None,
            Self::TopBottom => Some(towards),
            Self::LeftRight => Some(towards),
            Self::LeftTop => if towards == Directions::Right { Some(Directions::Top) } else { Some(Directions::Left) }
            Self::LeftBottom => if towards == Directions::Right { Some(Directions::Bottom) } else { Some(Directions::Left) }
            Self::RightTop => if towards == Directions::Left { Some(Directions::Top) } else { Some(Directions::Right) }
            Self::RightBottom => if towards == Directions::Left { Some(Directions::Bottom) } else { Some(Directions::Right) }
        }
    }
}

type PipeGrid = Grid<Pipe>;

fn get_pipes_in_loop(grid: &PipeGrid) -> Result<Vec<(Point, Pipe)>, String> {
    let start = match grid.entries().iter().find(|(_, pipe)| Pipe::Start.eq(pipe)) {
        Some((point, _)) => point.clone(),
        None => return Err(format!("Could not find a start node in the grid"))
    };

    // start is the only node that can connect on four sides. As mentioned, only two can be followed
    // so we just take off in the first one that connects.
    let mut next_direction =
        if grid.get_adjacent(&start, Directions::Top).first().is_some_and(|p| p.can_enter(Directions::Top)) {
            Directions::Top
        } else if grid.get_adjacent(&start, Directions::Right).first().is_some_and(|p| p.can_enter(Directions::Right)) {
            Directions::Right
        } else if grid.get_adjacent(&start, Directions::Bottom).first().is_some_and(|p| p.can_enter(Directions::Bottom)) {
            Directions::Bottom
        } else if grid.get_adjacent(&start, Directions::Left).first().is_some_and(|p| p.can_enter(Directions::Left)) {
            Directions::Left
        } else {
            return Err(format!("Could not move from start node?!"))
        };

    let mut result = vec![(start, Pipe::Start)];
    let (mut current_point, mut current_pipe) = grid.get_adjacent_entries(&start, next_direction).first().ok_or(format!("Couldn't traverse"))?;

    while current_point.ne(&start) {
        result.push((current_point.clone(), current_pipe.clone()));
        next_direction = current_pipe.get_next_direction(next_direction).ok_or(format!("Could not traverse pipe"))?;
        (current_point, current_pipe) = grid.get_adjacent_entries(&current_point, next_direction).first().cloned().ok_or(format!("Could not find next pipe"))?;
    }

    Ok(result)
}

fn get_steps_to_furthest_point(grid: &PipeGrid) -> Result<usize, String> {
    // In both the samples and my real input, there is only two valid ways from the start point.
    // From there on it's just following the trail until we're round to get the total length of the
    // pipe.
    let pipes = get_pipes_in_loop(grid)?;
    let length = pipes.len();

    // Make sure to take the middle in case length / 2 rounds down
    // E.g. 15 steps => 15/2 = 7, but we need the middle point which is 8
    Ok(length / 2 + length % 2)
}

fn get_tiles_enclosed_by_loop(grid: &PipeGrid) -> Result<usize, String> {
    // We need to find tiles that are inside the loop. With a noteworthy note that two pipes next
    // to each other can be slipped by. As such, the tiles should be on the inside of the loop, and
    // not between outsides.
    // It might not be super efficient, but it might work storing a one-way trip through the pipe,
    // going clock-wise, and for every cell check if we can find pipes traversed in the corresponding
    // way above (to right), below (to left), right (to bottom), and left (to top).
    // To ensure we go through the loop clockwise, we find the top-left F-section and go right.
    let pipes = get_pipes_in_loop(grid)?;
    let (start, _) = pipes.iter().min_by_key(|(point, _)| point).ok_or(format!("Could not find a pipe?!"))?;

    let mut directional_map: Grid<HashSet<Directions>> = Grid::empty();
    directional_map.set(start.clone(), HashSet::from([Directions::Right]));
    let mut next_direction = Directions::Right; // Start by going right.
    let (mut current_point, mut current_pipe) = grid.get_adjacent_entries(&start, next_direction).first().ok_or(format!("Couldn't traverse"))?;

    fn get_next_direction(pipe: Pipe, point: Point, towards: Directions, grid: &PipeGrid) -> Option<Directions> {
        if pipe.eq(&Pipe::None) { None }
        else if pipe.ne(&Pipe::Start) { pipe.get_next_direction(towards) }
        else {
            // Find the other pipe connecting to start, and continue in that direction:
            let mut options = vec![];
            // If going towards top, we're coming from bottom, etc.
            if towards.ne(&Directions::Top) { options.push(Directions::Bottom) }
            if towards.ne(&Directions::Right) { options.push(Directions::Left) }
            if towards.ne(&Directions::Bottom) { options.push(Directions::Top) }
            if towards.ne(&Directions::Left) { options.push(Directions::Right) }
            options.into_iter().filter_map(|d| grid.get_adjacent(&point, d.clone()).first().map(|p| (p.clone(), d)))
                .filter(|(p, d)| p.can_enter(d.clone()))
                .next().map(|(_, d)| d)
        }
    }

    while current_point.ne(&start) {
        let first_direction = next_direction;
        next_direction = get_next_direction(current_pipe, current_point, next_direction, grid).ok_or(format!("Could not traverse pipe {}", current_pipe))?;
        directional_map.set(current_point, HashSet::from([first_direction, next_direction]));
        (current_point, current_pipe) = grid.get_adjacent_entries(&current_point, next_direction).first().cloned().ok_or(format!("Could not find next pipe"))?;
    }

    fn is_enclosed(map: &Grid<HashSet<Directions>>, point: &Point) -> bool {
        map.get_in_direction(point, Directions::Top).first().is_some_and(|d| d.contains(&Directions::Right)) &&
            map.get_in_direction(point, Directions::Right).first().is_some_and(|d| d.contains(&Directions::Bottom)) &&
            map.get_in_direction(point, Directions::Bottom).first().is_some_and(|d| d.contains(&Directions::Left)) &&
            map.get_in_direction(point, Directions::Left).first().is_some_and(|d| d.contains(&Directions::Top))
    }

    Ok(directional_map.points()
        .iter()
        // Take all points that are not part of the loop:
        .filter(|p| pipes.iter().all(|(pp, _)| pp.ne(p)))
        .filter(|p| is_enclosed(&directional_map, p))
        .count())
}

#[cfg(test)]
mod tests {
    use crate::days::day10::{get_steps_to_furthest_point, get_tiles_enclosed_by_loop, PipeGrid};
    use crate::util::geometry::Bounds;

    #[test]
    fn test_parse_and_fmt() {
        let result = TEST_INPUT.parse::<PipeGrid>();
        assert!(result.is_ok(), "Expected Ok, got Err({})", result.err().unwrap());

        let grid = result.unwrap();
        assert_eq!(grid.bounds, Bounds { top: 0, left: 0, width: 5, height: 5 });
        assert_eq!(format!("{}", grid), "\
            ┐─┌┐─\n\
            ▪┌┘│┐\n\
            ◎┘└└┐\n\
            │┌──┘\n\
            └┘▪└┘\
        ");
    }

    #[test]
    fn test_get_steps_to_furthest_point() {
        let grid = TEST_INPUT.parse::<PipeGrid>().unwrap();

        assert_eq!(get_steps_to_furthest_point(&grid), Ok(8));
    }

    #[test]
    fn test_get_tiles_enclosed_by_loop() {
        let grid = TEST_INPUT_NEST_1.parse::<PipeGrid>().unwrap();
        assert_eq!(get_tiles_enclosed_by_loop(&grid), Ok(4));

        let grid = TEST_INPUT_NEST_2.parse::<PipeGrid>().unwrap();
        assert_eq!(get_tiles_enclosed_by_loop(&grid), Ok(8));

        let grid = TEST_INPUT_NEST_3.parse::<PipeGrid>().unwrap();
        assert_eq!(get_tiles_enclosed_by_loop(&grid), Ok(10));
    }

    const TEST_INPUT: &str = "\
        7-F7-\n\
        .FJ|7\n\
        SJLL7\n\
        |F--J\n\
        LJ.LJ\
    ";

    const TEST_INPUT_NEST_1: &str= "\
        ...........\n\
        .S-------7.\n\
        .|F-----7|.\n\
        .||.....||.\n\
        .||.....||.\n\
        .|L-7.F-J|.\n\
        .|..|.|..|.\n\
        .L--J.L--J.\n\
        ...........\
    ";

    const TEST_INPUT_NEST_2: &str = "\
        .F----7F7F7F7F-7....\n\
        .|F--7||||||||FJ....\n\
        .||.FJ||||||||L7....\n\
        FJL7L7LJLJ||LJ.L-7..\n\
        L--J.L7...LJS7F-7L7.\n\
        ....F-J..F7FJ|L7L7L7\n\
        ....L7.F7||L7|.L7L7|\n\
        .....|FJLJ|FJ|F7|.LJ\n\
        ....FJL-7.||.||||...\n\
        ....L---J.LJ.LJLJ...\
    ";

    const TEST_INPUT_NEST_3: &str = "\
        FF7FSF7F7F7F7F7F---7\n\
        L|LJ||||||||||||F--J\n\
        FL-7LJLJ||||||LJL-77\n\
        F--JF--7||LJLJ7F7FJ-\n\
        L---JF-JLJ.||-FJLJJ7\n\
        |F|F-JF---7F7-L7L|7|\n\
        |FFJF7L7F-JF7|JL---7\n\
        7-L-JL7||F7|L7F-7F7|\n\
        L.L7LFJ|||||FJL7||LJ\n\
        L7JLJL-JLJLJL--JLJ.L\
    ";
}

impl FromStr for Pipe {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars = s.chars().collect::<Vec<_>>();
        match chars.len() {
            0 => Err(format!("Cannot create pipe segment from empty string")),
            1 => Pipe::try_from(chars[0]),
            _ => Err(format!("Can only create pipe segment from single character"))
        }
    }
}

impl TryFrom<char> for Pipe {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::None),
            '|' => Ok(Self::TopBottom),
            '-' => Ok(Self::LeftRight),
            'J' => Ok(Self::LeftTop),
            '7' => Ok(Self::LeftBottom),
            'L' => Ok(Self::RightTop),
            'F' => Ok(Self::RightBottom),
            'S' => Ok(Self::Start),
            _ => Err(format!("Invalid pipe char: '{}'", value))
        }
    }
}

impl Display for Pipe {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "▪"),
            Self::TopBottom => write!(f, "│"),
            Self::LeftRight => write!(f, "─"),
            Self::LeftTop => write!(f, "┘"),
            Self::LeftBottom => write!(f, "┐"),
            Self::RightTop => write!(f, "└"),
            Self::RightBottom => write!(f, "┌"),
            Self::Start => write!(f, "◎"),
        }
    }
}