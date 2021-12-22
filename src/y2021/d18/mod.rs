pub(crate) mod part1;
pub(crate) mod part2;

use std::ops::Add;

pub(crate) mod parser;

#[derive(derive_more::IntoIterator)]
#[into_iterator(ref)]
pub struct Homework(Vec<Snailfish>);

::aoc::derive_FromIterator!(Homework, Snailfish);
::aoc::derive_FromStr_for_FromIterator!(Homework, Snailfish);

#[derive(Debug, Clone, PartialEq, Eq, derive_more::Display, derive_more::From)]
pub enum Node {
    Value(usize),

    Pair(#[from(forward)] Box<Snailfish>),
}

impl From<(usize, usize)> for Node {
    fn from(p: (usize, usize)) -> Self {
        Node::from(Snailfish::from(p))
    }
}

impl From<(f32, f32)> for Node {
    fn from((l, r): (f32, f32)) -> Self {
        (l.floor() as usize, r.ceil() as usize).into()
    }
}

impl Node {
    fn magnitude(&self) -> usize {
        match self {
            Self::Value(v) => *v,
            Self::Pair(b) => b.magnitude(),
        }
    }

    fn absorb(&mut self, from_left: bool, value: usize) {
        match self {
            Self::Value(v) => *v += value,
            Self::Pair(p) => p.absorb(from_left, value),
        }
    }

    fn reduce(&mut self, depth: usize) -> Option<(usize, usize)> {
        match self {
            Self::Value(_) => None,
            Self::Pair(b) if depth == 4 => {
                let rv = b.as_unit().unwrap();
                *self = Self::Value(0);
                Some(rv)
            }
            Self::Pair(b) => b.reduce(depth),
        }
    }

    fn split(&mut self) -> Option<()> {
        match self {
            Self::Value(v) if *v >= 10 => {
                let h = (*v as f32) / 2.0;
                *self = (h, h).into();
                Some(())
            }
            Self::Value(_) => None,
            Self::Pair(b) => b.split(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, derive_more::Display, derive_more::From)]
#[display(fmt = "[{},{}]", _0, _1)]
pub struct Snailfish(#[from(forward)] Node, #[from(forward)] Node);

impl Snailfish {
    fn as_unit(&self) -> Option<(usize, usize)> {
        match self {
            Snailfish(Node::Value(l), Node::Value(r)) => Some((*l, *r)),
            _ => None,
        }
    }

    fn magnitude(&self) -> usize {
        (self.0.magnitude() * 3) + (self.1.magnitude() * 2)
    }

    fn absorb(&mut self, from_left: bool, value: usize) {
        if from_left { &mut self.0 } else { &mut self.1 }.absorb(from_left, value)
    }

    fn reduce(&mut self, depth: usize) -> Option<(usize, usize)> {
        if let Some((a, b)) = self.0.reduce(depth + 1) {
            self.1.absorb(true, b);
            Some((a, 0))
        } else if let Some((a, b)) = self.1.reduce(depth + 1) {
            self.0.absorb(false, a);
            Some((0, b))
        } else {
            None
        }
    }

    fn split(&mut self) -> Option<()> {
        self.0.split().or_else(|| self.1.split())
    }
}

::aoc::derive_FromStr_for_nom!(Snailfish, parser::snailfish);

impl Add for Snailfish {
    type Output = Snailfish;

    fn add(self, rhs: Snailfish) -> Self::Output {
        let mut rv: Snailfish = (self, rhs).into();
        while rv.reduce(0).is_some() || rv.split().is_some() {
            //
        }
        rv
    }
}

impl<'rhs> Add<&'rhs Snailfish> for Snailfish {
    type Output = Snailfish;

    fn add(self, rhs: &'rhs Snailfish) -> Self::Output {
        self + rhs.to_owned()
    }
}

impl<'lhs, 'rhs> Add<&'rhs Snailfish> for &'lhs Snailfish {
    type Output = Snailfish;

    fn add(self, rhs: &'rhs Snailfish) -> Self::Output {
        self.to_owned() + rhs
    }
}

fn sum(input: &Homework) -> Snailfish {
    input.into_iter().cloned().reduce(Add::add).unwrap()
}

#[cfg(test)]
mod tests {
    ::aoc::tests_for_problem!(super::sum, {
        example_a => "[[[[1,1],[2,2]],[3,3]],[4,4]]".parse().unwrap(),
        example_b => "[[[[3,0],[5,3]],[4,4]],[5,5]]".parse().unwrap(),
        example_c => "[[[[5,0],[7,4]],[5,5]],[6,6]]".parse().unwrap(),
        example_d => "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]".parse().unwrap(),
        example_e => "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]".parse().unwrap(),
    });
}
