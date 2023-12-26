use std::collections::HashMap;
use std::str::FromStr;
use crate::days::Day;
use crate::util::collection::CollectionExtension;

pub const DAY25: Day = Day {
    puzzle1,
    puzzle2
};

fn puzzle1(input: &String) {
    let mess: Mess = input.parse().unwrap();

    println!("Result of groups: {}", mess.split_components().unwrap());
}
fn puzzle2(_input: &String) {
    // Part 2 is a 'freebie', assuming you got all stars. :see_no_evil:
    println!("Happy part 2 solvings~");
}

// We need to find 3 wires that, when cut, separate the big mess of components into two separate groups (of whatever sizes (>1 ofc)).
// We got about 1100 lines of real components (compared to the 13 in the test input)
// Might work by taking the first and per connection deciding to 'include' it or not; end conditions:
// - Exactly three connections outgoing = split
// - Only two components in unpicked = bust
// Need a representation that easily allows counting group sizes and counting the number of connections between the groups.
// Might work with recursive DFS?

#[derive(Eq, PartialEq, Debug, Clone)]
struct Mess {
    components: Vec<String>,
    wires: Vec<Wire>
}

impl Mess {
    fn get_outgoing_connections(&self, group: &Vec<String>) -> Vec<String> {
        self.wires.iter().filter_map(|w| match (group.contains(&w.left), group.contains(&w.right)) {
            (true, true) | (false, false) => None, // Either both or neither are in the group
            (true, false) => Some(w.right.clone()),
            (false, true) => Some(w.left.clone())
        }).collect()
    }

    fn split_components(&self) -> Option<usize> {
        fn duplicates(list: Vec<String>) -> Vec<(String, isize)> {
            let mut map: HashMap<String, isize> = HashMap::new();
            for item in list {
                let count = map.get(&item).unwrap_or(&0) + 1;
                map.insert(item.clone(), count);
            }

            let mut vec: Vec<_> = map.into_iter().collect();
            vec.sort_by_key(|(_, c)| -c);
            vec
        }

        // Add the first component, and from there use DFS? to add new components (probably prioritize ones with multiple
        // connections?) until we find a division where there is exactly three outgoing connections.
        fn find_group(mess: &Mess, left: Vec<String>, right: &Vec<String>) -> Option<Vec<String>> {
            let connections = mess.get_outgoing_connections(&left);
            // End condition: we have a group (>= 2 items) that can be cut by exactly three connections.
            if left.len() >= 2 && connections.len() == 3 { return Some(left) }

            // Otherwise, we include a new component based on the one that has most connections to our group first, and the others second.
            let mut next_right = right.clone();
            for (item, _) in duplicates(connections) {
                if right.contains(&item) { continue; } // already ignored before
                if let Some(group) = find_group(mess, left.append_item(&item), &next_right) {
                    return Some(group);
                }
                next_right.push(item);
            }

            None
        }

        let left = find_group(self, vec![self.components[0].clone()], &vec![])?.len();
        let right = self.components.len() - left;

        Some(left * right)
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct Wire {
    left: String,
    right: String
}

#[cfg(test)]
mod tests {
    use crate::days::day25::Mess;
    use crate::util::collection::VecToString;

    #[test]
    fn test_get_outgoing_connections() {
        let mess: Mess = TEST_INPUT.parse().unwrap();

        // Note: we include double connection to properly count the amount in our search.
        assert_eq!(mess.get_outgoing_connections(
            &vec!["jqt", "rhn", "xhk", "bvb"].to_string()
        ), vec!["nvd", "hfx", "cmg", "hfx", "hfx", "ntq", "ntq", "ntq"].to_string());
        // jqt -> nvd, hfx -> xhk, cmg -> bvb, rhn -> hfx, bvb -> hfx, ntq -> jqt, ntq -> bvb, ntq -> xhk

        assert_eq!(mess.get_outgoing_connections(
            &vec!["jqt", "rhn", "xhk", "bvb", "hfx"].to_string()
        ), vec!["nvd", "cmg", "pzl", "ntq", "ntq", "ntq", "ntq"].to_string());
        // jqt -> nvd, cmg -> bvb, pzl -> hfx, ntq -> jqt, ntq -> hfx, ntq -> bvb, ntq -> xhk

        assert_eq!(mess.get_outgoing_connections(
            &vec!["jqt", "rhn", "xhk", "bvb", "hfx", "ntq"].to_string()
        ), vec!["nvd", "cmg", "pzl"].to_string());
        // jqt -> nvd, cmg -> bvb, pzl -> hfx
    }

    #[test]
    fn test_split_components() {
        let mess: Mess = TEST_INPUT.parse().unwrap();

        assert_eq!(mess.split_components(), Some(54));
    }

    const TEST_INPUT: &str = "\
        jqt: rhn xhk nvd\n\
        rsh: frs pzl lsr\n\
        xhk: hfx\n\
        cmg: qnr nvd lhk bvb\n\
        rhn: xhk bvb hfx\n\
        bvb: xhk hfx\n\
        pzl: lsr hfx nvd\n\
        qnr: nvd\n\
        ntq: jqt hfx bvb xhk\n\
        nvd: lhk\n\
        lsr: lhk\n\
        rzs: qnr cmg lsr rsh\n\
        frs: qnr lhk lsr\
    ";
}

impl FromStr for Mess {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut components: Vec<String> = vec![];
        let mut wires = vec![];

        for line in s.lines() {
            let component = line[0..3].to_string();
            if !components.contains(&component) { components.push(component.clone()) }

            for connection in line[4..].trim().split(' ').collect::<Vec<_>>() {
                let other = connection.to_string();
                if !components.contains(&other) { components.push(other.clone()) }
                wires.push(Wire { left: component.clone(), right: other.clone() });
            }
        }

        Ok(Self { components, wires })
    }
}