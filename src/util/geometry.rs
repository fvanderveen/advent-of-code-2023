// Allow dead_code since this is a util file copied across years. Later in the AoC we might use everything, or not.
#![allow(dead_code)]

use std::cmp::{max, Ordering};
use std::collections::HashMap;
use std::{cmp, fmt};
use std::hash::Hash;
use std::ops::{Add, RangeInclusive};
use std::str::FromStr;
use num_traits::abs;
use crate::util::number;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub fn get_points_around(&self, directions: Directions) -> Vec<Point> {
        let mut points = vec![];
        if directions.has(Directions::TopLeft) { points.push((self.x - 1, self.y - 1).into()) }
        if directions.has(Directions::Top) { points.push((self.x, self.y - 1).into()) }
        if directions.has(Directions::TopRight) { points.push((self.x + 1, self.y - 1).into()) }
        if directions.has(Directions::Right) { points.push((self.x + 1, self.y).into()) }
        if directions.has(Directions::BottomRight) { points.push((self.x + 1, self.y + 1).into()) }
        if directions.has(Directions::Bottom) { points.push((self.x, self.y + 1).into()) }
        if directions.has(Directions::BottomLeft) { points.push((self.x - 1, self.y + 1).into()) }
        if directions.has(Directions::Left) { points.push((self.x - 1, self.y).into()) }

        return points;
    }

    pub fn manhattan_distance(&self, other: &Point) -> isize {
        abs(self.x - other.x) + abs(self.y - other.y)
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl From<(isize, isize)> for Point {
    fn from(p: (isize, isize)) -> Self {
        Point { x: p.0, y: p.1 }
    }
}

impl TryFrom<(usize, usize)> for Point {
    type Error = String;

    fn try_from(value: (usize, usize)) -> Result<Self, Self::Error> {
        let x: isize = isize::try_from(value.0).map_err(|e| format!("{}", e))?;
        let y: isize = isize::try_from(value.1).map_err(|e| format!("{}", e))?;
        Ok(Point { x, y })
    }
}

impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts_result: Result<Vec<isize>, String> = s.split(",").map(|p| number::parse_isize(p)).collect();
        let parts = match parts_result {
            Ok(v) => v,
            Err(e) => return Err(e)
        };
        match parts.len() {
            2 => Ok((parts[0], parts[1]).into()),
            _ => Err(format!("Invalid str format for Point '{}', expected 'x,y'", s))
        }
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        self.y.cmp(&other.y)
            .then_with(|| self.x.cmp(&other.x))
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Add<&Point> for Point {
    type Output = Point;

    fn add(self, rhs: &Point) -> Self::Output {
        Point { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}
impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        self + &rhs
    }
}
impl Add<(isize, isize)> for Point {
    type Output = Point;

    fn add(self, rhs: (isize, isize)) -> Self::Output {
        self + Point::from(rhs)
    }
}
impl Add<Point> for Vec<Point> {
    type Output = Vec<Point>;

    fn add(self, rhs: Point) -> Self::Output {
        self.iter().map(|p| rhs + p).collect()
    }
}

#[cfg(test)]
mod point_tests {
    use crate::util::geometry::{Directions, Point};

    #[test]
    fn test_from_str() {
        assert_eq!("3,5".parse(), Ok(Point { x: 3, y: 5 }));
        assert_eq!("3,-5".parse(), Ok(Point { x: 3, y: -5 }));
        assert_eq!("422,-2345".parse(), Ok(Point { x: 422, y: -2345 }));
    }

    #[test]
    fn test_from() {
        assert_eq!(Point::from((3, 5)), Point { x: 3, y: 5 });
        assert_eq!(Point::from((-42, -10)), Point { x: -42, y: -10 });
    }

    #[test]
    fn test_format() {
        assert_eq!(format!("{}", Point { x: 5, y: -10 }), "(5,-10)");
    }
    
    #[test]
    fn test_ord() {
        let mut points = vec![
            Point { x: 12, y: 1 },
            Point { x: 2, y: 5},
            Point { x: 4, y: 3 },
            Point { x: 1, y: 1 },
            Point { x: 5, y: 3 }
        ];
        points.sort();
        assert_eq!(points, vec![
            Point { x: 1, y: 1 },
            Point { x: 12, y: 1 },
            Point { x: 4, y: 3 },
            Point { x: 5, y: 3 },
            Point { x: 2, y: 5},
        ]);
    }

    #[test]
    fn test_get_points_around() {
        assert_eq!(Point::from((3, 2)).get_points_around(Directions::NonDiagonal), vec![(3,1).into(), (4,2).into(), (3,3).into(), (2,2).into()]);
        assert_eq!(Point::from((3, 2)).get_points_around(Directions::Diagonal), vec![(2,1).into(), (4,1).into(), (4,3).into(), (2,3).into()]);
    }

    #[test]
    fn test_manhattan_distance() {
        let point_a = Point { x: 1, y: 2 };
        let point_b = Point { x: 13, y: 4 };
        assert_eq!(point_a.manhattan_distance(&point_b), 14);
        assert_eq!(point_b.manhattan_distance(&point_a), 14);
    }
}


#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Hash)]
pub struct Point3D {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

impl fmt::Display for Point3D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

impl From<(isize, isize, isize)> for Point3D {
    fn from((x,y,z): (isize, isize, isize)) -> Self {
        Self { x, y, z }
    }
}

impl From<Point> for Point3D {
    fn from(p: Point) -> Self {
        Self { x: p.x, y: p.y, ..Self::default() }
    }
}

impl FromStr for Point3D {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let points = s.split(",").map(|p| number::parse_isize(p)).collect::<Result<Vec<isize>, String>>()?;
        if points.len() != 3 {
            Err(format!("Expected three coordinates, but got {}", points.len()))
        } else {
            Ok(Point3D { x: points[0], y: points[1], z: points[2] })
        }
    }
}

impl PartialOrd<Self> for Point3D {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point3D {
    fn cmp(&self, other: &Self) -> Ordering {
        self.x.cmp(&other.x)
            .then_with(|| self.y.cmp(&other.y))
            .then_with(|| self.z.cmp(&other.z))
    }
}

impl Point3D {
    pub fn distance(&self, other: &Self) -> Self {
        Point3D {
            x: other.x - self.x,
            y: other.y - self.y,
            z: other.z - self.z,
        }
    }

    pub fn manhattan(&self, other: &Self) -> usize {
        let x = (self.x - other.x).abs();
        let y = (self.y - other.y).abs();
        let z = (self.z - other.z).abs();
        return (x + y + z) as usize;
    }

    pub fn translate(&self, other: &Self) -> Self {
        Point3D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    pub fn get_points_around(&self) -> Vec<Point3D> {
        let mut points = vec![];

        for z in -1..=1 {
            for y in -1..=1 {
                for x in -1..=1 {
                    if x == 0 && y == 0 && z == 0 { continue; }
                    points.push((self.x + x,self.y + y, self.z + z).into());
                }
            }
        }

        points
    }
}

#[cfg(test)]
mod point3d_tests {
    use crate::util::geometry::{Point, Point3D};
    #[test]
    fn test_from_str() {
        assert_eq!("3,5,2".parse(), Ok(Point3D { x: 3, y: 5, z: 2 }));
        assert_eq!("3,-5,0".parse(), Ok(Point3D { x: 3, y: -5, z:0 }));
        assert_eq!("422,-2345,-99".parse(), Ok(Point3D { x: 422, y: -2345, z: -99 }));
    }

    #[test]
    fn test_from() {
        assert_eq!(Point3D::from((3, 5, 42)), Point3D { x: 3, y: 5, z: 42 });
        assert_eq!(Point3D::from((-42, -10, -211)), Point3D { x: -42, y: -10, z: -211 });
        assert_eq!(Point3D::from(Point { x: 10, y: -20 }), Point3D { x: 10, y: -20, z: 0 });
    }

    #[test]
    fn test_format() {
        assert_eq!(format!("{}", Point3D { x: 5, y: -10, z: 20 }), "(5,-10,20)");
    }

    #[test]
    fn test_manhattan() {
        assert_eq!(Point3D { x: 1105, y: -1205, z: 1229 }.manhattan(&Point3D { x: -92, y: -2380, z: -20 }), 3621);
        assert_eq!(Point3D { x: -92, y: -2380, z: -20 }.manhattan(&Point3D { x: 1105, y: -1205, z: 1229 }), 3621);
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Line {
    pub start: Point,
    pub end: Point,
}

impl Line {
    fn length(&self) -> usize {
        let x1 = self.start.x;
        let x2 = self.end.x;

        (x1 - x2).abs() as usize
    }

    fn height(&self) -> usize {
        let y1 = self.start.y;
        let y2 = self.end.y;

        (y1 - y2).abs() as usize
    }

    fn dx(&self) -> isize {
        let x1 = self.start.x;
        let x2 = self.end.x;

        if x1 == x2 {
            0
        } else if x2 > x1 {
            1
        } else {
            -1
        }
    }

    fn dy(&self) -> isize {
        let y1 = self.start.y;
        let y2 = self.end.y;

        if y1 == y2 {
            0
        } else if y2 > y1 {
            1
        } else {
            -1
        }
    }

    fn x(&self, t: usize) -> isize {
        let step: isize = (t as isize) * self.dx();
        self.start.x + step
    }

    fn y(&self, t: usize) -> isize {
        let step: isize = (t as isize) * self.dy();
        self.start.y + step
    }

    pub fn get_points(&self) -> Vec<Point> {
        let mut points: Vec<Point> = vec![];

        let length = self.length();
        let height = self.height();

        if length == 0 && height == 0 {
            return points;
        }

        // Given the puzzle, the lines seem to be either horizontal, vertical, or 45deg.
        // We'll panic for any other case for now.
        if length != 0 && height != 0 && length != height {
            panic!("Cannot get points for line {:?}", self);
        }

        let steps = max(length, height);
        for i in 0..steps + 1 {
            points.push((self.x(i), self.y(i)).into());
        }

        points
    }
}


#[cfg(test)]
mod line_tests {
    use crate::util::geometry::{Line, Point};

    const fn point(x: isize, y: isize) -> Point {
        Point { x, y }
    }

    const fn line(x1: isize, y1: isize, x2: isize, y2: isize) -> Line {
        Line { start: point(x1, y1), end: point(x2, y2) }
    }

    #[test]
    fn test_get_points() {
        assert_eq!(line(12, 0, 12, 6).get_points(), vec![point(12, 0), point(12, 1), point(12, 2), point(12, 3), point(12, 4), point(12, 5), point(12, 6)]);
        assert_eq!(line(2, 2, 4, 4).get_points(), vec![point(2, 2), point(3, 3), point(4, 4)]);
        assert_eq!(line(4, 0, 2, 0).get_points(), vec![point(4, 0), point(3, 0), point(2, 0)]);
    }
}


#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
pub struct Bounds {
    pub top: isize,
    pub left: isize,
    pub width: usize,
    pub height: usize,
}

#[allow(unused)]
impl Bounds {
    pub fn from_tlbr(top: isize, left: isize, bottom: isize, right: isize) -> Self {
        Self {
            top,
            left,
            width: (right - left).max(0) as usize + 1,
            height: (bottom - top).max(0) as usize + 1,
        }
    }

    pub fn try_from_tlbr(top: usize, left: usize, bottom: usize, right: usize) -> Result<Self, String> {
        Ok(Self::from_tlbr(
            top.try_into().map_err(|e| format!("{}", e))?,
            left.try_into().map_err(|e| format!("{}", e))?,
            bottom.try_into().map_err(|e| format!("{}", e))?,
            right.try_into().map_err(|e| format!("{}", e))?,
        ))
    }

    pub fn from_size(width: usize, height: usize) -> Self {
        Self { top: 0, left: 0, width, height }
    }
    
    pub fn grow(&mut self, by: isize) {
        self.top -= by;
        self.left -= by;
        self.width = (self.width as isize + 2 * by) as usize;
        self.height = (self.height as isize + 2 * by) as usize
    }

    pub fn y(&self) -> RangeInclusive<isize> {
        self.top..=self.bottom()
    }

    pub fn x(&self) -> RangeInclusive<isize> {
        self.left..=self.right()
    }

    pub fn right(&self) -> isize {
        self.left + self.width as isize - 1
    }

    pub fn bottom(&self) -> isize {
        self.top + self.height as isize - 1
    }

    pub fn contains(&self, pixel: &Point) -> bool {
        self.x().contains(&pixel.x) && self.y().contains(&pixel.y)
    }
    
    pub fn points(&self) -> Vec<Point> {
        let mut points = vec![];
        
        for y in self.y() {
            for x in self.x() {
                points.push((x, y).into());
            }
        }
        
        points
    }
}

#[derive(Eq, PartialEq, Clone)]
pub struct Grid<T> where T: Clone {
    pub bounds: Bounds,
    cells: HashMap<Point, T>,
}

impl<T> Default for Grid<T> where T: Clone + Default {
    fn default() -> Self {
        Grid {
            bounds: Bounds::default(),
            cells: HashMap::default()
        }
    }
}

#[repr(u8)]
#[derive(Eq, PartialEq, Clone, Copy, Debug, Hash)]
pub enum Directions {
    Top = 1,
    Right = 2,
    Bottom = 4,
    Left = 8,
    TopLeft = 16,
    TopRight = 32,
    BottomLeft = 64,
    BottomRight = 128,
    TopAll = Directions::TopLeft as u8 | Directions::Top as u8 | Directions::TopRight as u8,
    BottomAll = Directions::BottomLeft as u8 | Directions::Bottom as u8 | Directions::BottomRight as u8,
    LeftAll = Directions::TopLeft as u8 | Directions::Left as u8 | Directions::BottomLeft as u8,
    RightAll = Directions::TopRight as u8 | Directions::Right as u8 | Directions::BottomRight as u8,
    Diagonal = Directions::TopLeft as u8 | Directions::TopRight as u8 | Directions::BottomLeft as u8 | Directions::BottomRight as u8,
    Horizontal = Directions::Left as u8 | Directions::Right as u8,
    Vertical = Directions::Top as u8 | Directions::Bottom as u8,
    NonDiagonal = Directions::Horizontal as u8 | Directions::Vertical as u8,
    All = Directions::NonDiagonal as u8 | Directions::Diagonal as u8,
}

impl Directions {    
    pub fn has(&self, value: Directions) -> bool {
        (self.clone() as u8 & value as u8) != 0
    }
}

#[allow(unused)]
impl<T> Grid<T> where T: Clone {
    pub fn new(cells: HashMap<Point, T>) -> Self {
        let points: Vec<_> = cells.keys().collect();
        let top = points.iter().map(|p| p.y).min().unwrap_or(0);
        let left = points.iter().map(|p| p.x).min().unwrap_or(0);
        let bottom = points.iter().map(|p| p.y).max().unwrap_or(0);
        let right = points.iter().map(|p| p.x).max().unwrap_or(0);

        let bounds = Bounds::from_tlbr(top, left, bottom, right);
        Self { bounds, cells }
    }
    
    pub fn empty() -> Self {
        Self { bounds: Bounds::default(), cells: HashMap::new() }
    }

    pub fn get(&self, p: &Point) -> Option<T> {
        self.cells.get(p).map(|x| x.clone())
    }
    
    pub fn has(&self, p: &Point) -> bool {
        self.cells.contains_key(p)
    }

    pub fn get_mut(&mut self, p: &Point) -> Option<&mut T> {
        self.cells.get_mut(p)
    }

    pub fn set(&mut self, p: Point, v: T) {
        self.cells.insert(p, v);

        if self.bounds.contains(&p) {
            return;
        }

        // If this is the first insertion, make the bounds set to that point; otherwise expand:
        if self.cells.len() == 1 {
            self.bounds = Bounds::from_tlbr(p.y, p.x, p.y, p.x);
        } else {
            let top = cmp::min(self.bounds.top, p.y);
            let left = cmp::min(self.bounds.left, p.x);
            let bottom = cmp::max(self.bounds.bottom(), p.y);
            let right = cmp::max(self.bounds.right(), p.x);
            self.bounds = Bounds::from_tlbr(top, left, bottom, right);
        }
    }
    
    pub fn get_adjacent(&self, p: &Point, directions: Directions) -> Vec<T> {
        self.get_adjacent_points(p, directions).iter().filter_map(|p| self.get(p)).collect()
    }

    pub fn get_adjacent_points(&self, p: &Point, directions: Directions) -> Vec<Point> {
        p.get_points_around(directions).into_iter().filter(|p| self.bounds.contains(p)).collect()
    }

    pub fn get_adjacent_entries(&self, p: &Point, directions: Directions) -> Vec<(Point, T)> {
        self.get_adjacent_points(p, directions).into_iter().filter_map(|p| self.get(&p).map(|i| (p, i))).collect()
    }

    pub fn get_in_direction(&self, p: &Point, direction: Directions) -> Vec<T> {
        self.get_points_in_direction(p, direction).iter().filter_map(|p| self.get(p)).collect()
    }

    pub fn get_points_in_direction(&self, p: &Point, direction: Directions) -> Vec<Point> {
        match direction {
            Directions::Top | Directions::Right | Directions::Bottom | Directions::Left => {
                let mut points = vec![];
                let mut current = p.clone();
                loop {
                    let next = self.get_adjacent_points(&current, direction.clone());
                    if next.len() != 1 {
                        break;
                    }
                    current = next[0];
                    points.push(current);
                }
                points
            },
            _ => vec![]
        }
    }

    pub fn points(&self) -> Vec<Point> {
        let mut points = vec![];

        for y in self.bounds.y() {
            for x in self.bounds.x() {
                points.push((x, y).into());
            }
        }

        points
    }

    pub fn values(&self) -> Vec<T> {
        self.points().iter().filter_map(|p| self.get(p)).collect()
    }
    
    pub fn entries(&self) -> Vec<(Point, T)> {
        self.cells.iter().map(|(p, t)| (p.clone(), t.clone())).collect()
    }
}

impl<T> fmt::Debug for Grid<T> where T: fmt::Display + Clone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Grid")
            .field("bounds", &self.bounds)
            .field("map", &format_args!("{:,>}", &self))
            .finish()
    }
}

impl<T> fmt::Display for Grid<T> where T: fmt::Display + Clone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut lines = vec![];

        for y in self.bounds.y() {
            let mut line = vec![];
            for x in self.bounds.x() {
                if let Some(val) = self.cells.get(&(x, y).into()) {
                    line.push(format!("{}", val))
                } else {
                    line.push(" ".to_string())
                }
            }
            lines.push(line);
        }

        let cell_width = lines.iter().map(|line| line.iter().map(|v| v.len()).max().unwrap_or(0)).max().unwrap_or(0);

        let fill = f.fill().to_string();
        let formatted_lines: Vec<_> = lines.iter().map(|line| {
            let formatted_line: Vec<_> = line.iter().map(|v| " ".repeat(cell_width - v.len()) + v).collect();
            formatted_line.join(if f.align().is_some() { fill.as_str() } else { "" })
        }).collect();

        write!(f, "{}", formatted_lines.join("\n"))
    }
}

impl<T> FromStr for Grid<T> where T: FromStr + Clone + Default {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parse_result: Result<Vec<Vec<T>>, String> = s.lines()
            .filter(|l| !l.is_empty())
            .map(|l| l.chars().map(|c|
                String::from(c).parse::<T>().map_err(|_| format!("Could not parse '{}' to {}", c, std::any::type_name::<T>())))
                .collect::<Result<Vec<T>, String>>())
            .collect();

        let cells = match parse_result {
            Ok(lines) if lines.len() == 0 => {
                return Ok(Grid::default());
            }
            Ok(lines) => lines,
            Err(e) => return Err(e)
        };

        Grid::try_from(cells)
    }
}

