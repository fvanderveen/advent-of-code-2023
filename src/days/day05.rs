use std::ops::Range;
use std::str::FromStr;
use crate::days::Day;
use crate::util::parser::Parser;

pub const DAY5: Day = Day {
    puzzle1,
    puzzle2,
};

fn puzzle1(input: &String) {
    let almanac = input.parse::<Almanac>().unwrap();

    let lowest_location = almanac.initial_seeds.iter().map(|s| almanac.get_location(s)).min().unwrap();
    println!("Lowest location of initial seeds: {}", lowest_location);
}

fn puzzle2(input: &String) {
    let almanac = input.parse::<Almanac>().unwrap();

    println!("Lowest location of ranges: {}", almanac.find_lowest_destination_seed());
}

#[derive(Eq, PartialEq, Debug, Clone, Default)]
struct Almanac {
    initial_seeds: Vec<usize>,
    seed_to_soil: AlmanacMap,
    soil_to_fertilizer: AlmanacMap,
    fertilizer_to_water: AlmanacMap,
    water_to_light: AlmanacMap,
    light_to_temperature: AlmanacMap,
    temperature_to_humidity: AlmanacMap,
    humidity_to_location: AlmanacMap,
}

impl Almanac {
    fn get_location(&self, seed: &usize) -> usize {
        let soil = self.seed_to_soil.remap(seed);
        let fertilizer = self.soil_to_fertilizer.remap(&soil);
        let water = self.fertilizer_to_water.remap(&fertilizer);
        let light = self.water_to_light.remap(&water);
        let temperature = self.light_to_temperature.remap(&light);
        let humidity = self.temperature_to_humidity.remap(&temperature);
        self.humidity_to_location.remap(&humidity)
    }

    fn get_seed_to_location_map(&self) -> AlmanacMap {
        let seed_to_fertilizer = self.soil_to_fertilizer.remap_map(&self.seed_to_soil);
        let seed_to_water = self.fertilizer_to_water.remap_map(&seed_to_fertilizer);
        let seed_to_light = self.water_to_light.remap_map(&seed_to_water);
        let seed_to_temperature = self.light_to_temperature.remap_map(&seed_to_light);
        let seed_to_humidity = self.temperature_to_humidity.remap_map(&seed_to_temperature);
        self.humidity_to_location.remap_map(&seed_to_humidity)
    }

    fn find_lowest_destination_seed(&self) -> usize {
        // Seed inputs are considered pairs (start + length), given those ranges find the lowest position
        let mut seed_ranges = vec![];
        for i in (0..self.initial_seeds.len()).step_by(2) {
            let start = self.initial_seeds[i];
            let length = self.initial_seeds[i+1];
            seed_ranges.push(start..(start+length));
        }

        let seed_to_location_map = self.get_seed_to_location_map();
        // To still not try all the ranges, we order the ranges inside the big map from lowest destination to highest.
        // Then, we'll find the first range that has a overlap with a range, and find the first hit from there, and hope we're right.
        let mut ranges = seed_to_location_map.ranges.clone();
        ranges.sort_by_key(|r| r.destination_start);
        let interesting_range = ranges.iter().find(|r| seed_ranges.iter().any(|sr| r.overlaps(sr))).unwrap();
        println!("Lowest seed should be in {}-{}", interesting_range.source_range().start, interesting_range.source_range().end);

        let seed_range = seed_ranges.iter().find(|sr| interesting_range.overlaps(sr)).unwrap();
        println!("Seed should come from range {}-{}", seed_range.start, seed_range.end);

        // Result should be the max of the interesting range and seed range's starts (either the first remapped, or the first in range)
        let seed = interesting_range.source_start.max(seed_range.start);
        println!("The seed number should be {}", seed);

        interesting_range.remap(&seed).unwrap()
    }
}

impl FromStr for Almanac {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().collect::<Vec<_>>();

