use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use crate::days::Day;
use crate::days::day17::TrafficMode::{Normal, Ultra};
use crate::util::geometry::{Directions, Grid, Point};

pub const DAY17: Day = Day {
    puzzle1,
    puzzle2
};

fn puzzle1(input: &String) {
    let map = TrafficMap::parse(input).unwrap();
    println!("Least heat loss: {}", map.get_best_path(Normal));
}

fn puzzle2(input: &String) {
    let map = TrafficMap::parse(input).unwrap();
    println!("Least heat loss ultra crucibles™: {}", map.get_best_path(Ultra));
}

type TrafficMap = Grid<usize>;

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum TrafficMode {
    Normal,
    Ultra
}

impl TrafficMap {
    fn parse(input: &str) -> Result<TrafficMap, String> {
        input.parse()
    }

    fn get_best_path(&self, mode: TrafficMode) -> usize {
        // We need to find the best path from top-left (0,0) to bottom-right.
        // We can go at most three steps in the same direction (sadly, making this not a simple dijkstra...)
        // However, I hope we can fit this into something close to it by:
        // - Keeping a distance map keyed by (point, direction, steps) (yes, this could somewhat explode, but I hope it'll still work)
        // - Using a BinaryHeap to push next steps / getting the lowest current value from
        // - Ending when we find one that ends up at the destination
        // (Except the 'weird' key, and possible 12 times as large distance map as normal... it should be close to dijkstra)
        let mut distances: HashMap<TrafficDistanceKey, usize> = HashMap::new();
        let mut queue: BinaryHeap<TrafficDistanceEntry> = BinaryHeap::new();

        let destination: Point = (self.bounds.right(), self.bounds.bottom()).into();

        // Initial entry we start with an amount of 0, so that we can still travel three moves even in the same direction.
        queue.push(TrafficDistanceEntry { point: (0, 0).into(), heat_loss: 0, direction: Directions::Right, amount: 0 });

        while let Some(entry) = queue.pop() {
            // Are we done?
            if entry.point == destination {
                return entry.heat_loss
            }

            // Get key to distance map:
            let key = TrafficDistanceKey { point: entry.point, amount: entry.amount, direction: entry.direction };
            if let Some(distance) = distances.get(&key) {
                // Have we already been here with a better score?
                if distance <= &entry.heat_loss { continue; }
            }
            // Update distance map:
            distances.insert(key, entry.heat_loss);

            // If our direction is still allowed, we add it with an additional amount. We add all other directions with amount 1.
            // Note: we cannot turn around
            // Two different options here:
            // - in normal mode, we can turn whenever.
            // - in ultra mode, we can turn only after going in one direction for 4 blocks.
            let options = match entry.direction {
                Directions::Top if mode == Normal && entry.amount < 3 => vec![Directions::Top, Directions::Left, Directions::Right],
                Directions::Top if mode == Normal => vec![Directions::Left, Directions::Right],
                Directions::Top if mode == Ultra && entry.amount < 4 => vec![Directions::Top],
                Directions::Top if mode == Ultra && entry.amount < 10 => vec![Directions::Top, Directions::Left, Directions::Right],
                Directions::Top if mode == Ultra => vec![Directions::Left, Directions::Right],

                Directions::Right if mode == Normal && entry.amount < 3 => vec![Directions::Right, Directions::Top, Directions::Bottom],
                Directions::Right if mode == Normal => vec![Directions::Top, Directions::Bottom],
                Directions::Right if mode == Ultra && entry.amount < 4 => vec![Directions::Right],
                Directions::Right if mode == Ultra && entry.amount < 10 => vec![Directions::Right, Directions::Top, Directions::Bottom],
                Directions::Right if mode == Ultra => vec![Directions::Top, Directions::Bottom],

                Directions::Bottom if mode == Normal && entry.amount < 3 => vec![Directions::Bottom, Directions::Left, Directions::Right],
                Directions::Bottom if mode == Normal => vec![Directions::Left, Directions::Right],
                Directions::Bottom if mode == Ultra && entry.amount < 4 => vec![Directions::Bottom],
                Directions::Bottom if mode == Ultra && entry.amount < 10 => vec![Directions::Bottom, Directions::Left, Directions::Right],
                Directions::Bottom if mode == Ultra => vec![Directions::Left, Directions::Right],

                Directions::Left if mode == Normal && entry.amount < 3 => vec![Directions::Left, Directions::Top, Directions::Bottom],
                Directions::Left if mode == Normal => vec![Directions::Top, Directions::Bottom],
                Directions::Left if mode == Ultra && entry.amount < 4 => vec![Directions::Left],
                Directions::Left if mode == Ultra && entry.amount < 10 => vec![Directions::Left, Directions::Top, Directions::Bottom],
                Directions::Left if mode == Ultra => vec![Directions::Top, Directions::Bottom],
                _ => continue
            };

            for direction in options {
                if let [(next_point, heat_loss)] = self.get_adjacent_entries(&entry.point, direction)[..] {
                    queue.push(TrafficDistanceEntry { point: next_point, heat_loss: entry.heat_loss + heat_loss, direction, amount: if entry.direction == direction { entry.amount + 1 } else { 1 } })
                }
            }
        }

        // Error case, honestly
        usize::MAX
    }
}

