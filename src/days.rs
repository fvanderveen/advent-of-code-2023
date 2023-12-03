mod day01;
use day01::DAY1;
mod day02;
use day02::DAY2;
mod day03;
use day03::DAY3;
// « add day import »

pub struct Day {
    pub puzzle1: fn(input: &String),
    pub puzzle2: fn(input: &String)
}

pub fn get_day(day: i32) -> Result<Day, String> {
    match day {
        1 => Ok(DAY1),
        2 => Ok(DAY2),
        3 => Ok(DAY3),
        // « add day match »
        _ => Err(format!("No implementation yet for day {}", day))
    }
}