impl<T> TryFrom<Vec<Vec<T>>> for Grid<T> where T: Clone + Default {
    type Error = String;

    fn try_from(data: Vec<Vec<T>>) -> Result<Self, Self::Error> {
        let height = data.len();
        let width = data[0].len();

        let bounds = Bounds { top: 0, left: 0, width, height };

        if data.iter().all(|l| l.len() == width) {
            let mut cells = HashMap::new();
            for y in 0..height {
                for x in 0..width {
                    cells.insert((x, y).try_into().unwrap(), data[y][x].clone());
                }
            }

            Ok(Grid { bounds, cells })
        } else {
            Err(format!("Not all lines in input are the same width"))
        }
    }
}

#[cfg(test)]
mod grid_tests {
    use crate::util::geometry::{Grid, Directions, Bounds};

    const EXAMPLE_GRID_INPUT: &str = "\
        2199943210\n\
        3987894921\n\
        9856789892\n\
        8767896789\n\
        9899965678\
    ";

    fn get_example_grid() -> Grid<usize> {
        vec![
            vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
            vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
            vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
            vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
            vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
        ].try_into().unwrap()
    }

    #[test]
    fn test_grid_debug() {
        assert_eq!(format!("{:?}", get_example_grid()),
                   "Grid { \
                       bounds: Bounds { top: 0, left: 0, width: 10, height: 5 }, \
                       map: 2,1,9,9,9,4,3,2,1,0\n\
                            3,9,8,7,8,9,4,9,2,1\n\
                            9,8,5,6,7,8,9,8,9,2\n\
                            8,7,6,7,8,9,6,7,8,9\n\
                            9,8,9,9,9,6,5,6,7,8 \
                   }");
    }

