mod part1;
mod part2;

use std::convert::Infallible;
use std::str::FromStr;

#[derive(Debug)]
enum Line {
    Incomplete(Vec<char>),
    Invalid(char),
}

impl Line {
    fn score(self) -> Option<isize> {
        match self {
            Line::Incomplete(v) => Some(v.into_iter().fold(0, |score, c| {
                (score * 5)
                    + match c {
                        ')' => 1,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        _ => unreachable!(),
                    }
            })),
            Line::Invalid(_) => None,
        }
    }
}

impl FromIterator<char> for Line {
    fn from_iter<T: IntoIterator<Item = char>>(iter: T) -> Self {
        use Line::*;
        let mut stack = Vec::new();
        for c in iter {
            match c {
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                '{' => stack.push('}'),
                '<' => stack.push('>'),
                _ => match stack.pop() {
                    None => return Invalid(c),
                    Some(v) if c == v => {}
                    Some(_) => return Invalid(c),
                },
            }
        }
        stack.reverse();
        Incomplete(stack)
    }
}

impl FromStr for Line {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.chars().collect())
    }
}
