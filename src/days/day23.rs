use std::collections::{HashMap, HashSet, VecDeque};
use std::str::FromStr;
use crate::days::Day;
use crate::util::collection::CollectionExtension;
use crate::util::geometry::{Directions, Grid, Point};

pub const DAY23: Day = Day {
    puzzle1,
    puzzle2,
};

fn puzzle1(input: &String) {
    let map: Map = input.parse().unwrap();

    println!("Longest hike path: {} steps", map.longest_hike_path(true).unwrap());
}

fn puzzle2(input: &String) {
    let map: Map = input.parse().unwrap();

    println!("Longest non-slippery hike path: {} steps", map.longest_hike_path(false).unwrap());
}

#[derive(Eq, PartialEq, Debug, Default, Copy, Clone)]
enum Tile {
    #[default]
    Forest,
    Path,
    SlopeNorth,
    SlopeEast,
    SlopeSouth,
    SlopeWest,
}

type Map = Grid<Tile>;

impl Map {
    fn start(&self) -> Point {
        self.get_row(0).iter().position(|t| Tile::Path.eq(t)).map(|x| Point { x: x as isize, y: 0 })
            .ok_or("Could not find a start point?!")
            .unwrap()
    }

    fn end(&self) -> Point {
        let y = self.bounds.bottom();
        self.get_row(y).iter().position(|t| Tile::Path.eq(t)).map(|x| Point { x: x as isize, y })
            .ok_or("Could not find an end point?!")
            .unwrap()
    }

    // Obviously forcing it works only for the test input. However, the map itself isn't very complex. We should be able
    // to make the amount of computations less by just figuring out the junction nodes, and computing distances between
    // two connected ones. That should give us a weighted graph where we can just brute force through without too many
    // things to iterate over. (Since we cannot visit a tile more than once, we can also only visit a node once.)

