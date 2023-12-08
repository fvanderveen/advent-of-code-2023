use std::collections::{HashMap};
use std::str::FromStr;
use crate::days::Day;
use crate::util::number::lcm;
use crate::util::parser::Parser;

pub const DAY8: Day = Day {
    puzzle1,
    puzzle2
};

fn puzzle1(input: &String) {
    let map = input.parse::<Map>().unwrap();

    println!("It takes {} steps to get to the end", map.steps_to_end().unwrap());
}

fn puzzle2(input: &String) {
    let map = input.parse::<Map>().unwrap();

    println!("It takes {} ghost steps to get to the end", map.ghost_steps_to_end().unwrap());
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Direction {
    Left,
    Right
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct Map {
    directions: Vec<Direction>,
    nodes: HashMap<String, (String, String)>
}

impl Map {
    fn steps_to_end(&self) -> Result<usize, String> {
        let mut steps = 0;
        let mut node = &"AAA".to_string();

        while node.ne("ZZZ") {
            let (left, right) = match self.nodes.get(node) {
                Some(val) => val,
                None => return Err(format!("Missing node '{}' in map", node))
            };

            let direction = self.directions[steps % self.directions.len()];
            steps += 1;

            match direction {
                Direction::Left => node = left,
                Direction::Right => node = right
            }
        }

        Ok(steps)
    }

    fn ghost_loop_info(&self, start: &String) -> Result<(usize, usize), String> {
        // Loop through this route until we get back to a known state (based on direction index + node)
        // Note: validated by running some debug on this, each start node only comes by a single end node
        //  in their loops. Since that makes the solution simpler, we only care for that case.

        let mut seen: Vec<(usize, &String)> = vec![];
        let mut direction_index = 0;

        let mut node = start;
        loop {
            if let Some(index) = seen.iter().position(|(di, n)| direction_index.eq(di) && n.eq(&node)) {
                // Found the loop, index is the offset, and everything after it is the loop.
                // From the offset, find how much further the first end position is
                // That end position will be the offset from start (the first end) and the loop size we know.
                let loop_size = seen.len() - index;
                let end_index = seen.iter().rposition(|(_, n)| n.ends_with("Z")).ok_or(format!("No end in loop for {}", start))?;
                println!("Loop info for {}: at {} after {} steps, back there every {} next steps.", start, seen[end_index].1, end_index, loop_size);

                return Ok((end_index, loop_size))
            }

            seen.push((direction_index, node));

            let (left, right) = match self.nodes.get(node) {
                Some(val) => val,
                None => return Err(format!("Missing node '{}' in map", node))
            };

            let direction = self.directions[direction_index];
            direction_index = (direction_index + 1)%self.directions.len();

            match direction {
                Direction::Right => node = right,
                Direction::Left => node = left
            }
        }
    }

    fn ghost_steps_to_end(&self) -> Result<usize, String> {
        // Take all nodes ending with 'A', and follow these paths simultaneously until they all are
        // on a node ending with 'Z'.
        // Brute force was way too slow (of course) on the real set. We'll need to use some lcm magic.
        // We'll handle each route one by one:
        // - Find where they loop, and how long the loop is.
        // - Using that information, take the first two routes; and (taking the start + loop size of the first)
        //   find a point in time where it aligns with the second.
        // - When found, use the alignment point as starting offset, and continue with the loop size being the lcm
        //   of both loops.
        // - Using that new information, find an alignment on the third route, and so on.

        let start_nodes: Vec<_> = self.nodes.keys().filter(|k| k.ends_with("A")).collect();

        // We collect all the loop info's, giving us an initial offset and loop size.
        let loop_info= start_nodes.iter().map(|n| self.ghost_loop_info(n)).collect::<Result<Vec<_>, _>>()?;

        let mut index = 1;
        let (offset, mut cycle) = loop_info[0];
        let mut t = offset;

        loop {
            let (next_offset, next_cycle) = loop_info[index];

            if ((t + next_offset) % next_cycle) != 0 {
                t += cycle;
                continue;
            } // Not yet lined up

            println!("Aligned route #{} ({})", index, start_nodes[index]);
            index += 1;

            // If we just lined up the last route, we're done:
            if index == loop_info.len() { return Ok(t) }

            // Otherwise, make cycle the least common multiple of the current cycle and the next:
            cycle = lcm(cycle, next_cycle);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day08::{Direction, Map};

    #[test]
    fn test_parse_map() {
        let result = TEST_INPUT_A.parse::<Map>();
        assert!(result.is_ok(), "Expected Ok, but got Err({})", result.err().unwrap());

        let map = result.unwrap();
        assert_eq!(map.directions, vec![Direction::Right,Direction::Left]);
        assert_eq!(map.nodes.get(&"AAA".to_string()), Some(&("BBB".to_string(), "CCC".to_string())));

        let result = TEST_INPUT_B.parse::<Map>();
        assert!(result.is_ok(), "Expected Ok, but got Err({})", result.err().unwrap());

        let map = result.unwrap();
        assert_eq!(map.directions, vec![Direction::Left,Direction::Left,Direction::Right]);
    }

    #[test]
    fn test_steps_to_end() {
        let map_a = TEST_INPUT_A.parse::<Map>().unwrap();
        assert_eq!(map_a.steps_to_end(), Ok(2));

        let map_b = TEST_INPUT_B.parse::<Map>().unwrap();
        assert_eq!(map_b.steps_to_end(), Ok(6));
    }

    #[test]
    fn test_ghost_steps_to_end() {
        let ghost_map = GHOST_MAP.parse::<Map>().unwrap();
        assert_eq!(ghost_map.ghost_steps_to_end(), Ok(6));
    }

    const TEST_INPUT_A: &str = "\
        RL\n\
        \n\
        AAA = (BBB, CCC)\n\
        BBB = (DDD, EEE)\n\
        CCC = (ZZZ, GGG)\n\
        DDD = (DDD, DDD)\n\
        EEE = (EEE, EEE)\n\
        GGG = (GGG, GGG)\n\
        ZZZ = (ZZZ, ZZZ)\
    ";

    const TEST_INPUT_B: &str = "\
        LLR\n\
        \n\
        AAA = (BBB, BBB)\n\
        BBB = (AAA, ZZZ)\n\
        ZZZ = (ZZZ, ZZZ)\
    ";

    const GHOST_MAP: &str = "\
        LR\n\
        \n\
        11A = (11B, XXX)\n\
        11B = (XXX, 11Z)\n\
        11Z = (11B, XXX)\n\
        22A = (22B, XXX)\n\
        22B = (22C, 22C)\n\
        22C = (22Z, 22Z)\n\
        22Z = (22B, 22B)\n\
        XXX = (XXX, XXX)\
    ";
}

impl FromStr for Map {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<_> = s.lines().collect();
        // First line should be a L/R string to get the directions
        let mut directions = vec![];
        for char in lines[0].chars() {
            match char {
                'R' => directions.push(Direction::Right),
                'L' => directions.push(Direction::Left),
                _ => return Err(format!("Invalid direction '{}'", char))
            }
        }

        let mut nodes: HashMap<String, (String, String)> = HashMap::new();

        for i in 1..lines.len() {
            let line = lines[i];
            if line.trim().is_empty() { continue } // ignore empty lines
            // Each non-empty line should be a node => (left, right) mapping
            // Each node should be 3 characters long.
            let mut parser = Parser::new(line);
            let src = parser.str(3)?;
            parser.literal("=")?;
            parser.literal("(")?;
            let left = parser.str(3)?;
            parser.literal(",")?;
            let right = parser.str(3)?;
            parser.literal(")")?;
            parser.ensure_exhausted()?;

            nodes.insert(src.clone(), (left, right));
        }

        Ok(Map {
            directions,
            nodes
        })
    }
}
