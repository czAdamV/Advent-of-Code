use std::env;

fn main() -> Result<(), String> {
    match env::args().nth(1) {
        Some(day) => Err(format!("Špatný den {} 😡", day))?,
        None => Err("Chybí den 🥺")?
    }
}
