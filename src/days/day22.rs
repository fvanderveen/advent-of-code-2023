use std::collections::HashMap;
use std::str::FromStr;
use crate::days::Day;
use crate::util::collection::CollectionExtension;
use crate::util::geometry::{Point3D};

pub const DAY22: Day = Day {
    puzzle1,
    puzzle2,
};

fn puzzle1(input: &String) {
    let mut stack: Stack = input.parse().unwrap();
    stack.settle();

    println!("There are {} blocks that can be disintegrated.", stack.count_removable_blocks());
}

fn puzzle2(input: &String) {
    let mut stack: Stack = input.parse().unwrap();
    stack.settle();

    println!("Chain reaction size: {}", stack.sum_of_chain_reactions());
}

// For parsing:
// Two Point3D instances, but only one value should be different (blocks are straight lines, fortunately)
// A block should know all points in there
// Starting from the lowest blocks (based on (lowest) Z) we move the blocks down until on the ground or a preview block
// which gives us our initial input (for puzzle 1)
// We then need to count the number of blocks that can be removed without making another block drop. E.g., all blocks
// that either do not support a block, or support a block together with other blocks.

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
struct Block {
    from: Point3D,
    to: Point3D,
}

impl Block {
    fn points(&self) -> Vec<Point3D> {
        let mut points = vec![];
        for x in self.from.x..=self.to.x {
            for y in self.from.y..=self.to.y {
                for z in self.from.z..=self.to.z {
                    points.push(Point3D { x, y, z })
                }
            }
        }
        points
    }

    fn bottom(&self) -> isize {
        self.from.z.min(self.to.z)
    }

    fn top(&self) -> isize {
        self.from.z.max(self.to.z)
    }

    fn bottom_points(&self) -> Vec<Point3D> {
        let mut points = vec![];
        let z = self.bottom();
        for x in self.from.x..=self.to.x {
            for y in self.from.y..=self.to.y {
                points.push(Point3D { x, y, z })
            }
        }
        points
    }

    fn drop(&mut self, by: isize) {
        self.from.z -= by;
        self.to.z -= by;
    }

    fn points_below(&self) -> Vec<Point3D> {
        let z = self.bottom() - 1;
        let mut points = vec![];
        for x in self.from.x..=self.to.x {
            for y in self.from.y..=self.to.y {
                points.push(Point3D { x, y, z })
            }
        }
        points
    }