    #[test]
    fn test_grid_format() {
        assert_eq!(format!("{}", get_example_grid()), EXAMPLE_GRID_INPUT);
        assert_eq!(format!("{:|^}", get_example_grid()), "\
            2|1|9|9|9|4|3|2|1|0\n\
            3|9|8|7|8|9|4|9|2|1\n\
            9|8|5|6|7|8|9|8|9|2\n\
            8|7|6|7|8|9|6|7|8|9\n\
            9|8|9|9|9|6|5|6|7|8");
    }

    #[test]
    fn test_grid_from_str() {
        assert_eq!(EXAMPLE_GRID_INPUT.parse::<Grid<usize>>(), Ok(get_example_grid()));
    }

    #[test]
    fn test_get_adjacent() {
        let grid = get_example_grid();
        assert_eq!(grid.get_adjacent(&(0, 0).into(), Directions::NonDiagonal), vec![1, 3]);
        assert_eq!(grid.get_adjacent(&(0, 0).into(), Directions::All), vec![1, 9, 3]);
        assert_eq!(grid.get_adjacent(&(5, 0).into(), Directions::NonDiagonal), vec![3, 9, 9]);
        assert_eq!(grid.get_adjacent(&(5, 3).into(), Directions::NonDiagonal), vec![8, 6, 6, 8]);
        assert_eq!(grid.get_adjacent(&(9, 4).into(), Directions::NonDiagonal), vec![9, 7]);
    }

