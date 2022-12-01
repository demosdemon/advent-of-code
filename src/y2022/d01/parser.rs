use aoc::nom::usize;
use nom::character::complete::line_ending;
use nom::combinator::all_consuming;
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::terminated;
use nom::IResult;

use super::Elf;
use super::Elves;

pub fn elves(s: &str) -> IResult<&str, Elves> {
    map(all_consuming(separated_list1(line_ending, elf)), |mut v| {
        v.sort();
        Elves(v)
    })(s)
}

fn elf(s: &str) -> IResult<&str, Elf> {
    map(
        terminated(separated_list1(line_ending, usize), line_ending),
        Elf,
    )(s)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_example() {
        const TEXT: &str = include_str!("./inputs/example");
        let _ = elves(TEXT).unwrap();
    }

    #[test]
    fn test_parse_live() {
        const TEXT: &str = include_str!("./inputs/live");
        let _ = elves(TEXT).unwrap();
    }
}