// Key for the distance map to implement Dijkstra.
#[derive(Eq, PartialEq, Debug, Hash, Copy, Clone)]
struct TrafficDistanceKey {
    point: Point,
    direction: Directions,
    amount: usize,
}

// Entry for the Dijkstra BinaryHeap representing the current location data and heat loss (travel distance)
#[derive(Eq, PartialEq, Debug, Hash, Copy, Clone)]
struct TrafficDistanceEntry {
    point: Point,
    direction: Directions,
    amount: usize,
    heat_loss: usize,
}

impl Ord for TrafficDistanceEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        // Note: we invert the Ord here such that BinaryHeap.pop returns the _smallest_ value
        other.heat_loss.cmp(&self.heat_loss)
    }
}

impl PartialOrd<Self> for TrafficDistanceEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BinaryHeap;
    use crate::days::day17::{TrafficDistanceEntry, TrafficMap, TrafficMode};
    use crate::util::geometry::{Directions};

    #[test]
    fn test_traffic_distance_key_ordering() {
        let mut heap = BinaryHeap::new();
        let entry_1 = TrafficDistanceEntry { point: (0, 0).into(), direction: Directions::Left, amount: 4, heat_loss: 100 };
        let entry_2 = TrafficDistanceEntry { point: (10, 3).into(), direction: Directions::Left, amount: 1, heat_loss: 95 };
        let entry_3 = TrafficDistanceEntry { point: (12, 4).into(), direction: Directions::Bottom, amount: 3, heat_loss: 105 };

        heap.push(entry_1.clone());
        heap.push(entry_2.clone());
        heap.push(entry_3.clone());

        assert_eq!(heap.pop(), Some(entry_2));
        assert_eq!(heap.pop(), Some(entry_1));
        assert_eq!(heap.pop(), Some(entry_3));
    }

    #[test]
    fn test_get_best_path() {
        let map = TrafficMap::parse(TEST_INPUT).unwrap();

        assert_eq!(map.get_best_path(TrafficMode::Normal), 102);
        assert_eq!(map.get_best_path(TrafficMode::Ultra), 94);
    }

    const TEST_INPUT: &str = "\
        2413432311323\n\
        3215453535623\n\
        3255245654254\n\
        3446585845452\n\
        4546657867536\n\
        1438598798454\n\
        4457876987766\n\
        3637877979653\n\
        4654967986887\n\
        4564679986453\n\
        1224686865563\n\
        2546548887735\n\
        4322674655533\
    ";
}