    fn longest_hike_path(&self, slippery: bool) -> Option<usize> {
        let graph = Graph::new(self, slippery);
        graph.longest_path()
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct Path {
    destination: Point,
    length: usize,
}

#[derive(Eq, PartialEq, Debug, Default, Clone)]
struct Node {
    paths: Vec<Path>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct Graph {
    start: Point,
    end: Point,
    nodes: HashMap<Point, Node>,
}

impl Graph {
    fn new(map: &Map, slippery: bool) -> Self {
        // A point is a node if there are more than two paths adjacent to it
        fn is_node(map: &Map, p: &Point) -> bool {
            map.start().eq(p) || map.end().eq(p) ||
                map.get_adjacent(p, Directions::NonDiagonal).iter().filter(|t| Tile::Forest.ne(t)).count() > 2
        }

        fn can_enter(map: &Map, from: &Point, to: &Point, slippery: bool) -> bool {
            if let Some(tile) = map.get(to) {
                match tile {
                    Tile::Forest => false,
                    Tile::Path => true,
                    _ if !slippery => true, // Ignore slopes if it's not slippery
                    Tile::SlopeNorth => from.y > to.y, // Can only go down (north)
                    Tile::SlopeEast => from.x < to.x,
                    Tile::SlopeSouth => from.y < to.y,
                    Tile::SlopeWest => from.x > to.x,
                }
            } else {
                false
            }
        }

        fn follow_path(map: &Map, graph: &mut Graph, node: &Point, first: &Point, visited: &mut HashSet<Point>, slippery: bool) -> Option<Point> {
            let mut path = vec![*node]; // Keep points out of visited until we reach a next node, in case we cannot follow this.

            if visited.contains(first) { return None; } // Already visited this path before
            if !can_enter(map, node, first, slippery) { return None; } // Cannot enter first tile

            let mut current = *first;
            let mut one_direction = false;

            while !is_node(map, &current) {
                path.push(current);

                let next = match current.get_points_around(Directions::NonDiagonal).iter()
                    .filter(|p| !path.contains(p) && can_enter(map, &current, p, slippery))
                    .collect::<Vec<_>>()[..] {
                    [next] => *next,
                    [] => return None,
                    _ => panic!("Entered a junction?!")
                };

                if slippery {
                    one_direction = one_direction || map.get(&next).is_some_and(|t| Tile::Path != t)
                }

                current = next;
            }

            path.iter().for_each(|p| { visited.insert(*p); });

            graph.add_path(node, current, path.len());
            if !one_direction {
                graph.add_node(current); // Ensure node exists
                graph.add_path(&current, *node, path.len());
            }

            Some(current)
        }

        fn visit_map(map: &Map, graph: &mut Graph, slippery: bool) {
            let mut visited: HashSet<Point> = HashSet::new();
            let mut queue: VecDeque<Point> = VecDeque::new();
            queue.push_back(map.start());

            while !queue.is_empty() {
                let node = queue.pop_front().unwrap(); // Guarded by while check
                graph.add_node(node);

                node.get_points_around(Directions::NonDiagonal).iter()
                    .filter_map(|p| follow_path(map, graph, &node, p, &mut visited, slippery))
                    .for_each(|next_node| queue.push_back(next_node));
            }
        }
        let mut result = Self { start: map.start(), end: map.end(), nodes: HashMap::new() };

        visit_map(map, &mut result, slippery);

        result
    }

    fn add_node(&mut self, node: Point) {
        if !self.nodes.contains_key(&node) {
            self.nodes.insert(node, Node::default());
        }
    }

    fn add_path(&mut self, source: &Point, destination: Point, length: usize) {
        if let Some(node) = self.nodes.get_mut(&source) {
            node.paths.push(Path { destination, length })
        }
    }

    fn longest_path(&self) -> Option<usize> {
        // This is an NP-Hard problem, so I don't feel bad doing this brute-forced...
        // (We convert to this graph first so that we don't need to run over the whole path multiple times, saving
        //  us some processing time.)

        fn get_longest_path(graph: &Graph, nodes: Vec<Point>, current_length: usize) -> Option<usize> {
            let current = nodes.last().unwrap(); // Nodes should not be empty.

            if graph.end.eq(current) { return Some(current_length); }

            // For each connected – unvisited – node, try getting the longest path to end.
            let node = graph.nodes.get(current)?;
            let mut result = None;

            for path in &node.paths {
                if nodes.contains(&path.destination) { continue; }

                if let Some(distance) = get_longest_path(graph, nodes.append_item(&path.destination), current_length + path.length) {
                    result = match result {
                        None => Some(distance),
                        Some(current) if current < distance => Some(distance),
                        result => result
                    };
                }
            }

            result
        }

        get_longest_path(self, vec![self.start], 0)
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day23::{Graph, Map, Node, Path};

    #[test]
    fn test_longest_hike_path() {
        let map: Map = TEST_INPUT.parse().unwrap();

        assert_eq!(map.longest_hike_path(true), Some(94));
        assert_eq!(map.longest_hike_path(false), Some(154));
    }

    #[test]
    fn test_convert_to_graph() {
        let map: Map = TEST_INPUT.parse().unwrap();
        let slippery_graph = Graph::new(&map, true);

        assert_eq!(slippery_graph.nodes.contains_key(&slippery_graph.start), true);
        assert_eq!(slippery_graph.nodes.contains_key(&slippery_graph.end), true);

        assert_eq!(slippery_graph.nodes.contains_key(&(3, 5).into()), true);
        assert_eq!(slippery_graph.nodes.get(&(1, 0).into()), Some(&Node {
            paths: vec![
                Path { destination: (3, 5).into(), length: 15 }
            ]
        }));
        assert_eq!(slippery_graph.nodes.contains_key(&(5, 13).into()), true);
        assert_eq!(slippery_graph.nodes.contains_key(&(11, 3).into()), true, "{:?} should contain (11,3)", slippery_graph.nodes.keys());
        assert_eq!(slippery_graph.nodes.get(&(3, 5).into()), Some(&Node {
            paths: vec![
                // Note: no path back to start, as that was one-directional
                Path { destination: (11, 3).into(), length: 22 },
                Path { destination: (5, 13).into(), length: 22 },
            ]
        }));

        let full_graph = Graph::new(&map, false);

        assert_eq!(full_graph.nodes.contains_key(&(1, 0).into()), true);
        assert_eq!(full_graph.nodes.contains_key(&(3, 5).into()), true);
        assert_eq!(full_graph.nodes.get(&(1, 0).into()), Some(&Node {
            paths: vec![
                Path { destination: (3, 5).into(), length: 15 }
            ]
        }));
        assert_eq!(full_graph.nodes.contains_key(&(5, 13).into()), true);
        assert_eq!(full_graph.nodes.contains_key(&(11, 3).into()), true, "{:?} should contain (11,3)", full_graph.nodes.keys());
        assert_eq!(full_graph.nodes.get(&(3, 5).into()), Some(&Node {
            paths: vec![
                Path { destination: (1, 0).into(), length: 15 },
                Path { destination: (11, 3).into(), length: 22 },
                Path { destination: (5, 13).into(), length: 22 },
            ]
        }));
    }

    const TEST_INPUT: &str = "\
        #.#####################\n\
        #.......#########...###\n\
        #######.#########.#.###\n\
        ###.....#.>.>.###.#.###\n\
        ###v#####.#v#.###.#.###\n\
        ###.>...#.#.#.....#...#\n\
        ###v###.#.#.#########.#\n\
        ###...#.#.#.......#...#\n\
        #####.#.#.#######.#.###\n\
        #.....#.#.#.......#...#\n\
        #.#####.#.#.#########v#\n\
        #.#...#...#...###...>.#\n\
        #.#.#v#######v###.###v#\n\
        #...#.>.#...>.>.#.###.#\n\
        #####v#.#.###v#.#.###.#\n\
        #.....#...#...#.#.#...#\n\
        #.#########.###.#.#.###\n\
        #...###...#...#...#.###\n\
        ###.###.#.###v#####v###\n\
        #...#...#.#.>.>.#.>.###\n\
        #.###.###.#.###.#.#v###\n\
        #.....###...###...#...#\n\
        #####################.#\
    ";
}

impl FromStr for Tile {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "#" => Ok(Self::Forest),
            "." => Ok(Self::Path),
            "^" => Ok(Self::SlopeNorth),
            ">" => Ok(Self::SlopeEast),
            "v" => Ok(Self::SlopeSouth),
            "<" => Ok(Self::SlopeWest),
            _ => Err(format!("Invalid tile '{}'", s))
        }
    }
}