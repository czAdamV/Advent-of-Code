use std::env;

mod day2;
mod day3;

fn main() -> Result<(), String> {
    match env::args().nth(1).as_deref() {
        Some("2") => day2::main(),
        Some("3") => day3::main(),
        Some(day) => Err(format!("Špatný den {} 😡", day))?,
        None => Err("Chybí den 🥺")?
    }
}
