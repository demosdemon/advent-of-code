pub(crate) mod part1;
pub(crate) mod part2;

#[derive(derive_more::IntoIterator, macros::FromIterator)]
#[into_iterator(ref)]
#[from_iterator(Line)]
pub struct Lines(Vec<Line>);

::aoc::derive_FromStr_for_FromIterator!(Lines, Line);

#[derive(Debug)]
pub enum Line {
    Incomplete(Vec<u8>),
    Invalid(u8),
}

::aoc::derive_FromStr_for_bytes_TryFrom_collect!(Line, u8);

impl Line {
    fn score(&self) -> Option<usize> {
        match self {
            Line::Incomplete(v) => Some(v.iter().fold(0, |score, c| {
                (score * 5)
                    + match c {
                        b')' => 1,
                        b']' => 2,
                        b'}' => 3,
                        b'>' => 4,
                        _ => unreachable!(),
                    }
            })),
            Line::Invalid(_) => None,
        }
    }
}

impl FromIterator<u8> for Line {
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        use Line::*;
        let mut stack = Vec::new();
        for c in iter {
            match c {
                b'(' => stack.push(b')'),
                b'[' => stack.push(b']'),
                b'{' => stack.push(b'}'),
                b'<' => stack.push(b'>'),
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
