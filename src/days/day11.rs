use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::days::Day;
use crate::util::collection::CollectionExtension;
use crate::util::geometry::{Grid, Point};

pub const DAY11: Day = Day {
    puzzle1,
    puzzle2,
};

fn puzzle1(input: &String) {
    let map = input.parse::<GalaxyMap>().unwrap();
    let galaxy_map = expand_galaxy(&map,2, false);

    let distances = get_distance_between_galaxies(&galaxy_map);
    let sum: isize = distances.iter().sum();
    println!("Sum of distances between pairs of galaxies is: {}", sum);
}

fn puzzle2(input: &String) {
    let map = input.parse::<GalaxyMap>().unwrap();
    let galaxy_map = expand_galaxy(&map,1_000_000, false);

    let distances = get_distance_between_galaxies(&galaxy_map);
    let sum: isize = distances.iter().sum();
    println!("Sum of distances between pairs of galaxies is: {}", sum);
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, Default)]
enum MapTile {
    #[default]
    Nothing,
    Galaxy,
}

type GalaxyMap = Grid<MapTile>;

fn expand_galaxy(galaxy: &GalaxyMap, factor: usize, fill_empty: bool) -> GalaxyMap {
    // The given galaxy will expand any empty row and column, for puzzle one (at least) they need
    // to be doubled.

    // To arrange this, we'll gather the empty rows and columns, and just offset any non-empty point
    // depending on how many empty lines seen.
    let empty_cols: Vec<_> = galaxy.bounds.x().filter(|col| galaxy.get_column(*col).iter().all(|tile| MapTile::Nothing.eq(tile))).collect();
    let empty_rows: Vec<_> = galaxy.bounds.y().filter(|row| galaxy.get_row(*row).iter().all(|tile| MapTile::Nothing.eq(tile))).collect();

    let mut new_galaxy = GalaxyMap::new(HashMap::from_iter(
        galaxy.entries().into_iter().filter(|(_, tile)| MapTile::Galaxy.eq(tile))
            .map(|(point, tile)| {
                let empty_cols_before = empty_cols.iter().filter(|col| point.x > **col).count();
                let empty_rows_before = empty_rows.iter().filter(|row| point.y > **row).count();
                let x = point.x + ((empty_cols_before * factor) - empty_cols_before) as isize;
                let y = point.y + ((empty_rows_before * factor) - empty_rows_before) as isize;
                (Point::from((x, y)), tile)
            })
    ));
    // To make testing / formatting a bit easier, fill the rest of the bounds explicitly with nothing.
    if fill_empty {
        for point in new_galaxy.points() {
            if new_galaxy.get(&point).is_none() {
                new_galaxy.set(point, MapTile::Nothing);
            }
        }
    }
    new_galaxy
}

fn get_distance_between_galaxies(galaxy: &GalaxyMap) -> Vec<isize> {
    let galaxies: Vec<_> = galaxy.entries().into_iter().filter(|(_, tile)| MapTile::Galaxy.eq(tile)).map(|(p, _)| p).collect();

    let mut result = vec![];

    for i in 0..galaxies.len() {
        let current_point = galaxies[i];
        result.push_all(&galaxies[(i+1)..].iter().map(|other| other.manhattan_distance(&current_point)).collect())
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::days::day11::{expand_galaxy, GalaxyMap, get_distance_between_galaxies};

    #[test]
    fn test_expand_galaxy() {
        let galaxy = TEST_INPUT.parse::<GalaxyMap>().unwrap();
        let expanded = expand_galaxy(&galaxy,2, true);

        assert_eq!(format!("{}", expanded), "\
            ....#........\n\
            .........#...\n\
            #............\n\
            .............\n\
            .............\n\
            ........#....\n\
            .#...........\n\
            ............#\n\
            .............\n\
            .............\n\
            .........#...\n\
            #....#.......\
        ");
    }

    #[test]
    fn test_distance_between_galaxies() {
        let galaxy = TEST_INPUT.parse::<GalaxyMap>().unwrap();
        let expanded = expand_galaxy(&galaxy, 2, false);

        let distances = get_distance_between_galaxies(&expanded);
        assert_eq!(distances.iter().sum::<isize>(), 374);

        let expanded = expand_galaxy(&galaxy, 10, false);
        let distances = get_distance_between_galaxies(&expanded);
        assert_eq!(distances.iter().sum::<isize>(), 1030);

        let expanded = expand_galaxy(&galaxy, 100, false);
        let distances = get_distance_between_galaxies(&expanded);
        assert_eq!(distances.iter().sum::<isize>(), 8410);
    }

    const TEST_INPUT: &str = "\
        ...#......\n\
        .......#..\n\
        #.........\n\
        ..........\n\
        ......#...\n\
        .#........\n\
        .........#\n\
        ..........\n\
        .......#..\n\
        #...#.....\
    ";
}

impl FromStr for MapTile {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Self::Nothing),
            "#" => Ok(Self::Galaxy),
            _ => Err(format!("Invalid MapTile '{}'", s))
        }
    }
}

impl Display for MapTile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MapTile::Nothing => write!(f, "."),
            MapTile::Galaxy => write!(f, "#")
        }
    }
}