    fn supported_by(&self, block: &Block) -> bool {
        !self.points_below().union(&block.points()).is_empty()
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct Stack {
    blocks: Vec<Block>,
}

impl Stack {
    fn settle(&mut self) {
        // Sort own blocks from lowest to highest
        self.blocks.sort_by_key(|b| b.bottom());

        let mut seen_points: Vec<Point3D> = vec![];

        // For each block, find the lowest point (either ground or a previous block) for each of the bottom points
        for block in &mut self.blocks {
            // Get the Z value that would support this block
            let z_support = block.bottom_points().iter()
                .map(|p| seen_points.iter().filter(|sp| p.x == sp.x && p.y == sp.y).max_by_key(|sp| sp.z).map(|p| p.z).unwrap_or(0))
                .max().unwrap_or(0);
            // Drop the block to rest on that value:
            let drop_by = block.bottom() - (z_support + 1);
            block.drop(drop_by);
            seen_points.push_all(&block.points());
        }
    }

    fn count_removable_blocks(&self) -> usize {
        // Iterating multiple times is a bit slow
        let mut blocks_by_z: HashMap<isize, Vec<Block>> = HashMap::new();
        for block in &self.blocks {
            for z in block.bottom()..=block.top() {
                let mut vec = blocks_by_z.get(&z).cloned().unwrap_or(vec![]);
                vec.push(block.clone());
                blocks_by_z.insert(z, vec);
            }
        }

        let mut removable = 0;
        for block in &self.blocks {
            let above: Vec<_> = match blocks_by_z.get(&(block.top() + 1)) {
                Some(blocks) => blocks.iter().filter(|b| b.supported_by(&block)).collect(),
                None => vec![]
            };
            let siblings = match blocks_by_z.get(&block.top()) {
                Some(blocks) => blocks.except(block),
                None => vec![]
            };

            let can_remove = above.is_empty() || (!siblings.is_empty() && above.iter().all(|a| siblings.iter().any(|s| a.supported_by(s))));
            if can_remove {
                removable += 1
            }
        }

        removable
    }

    fn sum_of_chain_reactions(&self) -> usize {
        // Iterating multiple times is a bit slow
        let mut blocks_by_z: HashMap<isize, Vec<Block>> = HashMap::new();
        for block in &self.blocks {
            for z in block.bottom()..=block.top() {
                let mut vec = blocks_by_z.get(&z).cloned().unwrap_or(vec![]);
                vec.push(block.clone());
                blocks_by_z.insert(z, vec);
            }
        }

        // Build a map for each block which blocks support it?
        // Then, for each block, start collecting blocks that will no longer be supported
        let mut supported_by: HashMap<Block, Vec<Block>> = HashMap::new();
        for block in &self.blocks {
            let supports = match blocks_by_z.get(&(block.bottom() - 1)) {
                Some(blocks) => blocks.iter().filter(|b| block.supported_by(b)).cloned().collect(),
                None => vec![]
            };
            supported_by.insert(block.clone(), supports);
        }

        fn count_falling_blocks(falling: Vec<Block>, supported_by: &HashMap<Block, Vec<Block>>) -> usize {
            let new_falling: Vec<_> = supported_by.iter().filter(|(b, s)| !falling.contains(b) && !s.is_empty() && s.iter().all(|sb| falling.contains(sb))).map(|(b, _)| b).cloned().collect();
            if new_falling.len() == 0 {
                falling.len()
            } else {
                count_falling_blocks(falling.into_iter().chain(new_falling.into_iter()).collect(), supported_by)
            }
        }

        let mut chain_reaction = 0;

        for block in &self.blocks {
            let falling = count_falling_blocks(vec![block.clone()], &supported_by) - 1;
            chain_reaction += falling; // ignore self.
        }

        chain_reaction
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day22::{Block, Stack};
    use crate::util::geometry::Point3D;

    #[test]
    fn test_parse_block() {
        assert_eq!("0,0,2~2,0,2".parse(), Ok(Block { from: Point3D { x: 0, y: 0, z: 2 }, to: Point3D { x: 2, y: 0, z: 2 } }));
    }

    #[test]
    fn test_settle() {
        let mut stack: Stack = TEST_INPUT.parse().unwrap();
        stack.settle();

        assert_eq!(stack.blocks, vec![
            Block { from: (1, 0, 1).into(), to: (1, 2, 1).into() }, // A
            Block { from: (0, 0, 2).into(), to: (2, 0, 2).into() }, // B
            Block { from: (0, 2, 2).into(), to: (2, 2, 2).into() }, // C
            Block { from: (0, 0, 3).into(), to: (0, 2, 3).into() }, // D
            Block { from: (2, 0, 3).into(), to: (2, 2, 3).into() }, // E
            Block { from: (0, 1, 4).into(), to: (2, 1, 4).into() }, // F
            Block { from: (1, 1, 5).into(), to: (1, 1, 6).into() }, // G
        ]);
    }

    #[test]
    fn test_removable_block_count() {
        let mut stack: Stack = TEST_INPUT.parse().unwrap();
        stack.settle();

        assert_eq!(stack.count_removable_blocks(), 5);
    }

    #[test]
    fn test_supported_by() {
        let mut stack: Stack = TEST_INPUT.parse().unwrap();
        stack.settle();

        assert_eq!(stack.blocks[1].supported_by(&stack.blocks[0]), true);
        assert_eq!(stack.blocks[2].supported_by(&stack.blocks[0]), true);
    }

    #[test]
    fn test_chain_reaction() {
        let mut stack: Stack = TEST_INPUT.parse().unwrap();
        stack.settle();

        assert_eq!(stack.sum_of_chain_reactions(), 7);
    }

    const TEST_INPUT: &str = "\
        1,0,1~1,2,1\n\
        0,0,2~2,0,2\n\
        0,2,3~2,2,3\n\
        0,0,4~0,2,4\n\
        2,0,5~2,2,5\n\
        0,1,6~2,1,6\n\
        1,1,8~1,1,9\n\
    ";
}

impl FromStr for Block {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [from, to] = match s.split('~').collect::<Vec<_>>()[..] {
            [from_str, to_str] => Ok([from_str.parse()?, to_str.parse()?]),
            _ => Err(format!("Invalid block: '{}'", s))
        }?;

        Ok(Block { from, to })
    }
}

impl FromStr for Stack {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let blocks = s.lines().map(|s| s.parse()).collect::<Result<Vec<_>, _>>()?;
        Ok(Self { blocks })
    }
}
