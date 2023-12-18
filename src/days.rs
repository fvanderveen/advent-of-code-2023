mod day01;
use day01::DAY1;
mod day02;
use day02::DAY2;
mod day03;
use day03::DAY3;
mod day04;
use day04::DAY4;
mod day05;
use day05::DAY5;
mod day06;
use day06::DAY6;
mod day07;
use day07::DAY7;
mod day08;
use day08::DAY8;
mod day09;
use day09::DAY9;
mod day10;
use day10::DAY10;
mod day11;
use day11::DAY11;
mod day12;
use day12::DAY12;
mod day13;
use day13::DAY13;
mod day14;
use day14::DAY14;
mod day15;
use day15::DAY15;
mod day16;
use day16::DAY16;
mod day18;
use day18::DAY18;
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
        4 => Ok(DAY4),
        5 => Ok(DAY5),
        6 => Ok(DAY6),
        7 => Ok(DAY7),
        8 => Ok(DAY8),
        9 => Ok(DAY9),
        10 => Ok(DAY10),
        11 => Ok(DAY11),
        12 => Ok(DAY12),
        13 => Ok(DAY13),
        14 => Ok(DAY14),
        15 => Ok(DAY15),
        16 => Ok(DAY16),
        18 => Ok(DAY18),
        // « add day match »
        _ => Err(format!("No implementation yet for day {}", day))
    }
}