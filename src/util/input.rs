use std::fs::read_to_string;

pub fn read_input(day: i32) -> Result<String, String> {
    read_to_string(format!("resources/day{:02}.txt", day)).map_err(|e| format!("{}", e))
}