        // First line should be declaring the seeds:
        let mut initial_seeds = vec!();
        let mut parser = Parser::new(lines[0]);
        parser.literal("seeds:")?;
        while !parser.is_exhausted() {
            initial_seeds.push(parser.usize()?);
        }

        let mut almanac = Almanac {
            initial_seeds,
            ..Default::default()
        };

        // From there, we should find blocks for the different maps followed by relevant lines and
        // separated by a blank line.
        let mut current_map_type = None;
        let mut map_lines: Vec<&str> = vec![];
        // Ensure there is (at least) a single blank line at the end so we don't need to duplicate the
        // logic to add the values to the right map.
        lines.push("");
        for i in 2..lines.len() {
            if current_map_type.is_none() {
                // First line should be a map type:
                current_map_type = Some(lines[i]);
            } else if lines[i].trim().is_empty() {
                // Empty line separates, so we create the map and add it to almanac
                let ranges = map_lines.iter().map(|l| AlmanacRange::from_str(l)).collect::<Result<Vec<_>, _>>()?;
                let map = AlmanacMap { ranges };
                match current_map_type {
                    None if map_lines.len() == 0 => continue, // skip accidental multiple newlines
                    Some("seed-to-soil map:") => almanac.seed_to_soil = map,
                    Some("soil-to-fertilizer map:") => almanac.soil_to_fertilizer = map,
                    Some("fertilizer-to-water map:") => almanac.fertilizer_to_water = map,
                    Some("water-to-light map:") => almanac.water_to_light = map,
                    Some("light-to-temperature map:") => almanac.light_to_temperature = map,
                    Some("temperature-to-humidity map:") => almanac.temperature_to_humidity = map,
                    Some("humidity-to-location map:") => almanac.humidity_to_location = map,
                    None => return Err(format!("Missing map name for ranges [{}]", map_lines.join(", "))),
                    Some(unknown) => return Err(format!("Unknown map name: '{}'", unknown)),
                }
                current_map_type = None;
                map_lines.clear();
            } else {
                map_lines.push(lines[i]);
            }
        }

        Ok(almanac)
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Default)]
struct AlmanacMap {
    ranges: Vec<AlmanacRange>,
}

impl AlmanacMap {
    fn remap(&self, source: &usize) -> usize {
        self.ranges.iter().find_map(|r| r.remap(source)).unwrap_or(*source)
    }

    fn remap_map(&self, map: &AlmanacMap) -> AlmanacMap {
        // We cannot remap number-by-number, that would be too slow.
        // We assume the given map's destinations is what maps onto our ranges

        // We need to create a set of ranges to cover the ranges from both maps.
        // The lowest index is the minimum source index from both maps (as unmapped indexes get passed thru)
        // The highest index is the maximum source index
        // That range needs to be cut up in:
        // An optional part before our lowest range (still passing thru)
        // Parts where our ranges influence it
        // Parts between our ranges (where we keep the destination of the source, if any)
        // Ideally we don't include entries that are not remapped at all

        // We take all ranges from the given map, and remap them using this map
        let mut ranges = map.ranges.iter().flat_map(|r| self.remap_range(r)).collect::<Vec<_>>();

        // Then, we look at all our ranges, and make sub-ranges of the bits that are not yet mapped
        // by any of the already re-mapped input.
        for range in &self.ranges {
            let mut current_start = range.source_start;

            let mut overlaps = ranges.iter().filter(|r| r.overlaps(&range.source_range())).cloned().collect::<Vec<_>>();
            overlaps.sort_by_key(|o| o.source_start);
            for overlap in overlaps {
                // Since sorted, we should get these ordered by start index, allowing easy handling of the remaining gaps.
                if overlap.source_start > current_start {
                    // We have a gap to fill:
                    let offset = current_start - range.source_start;
                    let length = overlap.source_start - current_start;
                    ranges.push(AlmanacRange { source_start: current_start, destination_start: range.destination_start + offset, length });
                }
                // move current_start to the end of the handled range
                current_start = overlap.source_range().end;
            }

            // Make sure to map the last bit, if any:
            if current_start < range.source_range().end {
                let offset = current_start - range.source_start;
                let length = range.source_range().end - current_start;
                ranges.push(AlmanacRange { source_start: current_start, destination_start: range.destination_start + offset, length })
            }
        }

        ranges.sort_by_key(|r| r.source_start);

        AlmanacMap { ranges }
    }