    #[test]
    fn test_get_adjacent_points() {
        let grid = get_example_grid();

        assert_eq!(grid.get_adjacent_points(&(0, 0).into(), Directions::NonDiagonal), vec![(1, 0).into(), (0, 1).into()]);
        assert_eq!(grid.get_adjacent_points(&(0, 0).into(), Directions::All), vec![(1, 0).into(), (1, 1).into(), (0, 1).into()]);

        assert_eq!(grid.get_adjacent_points(&(5, 3).into(), Directions::NonDiagonal),
                   vec![(5, 2).into(), (6, 3).into(), (5, 4).into(), (4, 3).into()]);
        assert_eq!(grid.get_adjacent_points(&(5, 3).into(), Directions::All),
                   vec![(4, 2).into(), (5, 2).into(), (6, 2).into(), (6, 3).into(), (6, 4).into(), (5, 4).into(), (4, 4).into(), (4, 3).into()]);
    }

    #[test]
    fn test_get_points_in_direction() {
        let grid = get_example_grid();
        assert_eq!(grid.get_points_in_direction(&(0,0).into(), Directions::Left), vec![]);
        assert_eq!(grid.get_points_in_direction(&(1,0).into(), Directions::Left), vec![(0, 0).into()]);
        assert_eq!(grid.get_points_in_direction(&(2,0).into(), Directions::Left), vec![(1,0).into(), (0, 0).into()]);
    }

