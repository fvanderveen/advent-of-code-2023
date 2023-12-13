use std::collections::VecDeque;
use std::str::FromStr;
use crate::days::Day;
use crate::util::collection::CollectionExtension;
use crate::util::number::parse_usize;

pub const DAY12: Day = Day {
    puzzle1,
    puzzle2
};

fn puzzle1(input: &String) {
    let lines = input.lines().map(|l| l.parse::<SpringLine>()).collect::<Result<Vec<_>, _>>().unwrap();

    let result: usize = lines.iter().map(|l| l.get_valid_permutations()).sum();
    println!("Sum of valid permutations: {}", result);
}

fn puzzle2(input: &String) {
    let lines = input.lines().map(|l| l.parse::<SpringLine>()).collect::<Result<Vec<_>, _>>().unwrap();

    let result: usize = lines.iter().map(|l| l.unfold().get_valid_permutations()).sum();
    println!("Sum of valid unfolded permutations: {}", result);
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Spring {
    Unknown,
    Broken,
    Operational
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct SpringLine {
    springs: Vec<Spring>,
    broken_groups: Vec<usize>
}

impl SpringLine {
    fn get_broken_groups(springs: &[Spring]) -> Vec<usize> {
        let mut result = vec![];
        let mut current_group = 0;

        // We take unknown as operational for the sake of this function.
        for spring in springs {
            match spring {
                Spring::Operational | Spring::Unknown if current_group > 0 => {
                    result.push(current_group);
                    current_group = 0;
                },
                Spring::Operational | Spring::Unknown => { },
                Spring::Broken => { current_group += 1 },
            }
        }

        if current_group > 0 { result.push(current_group) }

        result
    }

    fn get_valid_permutations(&self) -> usize {
        // Given the broken_groups (ordered), find how many permutations are possible for broken springs to fill unknown
        // spots.
        // E.g. .??..??...?##. 1,1,3
        // 4 mutations, as the '3' group can only succeed by filling the last unknown, but both 1 groups can be in two spots.
        // For ?#?#?#?#?#?#?#? 1,3,1,6
        // We can deduce the first '?' needs to be operational (otherwise becomes ##, which cannot be). The same holds for the second.
        // .#.#?#?#?#?#?#? from the second '#', we need to make 3 happen, so the third and fourth '?' are also known
        // .#.###.#.###### The 1 and 6 then kinda follow the same.
        // As such, we should be able to build this by doing a slightly smarter BFS.
        // We can check for each '?' if it could be a '.' or '#', and if so, continue with the options.

        let mut permutations = 0;
        let mut queue: VecDeque<Vec<Spring>> = VecDeque::from([self.springs.clone()]);

        'main: loop {
            let current = match queue.pop_front() {
                Some(v) => v,
                None => break 'main
            };

            // Given current, take the first next unknown, and check the options. Push valid options on the queue.
            // If no valid options, this route is bust and we continue.
            // If no unknowns, and the list is valid, add a permutation
            if let Some(index) = current.iter().position(|s| Spring::Unknown.eq(s)) {
                // see until where the groups match the expected ones
                // check if we can still fix the unexpected one
                // (Might need similar logic so we still know if we're in a group though)
                let mut current_group: usize = 0;
                let mut group_index= 0;

                // We take unknown as operational for the sake of this function.
                for spring in &current[0..index] {
                    match spring {
                        // Note: technically unknown shouldn't happen, but we keep rust happy.
                        Spring::Operational | Spring::Unknown if current_group > 0 => {
                            // We're adding data without looking ahead, we might end up with an invalid state, so validate
                            // the group size against the target, rejecting this branch if failed:
                            match self.broken_groups.get(group_index) {
                                None => continue 'main,
                                Some(v) if *v != current_group => continue 'main,
                                Some(_) => { } // Group is valid
                            }

                            group_index += 1;
                            current_group = 0;
                        },
                        Spring::Operational | Spring::Unknown => { },
                        Spring::Broken => { current_group += 1 },
                    }
                }

                let group_target = *self.broken_groups.get(group_index).unwrap_or(&0);

                // Note: we need a faster way to determine a combination isn't valid, brute forcing this takes too long
                // (3 sec on test, not finished in 10 minutes on real input).

                // Options:
                // - group_target is 0 (we already handled all groups), we can take a shortcut and add a permutation (all other fields will be working)
                // - group_target equals current_group, the current unknown can only be operational
                // - current_group is 0, in which case we've passed a working spring, and this one could be working or broken
                // - group_target is larger than current_group, the current unknown can only be broken
                let add_operational = group_target == current_group || current_group == 0;
                // Only add broken springs if we need to fill a group, otherwise fill with working and check
                let add_broken = group_target > 0 && (group_target > current_group || current_group == 0);

                let mut next_group = current.clone();
                if add_operational {
                    next_group[index] = Spring::Operational;
                    queue.push_back(next_group.clone());
                }
                if add_broken {
                    next_group[index] = Spring::Broken;
                    queue.push_back(next_group);
                }
            } else if Self::get_broken_groups(&current) == self.broken_groups {
                permutations += 1;
            }
        }

        println!("Solved..., next");

        permutations
    }

    fn unfold(&self) -> Self {
        // unfold by joining the springs 5 times, separated by unknown
        // and by joining the broken sets 5 times.
        let mut new_springs = vec![];
        let mut new_groups = vec![];
        for i in 0..5 {
            if i > 0 { new_springs.push(Spring::Unknown) }
            new_springs.push_all(&self.springs);
            new_groups.push_all(&self.broken_groups);
        }

        Self { springs: new_springs, broken_groups: new_groups }
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day12::SpringLine;

    #[test]
    fn test_get_valid_permutations() {
        let lines = TEST_INPUT.lines().map(|l| l.parse::<SpringLine>()).collect::<Result<Vec<_>, _>>().unwrap();

        assert_eq!(lines[0].get_valid_permutations(), 1);
        assert_eq!(lines[1].get_valid_permutations(), 4);
        assert_eq!(lines[2].get_valid_permutations(), 1);
        assert_eq!(lines[3].get_valid_permutations(), 1);
        assert_eq!(lines[4].get_valid_permutations(), 4);
        assert_eq!(lines[5].get_valid_permutations(), 10);
    }

    #[test]
    fn test_get_unfolded_valid_permutations() {
        let lines = TEST_INPUT.lines().map(|l| l.parse::<SpringLine>()).collect::<Result<Vec<_>, _>>().unwrap();

        assert_eq!(lines[0].unfold().get_valid_permutations(), 1);
        assert_eq!(lines[1].unfold().get_valid_permutations(), 16384);
        assert_eq!(lines[2].unfold().get_valid_permutations(), 1);
        assert_eq!(lines[3].unfold().get_valid_permutations(), 16);
        assert_eq!(lines[4].unfold().get_valid_permutations(), 2500);
        assert_eq!(lines[5].unfold().get_valid_permutations(), 506250);
    }

    const TEST_INPUT: &str = "\
        ???.### 1,1,3\n\
        .??..??...?##. 1,1,3\n\
        ?#?#?#?#?#?#?#? 1,3,1,6\n\
        ????.#...#... 4,1,1\n\
        ????.######..#####. 1,6,5\n\
        ?###???????? 3,2,1\
    ";
}

impl TryFrom<char> for Spring {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Spring::Operational),
            '#' => Ok(Spring::Broken),
            '?' => Ok(Spring::Unknown),
            _ => Err(format!("Unknown spring '{}'", value))
        }
    }
}

impl FromStr for SpringLine {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(" ").collect();
        match parts.len() {
            2 => {
                let springs = parts[0].chars().map(|c| Spring::try_from(c)).collect::<Result<Vec<_>, _>>()?;
                let broken_groups = parts[1].split(",").map(|p| parse_usize(p)).collect::<Result<Vec<_>, _>>()?;
                Ok(Self { springs, broken_groups })
            },
            _ => Err(format!("Expected a string with two parts, but got {}", parts.len()))
        }
    }
}