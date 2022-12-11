use ::aoc::nom::eol;
use ::aoc::nom::usize;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::sequence::preceded;
use nom::sequence::terminated;
use nom::sequence::tuple;
use nom::IResult;

use super::Item;
use super::Monkey;
use super::MonkeyState;
use super::Operation;

pub(super) fn monkey_state(mut s: &str) -> IResult<&str, MonkeyState> {
    let mut v = Vec::new();
    while !s.is_empty() {
        let (rest, o) = monkey(s)?;
        s = rest;
        v.push(o);
    }
    Ok((s, MonkeyState(v)))
}

pub(super) fn monkey(s: &str) -> IResult<&str, Monkey> {
    let (s, id) = monkey_id(s)?;
    let (s, items) = starting_items(s)?;
    let (s, (operation, operand)) = operation(s)?;
    let (s, divisor) = divisor(s)?;
    let (s, toss_true) = toss_true(s)?;
    let (s, toss_false) = toss_false(s)?;
    let (s, _) = eol(s)?;
    Ok((s, Monkey {
        id,
        items,
        operation,
        operand,
        divisor,
        toss_true,
        toss_false,
        inspected: 0,
    }))
}

fn monkey_id(s: &str) -> IResult<&str, usize> {
    terminated(delimited(tag("Monkey "), usize, tag(":")), line_ending)(s)
}

fn starting_items(s: &str) -> IResult<&str, Vec<Item>> {
    delimited(
        tag("  Starting items: "),
        separated_list1(tag(", "), map(usize, Item)),
        line_ending,
    )(s)
}

fn operation(s: &str) -> IResult<&str, (Operation, Option<usize>)> {
    fn op(s: &str) -> IResult<&str, Operation> {
        alt((
            map(tag("+"), |_| Operation::Add),
            map(tag("*"), |_| Operation::Multiply),
        ))(s)
    }

    fn operand(s: &str) -> IResult<&str, Option<usize>> {
        alt((map(tag("old"), |_| None), map(usize, Some)))(s)
    }

    tuple((
        preceded(tag("  Operation: new = old "), op),
        delimited(tag(" "), operand, line_ending),
    ))(s)
}

fn divisor(s: &str) -> IResult<&str, usize> {
    delimited(tag("  Test: divisible by "), usize, line_ending)(s)
}

fn toss_true(s: &str) -> IResult<&str, usize> {
    delimited(tag("    If true: throw to monkey "), usize, line_ending)(s)
}

fn toss_false(s: &str) -> IResult<&str, usize> {
    delimited(tag("    If false: throw to monkey "), usize, line_ending)(s)
}

#[cfg(test)]
mod tests {
    use nom::Parser;

    fn consume<'a, P, O, E>(mut s: &'a str, mut parser: P) -> Result<Vec<O>, nom::Err<E>>
    where
        P: Parser<&'a str, O, E>,
    {
        let mut v: Vec<O> = Vec::new();
        while !s.is_empty() {
            let (rest, o) = parser.parse(s)?;
            s = rest;
            v.push(o);
        }
        Ok(v)
    }

    #[test]
    fn test_parses() {
        let example = include_str!("./inputs/example");
        let live = include_str!("./inputs/live");

        let _ = consume(example, super::monkey).unwrap();
        let _ = consume(live, super::monkey).unwrap();
    }
}