    #[test]
    fn test_values() {
        let grid: Grid<usize> = vec![vec![1, 2, 3], vec![9, 8, 7], vec![5, 6, 4]].try_into().unwrap();
        assert_eq!(grid.values(), vec![1, 2, 3, 9, 8, 7, 5, 6, 4]);
    }

    #[test]
    fn test_growing_grid() {
        let mut grid: Grid<usize> = Grid::default();
        assert_eq!(grid.bounds, Bounds { top: 0, left: 0, width: 0, height: 0 });
        assert_eq!(grid.points(), vec![]);

        grid.set((2, 3).into(), 42);
        assert_eq!(grid.bounds, Bounds { top: 3, left: 2, width: 1, height: 1 });
        assert_eq!(grid.points(), vec![
            (2,3).into()
        ]);

        grid.set((1,2).into(), 22);
        assert_eq!(grid.bounds, Bounds { top: 2, left: 1, width: 2, height: 2 });
        assert_eq!(grid.points(), vec![
            (1,2).into(), (2,2).into(),
            (1,3).into(), (2,3).into()
        ]);

        grid.set((-2, -2).into(), 12);
        assert_eq!(grid.bounds, Bounds { top: -2, left: -2, width: 5, height: 6 });
        assert_eq!(grid.points(), vec![
            (-2,-2).into(), (-1,-2).into(), (0,-2).into(), (1,-2).into(), (2,-2).into(),
            (-2,-1).into(), (-1,-1).into(), (0,-1).into(), (1,-1).into(), (2,-1).into(),
            (-2,0).into(), (-1,0).into(), (0,0).into(), (1,0).into(), (2,0).into(),
            (-2,1).into(), (-1,1).into(), (0,1).into(), (1,1).into(), (2,1).into(),
            (-2,2).into(), (-1,2).into(), (0,2).into(), (1,2).into(), (2,2).into(),
            (-2,3).into(), (-1,3).into(), (0,3).into(), (1,3).into(), (2,3).into()
        ]);
    }
}