use std::io::stdin;

fn part_two_correct(opponent: u8, me: u8) -> u8 {
    match me {
        0 => (opponent as i16 - 1).rem_euclid(3) as u8,
        1 => opponent,
        2 => (opponent + 1) % 3,
        _ => unreachable!("this shouldn't happen lol")
    }
}

fn simple_maffs(opponent: u8, me: u8) -> u8 {
    let mut score = me + 1;

    if (opponent + 1) % 3 == me {
        score += 6
    }

    if opponent == me {
        score += 3;
    }

    score
}

pub fn main() -> Result<(), String> {
    let mut score_part_one: u64 = 0;
    let mut score_part_two: u64 = 0;

    for line in stdin().lines().map(|x| x.unwrap()).filter(|x| x != "") {
        let (opponent, me) = match line.as_bytes() {
            [opponent @ b'A' ..= b'C', b' ', me @ b'X' ..= b'Z'] =>
                (opponent - b'A', me - b'X'),
            _ => Err(format!("Invalid line {}", line))?
        };

        score_part_one += simple_maffs(opponent, me) as u64;
        score_part_two += simple_maffs(opponent, part_two_correct(opponent, me)) as u64;
    }
    
    println!("{}", score_part_one);
    println!("{}", score_part_two);

    Ok(())
}