    fn remap_range(&self, source: &AlmanacRange) -> Vec<AlmanacRange> {
        // Since we assume (/know) ranges don't overlap; we find the first range in this map that overlaps
        // the given range
        let mut ranges = vec![];

        let source_end = source.destination_start + source.length;
        let mut current_offset = 0; // offset inside the range we're currently at

        loop {
            if current_offset >= source.length {
                break;
            }

            let leftover_range = (source.destination_start + current_offset)..source_end;
            if let Some(range) = self.ranges.iter().filter(|r| r.overlaps(&leftover_range)).min_by_key(|r| r.source_start) {
                let current_start = source.destination_start + current_offset;
                if current_start < range.source_start {
                    // Add a range from current_start => range.source_start
                    let length = range.source_start - current_start;
                    ranges.push(AlmanacRange { source_start: source.source_start + current_offset, destination_start: current_start, length });
                    current_offset += length;
                }
                // create a range from range.source_start to min(range.source_end, source_end)
                let current_start = source.destination_start + current_offset;
                let current_end = (range.source_start + range.length).min(source_end);
                let length = current_end - current_start;
                let range_offset = current_start - range.source_start;
                ranges.push(AlmanacRange { source_start: source.source_start + current_offset, destination_start: range.destination_start + range_offset, length });
                current_offset += length;
            } else {
                break // We don't have any more ranges affecting this
            }
        }

        // Add a range from current_start => destination_end if applicable
        let current_start = source.destination_start + current_offset;
        let current_end = source.destination_start + source.length;
        if current_start < current_end {
            ranges.push(AlmanacRange { source_start: source.source_start + current_offset, destination_start: current_start, length: current_end - current_start })
        }

        ranges
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
struct AlmanacRange {
    source_start: usize,
    destination_start: usize,
    length: usize,
}

impl FromStr for AlmanacRange {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parser = Parser::new(s);
        // A range should be a single line consisting of three numbers:
        let destination_start = parser.usize()?;
        let source_start = parser.usize()?;
        let length = parser.usize()?;
        parser.ensure_exhausted()?;
        Ok(Self {
            source_start,
            destination_start,
            length,
        })
    }
}

impl AlmanacRange {
    fn source_range(&self) -> Range<usize> {
        self.source_start..(self.source_start + self.length)
    }

    fn contains(&self, source: &usize) -> bool {
        self.source_range().contains(source)
    }

    fn overlaps(&self, range: &Range<usize>) -> bool {
        let source_range = self.source_range();

        source_range.start < range.end && source_range.end > range.start
    }

