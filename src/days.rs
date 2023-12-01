mod day01;
use day01::DAY1;
// « add day import »

pub struct Day {
    pub puzzle1: fn(input: &String),
    pub puzzle2: fn(input: &String)
}

pub fn get_day(day: i32) -> Result<Day, String> {
    match day {
        1 => Ok(DAY1),
        // « add day match »
        _ => Err(format!("No implementation yet for day {}", day))
    }
}