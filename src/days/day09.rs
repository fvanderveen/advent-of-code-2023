use crate::days::Day;
use crate::util::number::parse_isize;

pub const DAY9: Day = Day {
    puzzle1,
    puzzle2
};

fn parse_input(input: &String) -> Vec<Vec<isize>> {
    input.lines().map(|l| l.split(" ").map(|c| parse_isize(c).unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>()
}

fn puzzle1(input: &String) {
    let parsed = parse_input(input);

    let result = parsed.iter().map(|list| get_next_value(list, Direction::Future)).sum::<isize>();
    println!("Puzzle 1: {}", result);
}

fn puzzle2(input: &String) {
    let parsed = parse_input(input);

    let result = parsed.iter().map(|list| get_next_value(list, Direction::History)).sum::<isize>();
    println!("Puzzle 2: {}", result);
}

enum Direction {
    Future,
    History
}

fn get_next_value(input: &Vec<isize>, direction: Direction) -> isize {
    // To get the next value, we need to compute sub-lists based on the differences between the numbers.
    // We repeat that until the differences list is all zeroes, from which we can calculate back by
    // adding a 0, which means adding the same number to the previous list, which mean adding that to
    // the one before that, etc.
    // 10 13 16 21 30 45 *68
    //   3  3  5  9  15 *23
    //     0  2  4  6  *8
    //       2  2  2  *2
    //         0  0  *0

    let mut list_stack = vec![input.clone()];

    loop {
        let mut next_list = vec![];
        let current_list = list_stack.iter().last().unwrap(); // Should always be at least one list here.
        for i in 0..current_list.len()-1 {
            next_list.push(current_list[i+1]-current_list[i]);
        }
        if next_list.iter().all(|v| 0.eq(v)) { break; }
        list_stack.push(next_list);
    }

    let mut next_value=  0;
    loop {
        // While we have a list, use the value to calculate the next in sequence
        if let Some(list) = list_stack.pop() {
            next_value = match direction {
                Direction::Future => list[list.len() - 1] + next_value,
                Direction::History => list[0] - next_value
            };
        } else {
            // When we handled the last (initial) list, we can return the next value
            return next_value;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day09::{Direction, get_next_value};

    #[test]
    fn test_get_next_value() {
        assert_eq!(get_next_value(&vec![0, 3, 6, 9, 12, 15], Direction::Future), 18);
        assert_eq!(get_next_value(&vec![0, -3, -6, -9, -12, -15], Direction::Future), -18);
        assert_eq!(get_next_value(&vec![1, 3, 6, 10, 15, 21], Direction::Future), 28);
        assert_eq!(get_next_value(&vec![10, 13, 16, 21, 30, 45], Direction::Future), 68);

        assert_eq!(get_next_value(&vec![0, 3, 6, 9, 12, 15], Direction::History), -3);
        assert_eq!(get_next_value(&vec![0, -3, -6, -9, -12, -15], Direction::History), 3);
        assert_eq!(get_next_value(&vec![1, 3, 6, 10, 15, 21], Direction::History), 0);
        assert_eq!(get_next_value(&vec![10, 13, 16, 21, 30, 45], Direction::History), 5);
    }
}