    fn remap(&self, source: &usize) -> Option<usize> {
        if !self.contains(source) {
            None
        } else {
            let offset = source - self.source_start;
            Some(self.destination_start + offset)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day05::{Almanac, AlmanacMap, AlmanacRange};

    #[test]
    fn test_almanac_range_remap() {
        let small_range = AlmanacRange { source_start: 98, destination_start: 50, length: 2 };
        assert_eq!(small_range.remap(&98), Some(50));
        assert_eq!(small_range.remap(&99), Some(51));
        assert_eq!(small_range.remap(&100), None);
        assert_eq!(small_range.remap(&50), None);

        let large_range = AlmanacRange { source_start: 50, destination_start: 52, length: 48 };
        assert_eq!(large_range.remap(&50), Some(52));
        assert_eq!(large_range.remap(&52), Some(54));
        assert_eq!(large_range.remap(&97), Some(99));
        assert_eq!(large_range.remap(&98), None);
        assert_eq!(large_range.remap(&49), None);
    }

    #[test]
    fn test_almanac_range_from_str() {
        assert_eq!("50 98 2".parse::<AlmanacRange>(), Ok(AlmanacRange { source_start: 98, destination_start: 50, length: 2 }));
    }

    #[test]
    fn test_almanac_map_remap() {
        let map = AlmanacMap {
            ranges: vec![
                AlmanacRange { source_start: 98, destination_start: 50, length: 2 },
                AlmanacRange { source_start: 50, destination_start: 52, length: 48 },
            ]
        };

        assert_eq!(map.remap(&13), 13);
        assert_eq!(map.remap(&40), 40);
        assert_eq!(map.remap(&50), 52);
        assert_eq!(map.remap(&74), 76);
        assert_eq!(map.remap(&97), 99);
        assert_eq!(map.remap(&98), 50);
        assert_eq!(map.remap(&99), 51);
    }

    #[test]
    fn test_almanac_map_remap_range() {
        let map = AlmanacMap {
            ranges: vec![
                AlmanacRange { source_start: 20, destination_start: 0, length: 32 },
                AlmanacRange { source_start: 52, destination_start: 37, length: 2 },
                AlmanacRange { source_start: 0, destination_start: 39, length: 15 },
            ]
        };

        let range = AlmanacRange { source_start: 98, destination_start: 50, length: 2 };
        assert_eq!(map.remap_range(&range), vec![AlmanacRange { source_start: 98, destination_start: 30, length: 2 }]);

        let range = AlmanacRange { source_start: 50, destination_start: 52, length: 48 };
        assert_eq!(map.remap_range(&range), vec![
            AlmanacRange { source_start: 50, destination_start: 37, length: 2 }, // 50 (=>52) and 51 (=>53) are remapped by the map
            AlmanacRange { source_start: 52, destination_start: 54, length: 46 } // 52 and up are kept as mapped by the range (offset by 2)
        ]);
    }

    #[test]
    fn test_almanac_remap_map() {
        let first = AlmanacMap {
            ranges: vec![
                AlmanacRange { source_start: 98, destination_start: 50, length: 2 },
                AlmanacRange { source_start: 50, destination_start: 52, length: 48 },
            ]
        };
        let second = AlmanacMap {
            ranges: vec![
                AlmanacRange { source_start: 20, destination_start: 0, length: 32 },
                AlmanacRange { source_start: 52, destination_start: 37, length: 2 },
                AlmanacRange { source_start: 0, destination_start: 39, length: 15 },
            ]
        };

        let result = second.remap_map(&first);
        assert_eq!(result, AlmanacMap {
            ranges: vec![
                AlmanacRange { source_start: 0, destination_start: 39, length: 15 }, //  0 - 15 | Fully from second, no overlap from first
                AlmanacRange { source_start: 20, destination_start: 0, length: 30 }, // 20 - 50 | Partially from second the bit without overlap
                AlmanacRange { source_start: 50, destination_start: 37, length: 2 }, // 50 - 52 | Partially from first, 50-52 => 52-54, remapped by the second rule in second
                AlmanacRange { source_start: 52, destination_start: 54, length: 46 },// 52 - 98 | Rest of the range from first
                AlmanacRange { source_start: 98, destination_start: 30, length: 2 }  // 98 - 100| Other range of first, remapped to 50/51 and then by the first rule of second to 30/31
            ]
        });
    }

    #[test]
    fn test_almanac_from_str() {
        let result = TEST_INPUT.parse::<Almanac>();
        assert!(result.is_ok(), "Expected OK but got Err({})", result.err().unwrap());
        let map = result.unwrap();

        assert_eq!(map.initial_seeds, vec![79, 14, 55, 13]);
        assert_eq!(map.seed_to_soil.ranges.len(), 2);
        assert_eq!(map.soil_to_fertilizer.ranges.len(), 3);
        assert_eq!(map.fertilizer_to_water.ranges.len(), 4);
        assert_eq!(map.water_to_light.ranges.len(), 2);
        assert_eq!(map.light_to_temperature.ranges.len(), 3);
        assert_eq!(map.temperature_to_humidity.ranges.len(), 2);
        assert_eq!(map.humidity_to_location.ranges.len(), 2);

        // Seed 79, soil 81, fertilizer 81, water 81, light 74, temperature 78, humidity 78, location 82.
        assert_eq!(map.seed_to_soil.remap(&79), 81);
        assert_eq!(map.soil_to_fertilizer.remap(&81), 81);
        assert_eq!(map.fertilizer_to_water.remap(&81), 81);
        assert_eq!(map.water_to_light.remap(&81), 74);
        assert_eq!(map.light_to_temperature.remap(&74), 78);
        assert_eq!(map.temperature_to_humidity.remap(&78), 78);
        assert_eq!(map.humidity_to_location.remap(&78), 82);
    }

    #[test]
    fn test_almanac_get_location() {
        let almanac = TEST_INPUT.parse::<Almanac>().unwrap();

        // Seed 79, soil 81, fertilizer 81, water 81, light 74, temperature 78, humidity 78, location 82.
        assert_eq!(almanac.get_location(&79), 82);
        // Seed 14, soil 14, fertilizer 53, water 49, light 42, temperature 42, humidity 43, location 43.
        assert_eq!(almanac.get_location(&14), 43);
        // Seed 55, soil 57, fertilizer 57, water 53, light 46, temperature 82, humidity 82, location 86.
        assert_eq!(almanac.get_location(&55), 86);
        // Seed 13, soil 13, fertilizer 52, water 41, light 34, temperature 34, humidity 35, location 35.
        assert_eq!(almanac.get_location(&13), 35);
    }

    #[test]
    fn test_almanac_get_seed_to_location_map() {
        let almanac = TEST_INPUT.parse::<Almanac>().unwrap();
        let seed_to_location = almanac.get_seed_to_location_map();

        assert_eq!(seed_to_location.remap(&79), 82);
        assert_eq!(seed_to_location.remap(&14), 43);
        assert_eq!(seed_to_location.remap(&55), 86);
        assert_eq!(seed_to_location.remap(&13), 35);
    }

    #[test]
    fn test_almanac_get_lowest_location_seed() {
        let almanac = TEST_INPUT.parse::<Almanac>().unwrap();

        let result = almanac.find_lowest_destination_seed();
        assert_eq!(result, 46);
    }

    const TEST_INPUT: &str = "\
        seeds: 79 14 55 13\n\
        \n\
        seed-to-soil map:\n\
        50 98 2\n\
        52 50 48\n\
        \n\
        soil-to-fertilizer map:\n\
        0 15 37\n\
        37 52 2\n\
        39 0 15\n\
        \n\
        fertilizer-to-water map:\n\
        49 53 8\n\
        0 11 42\n\
        42 0 7\n\
        57 7 4\n\
        \n\
        water-to-light map:\n\
        88 18 7\n\
        18 25 70\n\
        \n\
        light-to-temperature map:\n\
        45 77 23\n\
        81 45 19\n\
        68 64 13\n\
        \n\
        temperature-to-humidity map:\n\
        0 69 1\n\
        1 0 69\n\
        \n\
        humidity-to-location map:\n\
        60 56 37\n\
        56 93 4\n\
    ";

    // ;; Brain dump, flattening maps?
    // humidity => location ranges: (just the last map)
    // 0..56 => 0..56 | 56..93 => 60..97 | 93..97 => 56..60 | 97.. => 97..
    // temperature => location ranges (merging the last two maps)
    // 0..69 => 1..70 :> [(0..55) 1..56 => 1..56, (55..69) 56..70 => 60..74]
    // 69 => 0 :> 0
    // 70.. => 70.. :> [70..93 => 74..97, 93..97 => 56..60, 97..]

}