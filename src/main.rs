extern crate core;

mod days;
mod util;

use std::env::args;
use days::{get_day, Day};
use util::input::{read_input};
use util::number::{parse_i32};

fn print_usage()
{
    eprintln!("
Usage: cargo run <command> [<command_arg>, ...]

Commands:
    day <day number> - run the puzzles for the given day.
    add <day number> - add base files and wiring for a new day.
");
}

fn main() {
    let a: Vec<String> = args().collect();

    if a.len() < 3 {
        print_usage();
        return;
    }

    match a[1].as_str() {
        "day" => {
            run_day(&a[2])
        }
        "add" => {
            add_day(&a[2])
        }
        _ => {
            print_usage();
        }
    }
}

fn run_day(day_num: &str)
{
    let result: Result<(String, Day), String> = parse_i32(day_num)
        .and_then(|d| get_day(d).and_then(|day| read_input(d).and_then(|input| Ok((input, day)))));
    match result {
        Ok((input, day)) => {
            (day.puzzle1)(&input);
            (day.puzzle2)(&input);
        }
        Err(err) => {
            eprintln!("{}", err);
        }
    }
}

fn add_day(input: &str)
{
    // This is going to be fun. Write code to modify the running code! Woohoo!
    match parse_i32(input) {
        Ok(day) => {
            match util::create_day::create_day(day) {
                Ok(_) => { println!("Successfully added day {}", day); }
                Err(e) => { panic!("{}", e); }
            }
        }
        Err(err) => {
            panic!("{}", err);
        }
    }
}