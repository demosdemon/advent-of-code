pub(crate) mod part1;
pub(crate) mod part2;

#[derive(derive_more::IntoIterator, macros::FromLines)]
#[into_iterator(ref)]
#[from_lines(Line)]
pub struct Lines(Vec<Line>);

#[derive(Debug)]
pub enum Line {
    Incomplete(Vec<u8>),
    Invalid(u8),
}

impl std::str::FromStr for Line {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.bytes().collect())
    }
}

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
