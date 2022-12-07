use aoc::nom::usize;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::character::complete::not_line_ending;
use nom::multi::separated_list0;
use nom::sequence::separated_pair;
use nom::sequence::terminated;
use nom::IResult;

pub(super) fn shell(mut s: &str) -> IResult<&str, super::Shell> {
    let mut input = Vec::new();
    while !s.is_empty() {
        let (rest, v) = terminated(command, line_ending)(s)?;
        s = rest;
        input.push(v);
    }
    Ok((s, super::Shell(input)))
}

fn command(s: &str) -> IResult<&str, super::Command> {
    alt((chdir, list))(s)
}

fn chdir(s: &str) -> IResult<&str, super::Command> {
    let (s, _) = tag("$ cd ")(s)?;
    let (s, v) = not_line_ending(s)?;
    Ok((s, super::Command::chdir(v)))
}

fn list(s: &str) -> IResult<&str, super::Command> {
    let (s, _) = tag("$ ls")(s)?;
    let (s, _) = line_ending(s)?;
    let (s, list) = separated_list0(line_ending, list_line)(s)?;
    Ok((s, super::Command::list(list)))
}

fn list_line(s: &str) -> IResult<&str, super::ListLine> {
    alt((list_line_dir, list_line_file))(s)
}

fn list_line_dir(s: &str) -> IResult<&str, super::ListLine> {
    let (s, _) = tag("dir ")(s)?;
    let (s, v) = not_line_ending(s)?;
    Ok((s, super::ListLine::directory(v)))
}

fn list_line_file(s: &str) -> IResult<&str, super::ListLine> {
    let (s, (size, name)) = separated_pair(usize, tag(" "), not_line_ending)(s)?;
    Ok((s, super::ListLine::file(name, size)))
}
