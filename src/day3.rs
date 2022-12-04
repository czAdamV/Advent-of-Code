use std::{collections::HashSet, io::stdin, ops::BitAnd};

fn priority(item: char) -> Result<u8, String> {
    Ok(match item {
        'a'..='z' => item as u8 - b'a' + 1,
        'A'..='Z' => item as u8 - b'A' + 27,
        _ => Err("Invalid input")?,
    })
}

fn common_character<I>(strings: I) -> Option<char>
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    strings
        .into_iter()
        .map(|s| HashSet::<_>::from_iter(s.as_ref().chars()))
        .reduce(|acc, x| acc.bitand(&x))
        .and_then(|s| s.into_iter().nth(0))
}

pub fn main() -> Result<(), String> {
    let mut priorities_part_one: u64 = 0;
    let mut priorities_part_two: u64 = 0;

    // Collect stdin into a vector of lines so that
    // I can iterate over it more than once
    let lines = stdin().lines().map(|x| x.unwrap()).filter(|x| x != "");
    let lines: Vec<String> = lines.collect();

    for line in lines.iter() {
        let (left, right) = line.split_at(line.len() / 2);
        let res = common_character([left, right]).unwrap();

        priorities_part_one += priority(res)? as u64;
    }

    for chunk in lines.chunks(3) {
        priorities_part_two += priority(common_character(chunk).unwrap())? as u64;
    }

    println!("{priorities_part_one}");
    println!("{priorities_part_two}");

    Ok(())
}
