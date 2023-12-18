use std::ops::RangeInclusive;
use std::str::FromStr;
use crate::days::Day;
use crate::util::geometry::{Directions};
use crate::util::number::{parse_usize_radix};
use crate::util::parser::Parser;

pub const DAY18: Day = Day {
    puzzle1,
    puzzle2,
};

fn puzzle1(input: &String) {
    let operations = Operation::parse_input(input).unwrap();
    println!("Lagoon size after digging: {}", fill(&operations, false));
}

fn puzzle2(input: &String) {
    let operations = Operation::parse_input(input).unwrap();
    println!("Lagoon size after digging: {}", fill(&operations, true));
}

fn fill(operations: &Vec<Operation>, use_encoded_data: bool) -> isize {
    // The naive implementation to actually draw the walls might be too slow given puzzle two uses the 6-char hex
    // values as amounts instead.
    // The amount of tiles 'to fill' is also going to be be too high to traverse one-by-one, even for the test input.
    // 952_408_144_115, even assuming a million operations per ms, this would take 952 _seconds_ to go through.
    // So we need a smarter solution...
    // Given we know all operations, we should be able to create regions of dug-out ground.
    // Taking a single dig-right operation, we should find matching dig-left operations (getting columns of ground)
    // We do need to know the column/row for dig operations, so we need to pre-process the given operations a bit.
    #[derive(Eq, PartialEq, Debug)]
    enum ProcessedOperation {
        Right(RangeInclusive<isize>, isize),
        Left(RangeInclusive<isize>, isize),
    }

    let mut processed = vec![];

    let mut current_row: isize = 0;
    let mut current_col: isize = 0;

    for oi in 0..operations.len() {
        let operation = operations[oi];
        let prev_operation = if oi > 0 { operations[oi - 1] } else { operations[operations.len() - 1] };
        let next_operation = if oi < operations.len() - 1 { operations[oi + 1] } else { operations[0] };
        let amount = operation.amount(use_encoded_data) as isize;
        // Note: we only store left/right, as that contains all information we need.
        match operation.direction(use_encoded_data) {
            Directions::Top => { current_row -= amount; }
            Directions::Bottom => { current_row += amount; }
            Directions::Right => {
                let mut start = current_col;
                let mut length = amount;
                // If the previous operation went down, our first cell is empty
                if prev_operation.direction(use_encoded_data) == Directions::Bottom {
                    start += 1;
                    length -= 1;
                }
                // If the next operation goes up, our last cell is empty
                if next_operation.direction(use_encoded_data) == Directions::Top {
                    length -= 1;
                }

                let range = start..=(start + length);
                processed.push(ProcessedOperation::Right(range, current_row));
                current_col += amount;
            }
            Directions::Left => {
                let mut start = current_col - amount;
                let mut length = amount;
                // If the previous operation went up, our last cell is empty
                if prev_operation.direction(use_encoded_data) == Directions::Top {
                    length -= 1;
                }
                // If the next operation goes down, our first cell is empty
                if next_operation.direction(use_encoded_data) == Directions::Bottom {
                    start += 1;
                    length -= 1;
                }

                let range = start..=(start + length);
                processed.push(ProcessedOperation::Left(range, current_row));
                current_col -= amount;
            }
            _ => {} // Ignore wrong direction, should not happen from input
        }
    }

    // Just a sanity check if we did right.
    if current_row != 0 || current_col != 0 { panic!("Did not make a loop?! {}, {}", current_row, current_col) }

    // Build the blocks of lagoon based on the ranges we now have (no clue yet how though :joy:)
    // Since the lagoon is a loop, we should be able to get the size starting from all ranges going right, and finding
    // corresponding ranges that go left below it. (could be multiple, but there should _always_ be one)
    let mut lagoon_size = 0;

    let dig_rights: Vec<(RangeInclusive<isize>, isize)> = processed.iter().filter_map(|o| match o {
        ProcessedOperation::Right(range, col) => Some((range.clone(), *col)),
        _ => None
    }).collect();
    let dig_lefts: Vec<(RangeInclusive<isize>, isize)> = processed.iter().filter_map(|o| match o {
        ProcessedOperation::Left(range, col) => Some((range.clone(), *col)),
        _ => None
    }).collect();
    // Sort the left-digs by their row (ascending) to easily find the closest digs to the right ones.

    fn ranges_overlap(left: &RangeInclusive<isize>, right: &RangeInclusive<isize>) -> bool {
        left.start() <= right.end() && right.start() <= left.end()
    }

    // dig_rights.iter().for_each(|(r, row)| println!("Right @ {} => {:?}", row, r));
    // dig_lefts.iter().for_each(|(r, row)| println!("Left @ {} => {:?}", row, r));

    for dig_right in dig_rights {
        // We need to find all dig_lefts that overlap our dug range, we can then find the lagoon area of that big taking
        // the overlapped range size, and the difference in column.
        // Going over our range, find the closest left range that ends at the same index.
        // If the next range is closer, we need to end the first range at the start of the other.
        // If the next range is further away, we start that range on the overlapping index
        let mut current_start = *dig_right.0.start();
        // Note: we need to ensure current_start, as well as target_end are compensated if there is a line above this one
        // Either here, or when processing the ranges..
        while current_start <= *dig_right.0.end() {
            // println!("Trying to find a range left from {} (till {}), {}", current_start, dig_right.0.end(), dig_right.1);
            let next_range = dig_lefts.iter().filter(|(r, row)| row > &dig_right.1 && r.contains(&current_start)).min_by_key(|(_, row)| row).unwrap();
            let mut end = (*next_range.0.end()).min(*dig_right.0.end());

            // Find if a range overlaps next_range above it, if so, we end right before its start (we already cut off empty cells above)
            if let Some(overlap) = dig_lefts.iter().filter(|(r, row)| row < &next_range.1 && r.start() > &current_start && r.start() <= &end && ranges_overlap(r, &next_range.0)).min_by_key(|(r, _)| r.start()) {
                end = *overlap.0.start() - 1;
            }

            let area = (next_range.1 - dig_right.1 + 1) * (end - current_start + 1); // end and start are inclusive
            // println!("Matched range from {} to {} of height {} => {}", current_start, end, next_range.1 - dig_right.1, area);
            lagoon_size += area;
            current_start = end + 1;
        }
    }

    lagoon_size
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
struct Operation {
    raw_direction: Directions,
    raw_amount: usize,

    encoded_direction: Directions,
    encoded_amount: usize,
}

impl Operation {
    fn parse_input(input: &str) -> Result<Vec<Operation>, String> {
        input.lines().map(|l| l.parse::<Operation>()).collect()
    }

    fn direction(&self, use_encoded_data: bool) -> Directions {
        if use_encoded_data { self.encoded_direction } else { self.raw_direction }
    }
    fn amount(&self, use_encoded_data: bool) -> usize {
        if use_encoded_data { self.encoded_amount } else { self.raw_amount }
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day18::{Operation, fill};
    use crate::util::geometry::Directions;

    #[test]
    fn test_parse_operation() {
        assert_eq!("R 6 (#70c710)".parse::<Operation>(), Ok(Operation { raw_direction: Directions::Right, raw_amount: 6, encoded_direction: Directions::Right, encoded_amount: 0x70c71 }));
        assert_eq!("L 2 (#5713f0)".parse::<Operation>(), Ok(Operation { raw_direction: Directions::Left, raw_amount: 2, encoded_direction: Directions::Right, encoded_amount: 0x5713f }));
        assert_eq!("U 3 (#a77fa3)".parse::<Operation>(), Ok(Operation { raw_direction: Directions::Top, raw_amount: 3, encoded_direction: Directions::Top, encoded_amount: 0xa77fa }));
        assert_eq!("D 2 (#411b91)".parse::<Operation>(), Ok(Operation { raw_direction: Directions::Bottom, raw_amount: 2, encoded_direction: Directions::Bottom, encoded_amount: 0x411b9 }));
    }

    #[test]
    fn test_fill() {
        let operations = Operation::parse_input(TEST_INPUT).unwrap();
        assert_eq!(fill(&operations, false), 62);
        assert_eq!(fill(&operations, true), 952408144115);
    }

    const TEST_INPUT: &str = "\
        R 6 (#70c710)\n\
        D 5 (#0dc571)\n\
        L 2 (#5713f0)\n\
        D 2 (#d2c081)\n\
        R 2 (#59c680)\n\
        D 2 (#411b91)\n\
        L 5 (#8ceee2)\n\
        U 2 (#caa173)\n\
        L 1 (#1b58a2)\n\
        U 2 (#caa171)\n\
        R 2 (#7807d2)\n\
        U 3 (#a77fa3)\n\
        L 2 (#015232)\n\
        U 2 (#7a21e3)\
    ";
}

impl FromStr for Operation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parser = Parser::new(s);

        let raw_direction = match parser.one_of(vec!["U", "R", "D", "L"])? {
            "U" => Directions::Top,
            "R" => Directions::Right,
            "D" => Directions::Bottom,
            "L" => Directions::Left,
            s => return Err(format!("Invalid direction {}", s))
        };
        let raw_amount = parser.usize()?;
        parser.literal("(#")?;
        let encoded_amount = parse_usize_radix(&parser.str(5)?, 16)?;
        let encoded_direction = match &*(parser.str(1)?) {
            "0" => Directions::Right,
            "1" => Directions::Bottom,
            "2" => Directions::Left,
            "3" => Directions::Top,
            s => return Err(format!("Invalid encoded direction {}", s))
        };
        parser.literal(")")?;
        parser.ensure_exhausted()?;

        Ok(Self { raw_direction, raw_amount, encoded_direction, encoded_amount })
    }
}
