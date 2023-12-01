use crate::days::Day;
use crate::util::number::{parse_i32};

pub const DAY1: Day = Day {
    puzzle1,
    puzzle2
};

fn puzzle1(input: &String) {
    let result: i32 = input.lines().map(|l| parse_calibration_line(l).unwrap()).sum();
    println!("Puzzle 1: {}", result);
}
fn puzzle2(input: &String) {
    let result: i32 = input.lines().map(|l| parse_calibration_line_v2(l).unwrap()).sum();
    println!("Puzzle 1: {}", result);
}

// The newly-improved calibration document consists of lines of text;
// each line originally contained a specific calibration value that the Elves now need to recover.
// On each line, the calibration value can be found by combining the first digit and the last digit
// (in that order) to form a single two-digit number.
fn parse_calibration_line(line: &str) -> Result<i32, String> {
    let digits: Vec<char> = line.chars().filter(|c| c.is_digit(10)).collect();
    parse_i32(format!("{}{}", digits[0], digits.last().ok_or(format!("{}", "No digits in input"))?).as_str())
}

// Your calculation isn't quite right. It looks like some of the digits are actually spelled out
// with letters: one, two, three, four, five, six, seven, eight, and nine also count as valid "digits".
fn parse_calibration_line_v2(line: &str) -> Result<i32, String> {
    let mut result: i32 = 0;

    // Walk over the line and find the first (spelled out) digit
    for i in 0..line.len() {
        let chars: Vec<_> = line.chars().collect();
        if chars[i].is_digit(10) {
            result += chars[i].to_string().parse::<i32>().map_err(|e| format!("{}", e))?;
            break;
        }

        let (_, check) = line.split_at(i);
        if check.starts_with("one") { result = 1; break; }
        else if check.starts_with("two") { result = 2; break; }
        else if check.starts_with("three") { result = 3; break; }
        else if check.starts_with("four") { result = 4; break; }
        else if check.starts_with("five") { result = 5; break; }
        else if check.starts_with("six") { result = 6; break; }
        else if check.starts_with("seven") { result = 7; break; }
        else if check.starts_with("eight") { result = 8; break; }
        else if check.starts_with("nine") { result = 9; break; }
    }

    result *= 10;

    // Walk backwards and find the last (spelled out) digit
    for i in (0..line.len()).rev() {
        let chars: Vec<_> = line.chars().collect();
        if chars[i].is_digit(10) {
            result += chars[i].to_string().parse::<i32>().map_err(|e| format!("{}", e))?;
            break;
        }

        let (check, _) = line.split_at(i + 1);
        if check.ends_with("one") { result += 1; break; }
        else if check.ends_with("two") { result += 2; break; }
        else if check.ends_with("three") { result += 3; break; }
        else if check.ends_with("four") { result += 4; break; }
        else if check.ends_with("five") { result += 5; break; }
        else if check.ends_with("six") { result += 6; break; }
        else if check.ends_with("seven") { result += 7; break; }
        else if check.ends_with("eight") { result += 8; break; }
        else if check.ends_with("nine") { result += 9; break; }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::days::day01::{parse_calibration_line, parse_calibration_line_v2};

    #[test]
    fn test_parse_calibration_line() {
        assert_eq!(parse_calibration_line("1abc2"), Ok(12));
        assert_eq!(parse_calibration_line("pqr3stu8vwx"), Ok(38));
        assert_eq!(parse_calibration_line("a1b2c3d4e5f"), Ok(15));
        assert_eq!(parse_calibration_line("treb7uchet"), Ok(77));
    }

    #[test]
    fn test_parse_calibration_line_v2() {
        assert_eq!(parse_calibration_line_v2("two1nine"), Ok(29));
        assert_eq!(parse_calibration_line_v2("eightwothree"), Ok(83));
        assert_eq!(parse_calibration_line_v2("abcone2threexyz"), Ok(13));
        assert_eq!(parse_calibration_line_v2("xtwone3four"), Ok(24));
        assert_eq!(parse_calibration_line_v2("4nineeightseven2"), Ok(42));
        assert_eq!(parse_calibration_line_v2("zoneight234"), Ok(14));
        assert_eq!(parse_calibration_line_v2("7pqrstsixteen"), Ok(76));

        // Unsure if this should work, but I want it to.
        assert_eq!(parse_calibration_line_v2("oneight"), Ok(18));
    }
}