use itertools::{all, Itertools};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, digit1, line_ending},
    combinator::{all_consuming, eof, map, map_res, verify},
    multi::{many0, many1, separated_list1},
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};
use num::Integer;
use std::{
    io::{stdin, Read},
    ops::DerefMut,
    str::FromStr,
};

pub fn main() -> Result<(), String> {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let (_, (stacks, commands)) = parse(&input).unwrap();

    let part1 = simulate(&commands, stacks.clone(), |size, source, target| {
        for _ in 0..size {
            target.push(source.pop().unwrap())
        }
    });

    let part2 = simulate(&commands, stacks, |size, source, target| {
        target.append(&mut source.drain(source.len() - size..).collect())
    });

    println!("{part1}");
    println!("{part2}");

    Ok(())
}

pub fn simulate<F>(
    commands: &Vec<(usize, usize, usize)>,
    mut stacks: Vec<Vec<char>>,
    restack: F,
) -> String
where
    F: Fn(usize, &mut Vec<char>, &mut Vec<char>),
{
    for (size, source, target) in commands {
        let (source, target) = borrow_two_mut(&mut stacks, source - 1, target - 1);

        restack(*size, source, target);
    }

    stacks
        .iter()
        .map(|x| x.last().cloned().unwrap_or(' '))
        .collect()
}

fn borrow_two_mut<I, T>(container: &mut I, first: usize, second: usize) -> (&mut T, &mut T)
where
    I: DerefMut<Target = [T]>,
{
    assert!(first != second);

    if second < first {
        let (right, left) = borrow_two_mut(container, second, first);
        return (left, right);
    }

    let (left, right) = container.split_at_mut(second);
    (&mut left[first], &mut right[0])
}

// The parser is complete overkill for such a simple task to be honest.
// But it was an interesting opportunity to play around with nom.
fn parse(input: &str) -> IResult<&str, (Vec<Vec<char>>, Vec<(usize, usize, usize)>)> {
    all_consuming(tuple((
        terminated(package_stacks, line_ending),
        many0(terminated(command, alt((line_ending, eof)))),
    )))(input)
}

fn package_stacks(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    map(
        verify(
            terminated(tuple((package_lines, description)), line_ending),
            |(cargo, desc)| desc.len() == cargo[0].len(),
        ),
        |(mut package_lines, desc)| {
            let mut package_stacks: Vec<Vec<char>> = vec![vec![]; desc.len()];

            for (target, source) in desc.iter().enumerate() {
                package_stacks[target] = package_lines
                    .iter_mut()
                    .rev()
                    .filter_map(|x| x[*source - 1])
                    .collect();
            }

            package_stacks
        },
    )(input)
}

fn command(input: &str) -> IResult<&str, (usize, usize, usize)> {
    tuple((
        preceded(tag("move "), num),
        preceded(tag(" from "), num),
        preceded(tag(" to "), num),
    ))(input)
}

fn num<I>(input: &str) -> IResult<&str, I>
where
    I: FromStr + Integer,
{
    map_res(digit1, str::parse::<I>)(input)
}

fn package(input: &str) -> IResult<&str, Option<char>> {
    alt((
        map(tag("   "), |_| None),
        map(delimited(tag("["), anychar, tag("]")), |x| Some(x)),
    ))(input)
}

fn package_line(input: &str) -> IResult<&str, Vec<Option<char>>> {
    terminated(separated_list1(tag(" "), package), line_ending)(input)
}

fn package_lines(input: &str) -> IResult<&str, Vec<Vec<Option<char>>>> {
    verify(many1(package_line), |res: &Vec<Vec<_>>| {
        res.iter().map(|x| x.len()).all_equal()
    })(input)
}

fn description(input: &str) -> IResult<&str, Vec<usize>> {
    verify(
        delimited(tag(" "), separated_list1(tag("   "), num), tag(" ")),
        |desc: &Vec<usize>| all(desc, |x| x <= &desc.len()) && desc.iter().all_unique(),
    )(input)
}
