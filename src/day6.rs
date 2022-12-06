use itertools::Itertools;
use std::io::stdin;
use std::io::Read;

pub fn start(input: &[u8], size: usize) -> Option<usize> {
    input
        .windows(size)
        .position(|x| x.iter().all_unique())
        .map(|x| x + size)
}

pub fn main() -> Result<(), String> {
    let mut input = vec![];
    stdin().read_to_end(&mut input).unwrap();

    println!("{}", start(input.as_ref(), 4).unwrap());
    println!("{}", start(input.as_ref(), 14).unwrap());

    Ok(())
}
