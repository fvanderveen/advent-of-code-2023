use crate::days::Day;
use crate::util::geometry::{Bounds, Point};
use crate::util::number::parse_isize;

pub const DAY3: Day = Day {
    puzzle1,
    puzzle2,
};

fn puzzle1(input: &String) {
    let schematic = parse_input(input).unwrap();
    let part_numbers = get_part_numbers(&schematic);

    let result: isize = part_numbers.iter().sum();
    println!("Sum of part numbers: {}", result);
}

fn puzzle2(input: &String) {
    let schematic = parse_input(input).unwrap();
    let gear_ratios = get_gear_ratios(&schematic);

    let result: isize = gear_ratios.iter().map(|(_, r)| r).sum();
    println!("Sum of gear ratios: {}", result);
}

// Any number that touches a symbol (also diagonal) is a part number
// Parse to a structures:
// - Number + area for symbols
// - Symbol + point

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
struct Number {
    number: isize,
    bounds: Bounds,
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
struct Symbol {
    symbol: char,
    point: Point,
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct Schematic {
    numbers: Vec<Number>,
    symbols: Vec<Symbol>,
}

fn parse_input(input: &str) -> Result<Schematic, String> {
    let mut numbers: Vec<Number> = vec!();
    let mut symbols: Vec<Symbol> = vec!();

    let lines: Vec<_> = input.lines().collect();
    for y in 0..lines.len() {
        let chars: Vec<_> = lines[y].chars().collect();
        let mut current_number: isize = 0;
        let mut current_number_start: Option<usize> = None;

        for x in 0..chars.len() {
            match chars[x] {
                digit if digit.is_numeric() => {
                    if current_number_start.is_none() {
                        current_number_start = Some(x)
                    }

                    let value: isize = parse_isize(digit.to_string().as_str())?;
                    current_number *= 10;
                    current_number += value;
                }
                symbol => {
                    if let Some(start_x) = current_number_start {
                        let mut bounds = Bounds::try_from_tlbr(y, start_x, y, x - 1)?;
                        bounds.grow(1);
                        numbers.push(Number { number: current_number, bounds });
                        current_number = 0;
                        current_number_start = None;
                    }

                    if symbol != '.' {
                        symbols.push(Symbol { symbol, point: (x, y).try_into()? });
                    }
                }
            }
        }

        if let Some(start_x) = current_number_start {
            let mut bounds = Bounds::try_from_tlbr(y, start_x, y, chars.len() - 1)?;
            bounds.grow(1);
            numbers.push(Number { number: current_number, bounds });
        }
    }

    Ok(Schematic { numbers, symbols })
}

fn get_part_numbers(schematic: &Schematic) -> Vec<isize> {
    schematic.numbers.iter().filter(|n| schematic.symbols.iter().any(|s| n.bounds.contains(&s.point)))
        .map(|n| n.number)
        .collect()
}

fn get_gear_ratios(schematic: &Schematic) -> Vec<(&Symbol, isize)> {
    // A gear is a '*' symbol with two numbers adjacent. The ratio is the multiplication of both
    schematic.symbols.iter()
        .filter(|s| s.symbol == '*')
        .map(|s| (s, schematic.numbers.iter().filter(|n| n.bounds.contains(&s.point)).collect::<Vec<_>>()))
        .filter(|(_, l)| l.len() == 2)
        .map(|(s, l)| (s, l.iter().map(|n| n.number).reduce(|l,r| l*r).unwrap()))
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::days::day03::{get_gear_ratios, get_part_numbers, parse_input};
    use crate::util::geometry::Bounds;

    const TEST_INPUT: &str = "\
        467..114..\n\
        ...*......\n\
        ..35..633.\n\
        ......#...\n\
        617*......\n\
        .....+.58.\n\
        ..592.....\n\
        ......755.\n\
        ...$.*....\n\
        .664.598..\
    ";

    #[test]
    fn test_parse_input() {
        let result = parse_input(TEST_INPUT);
        assert!(result.is_ok(), "Expected OK, but got Err({})", result.err().unwrap());

        let schematic = result.unwrap();
        assert_eq!(schematic.numbers.iter().map(|s| s.number).collect::<Vec<_>>(), vec![467, 114, 35, 633, 617, 58, 592, 755, 664, 598]);
        assert_eq!(schematic.symbols.iter().map(|s| s.symbol).collect::<Vec<_>>(), vec!['*', '#', '*', '+', '$', '*']);

        assert_eq!(schematic.numbers[0].bounds, Bounds::from_tlbr(-1, -1, 1, 3));
        assert_eq!(schematic.symbols[0].point, (3, 1).into());
    }

    #[test]
    fn test_get_part_numbers() {
        let schematic = parse_input(TEST_INPUT).unwrap();
        let result = get_part_numbers(&schematic);

        assert_eq!(result, vec![467, 35, 633, 617, 592, 755, 664, 598]);
    }

    #[test]
    fn test_get_gear_ratios() {
        let schematic = parse_input(TEST_INPUT).unwrap();
        let result = get_gear_ratios(&schematic);

        assert_eq!(result.len(), 2);
        assert_eq!(result[0], (&schematic.symbols[0], 16345));
        assert_eq!(result[1], (&schematic.symbols[5], 451490));
    }
}