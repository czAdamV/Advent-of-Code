use std::env;

mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn main() -> Result<(), String> {
    match env::args().nth(1).as_deref() {
        Some("2") => day2::main(),
        Some("3") => day3::main(),
        Some("4") => day4::main(),
        Some("5") => day5::main(),
        Some("6") => day6::main(),
        Some(day) => Err(format!("Špatný den {} 😡", day))?,
        None => Err("Chybí den 🥺")?
    }
}
