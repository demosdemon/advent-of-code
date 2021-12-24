pub(crate) mod part1;
pub(crate) mod part2;

mod parser;

use std::{
    collections::BTreeMap,
    ops::{Index, IndexMut},
};

use anyhow::Context;

#[derive(Debug, Clone, Copy)]
pub enum Variable {
    W,
    X,
    Y,
    Z,
}

#[derive(Debug, Clone, Copy)]
pub enum Value {
    Variable(Variable),

    Literal(isize),
}

impl Value {
    fn literal(&self) -> Option<&isize> {
        match self {
            Self::Variable(_) => None,
            Self::Literal(v) => Some(v),
        }
    }
}

#[derive(derive_more::IsVariant, Debug, Clone, Copy)]
pub enum Instruction {
    Input(Variable),

    Add(Variable, Value),

    Multiply(Variable, Value),

    Divide(Variable, Value),

    Modulo(Variable, Value),

    Equals(Variable, Value),
}

impl Instruction {
    fn rhs(&self) -> Option<&Value> {
        match self {
            Instruction::Input(_) => None,
            Instruction::Add(_, v) => Some(v),
            Instruction::Multiply(_, v) => Some(v),
            Instruction::Divide(_, v) => Some(v),
            Instruction::Modulo(_, v) => Some(v),
            Instruction::Equals(_, v) => Some(v),
        }
    }
}

::aoc::derive_FromStr_for_nom!(Instruction, parser::instruction);

pub struct Instructions(Vec<Instruction>);

::aoc::derive_FromIterator!(Instructions, Instruction);
::aoc::derive_FromStr_for_FromIterator!(Instructions, Instruction);

impl Instructions {
    fn count_inputs(&self) -> usize {
        self.0.iter().filter(|i| i.is_input()).count()
    }

    pub fn opt(&self) -> (usize, usize) {
        let blocks = self.count_inputs();
        debug_assert_eq!(blocks, 14);
        let block_len = self.0.len() / blocks;
        debug_assert_eq!(block_len, 18);

        let stack = Vec::new();
        let links = BTreeMap::new();

        let (_, links) = (0..blocks)
            .map(|b| {
                let lhs = self.0[b * block_len + 5].rhs().unwrap().literal().unwrap();
                let rhs = self.0[b * block_len + 15].rhs().unwrap().literal().unwrap();
                (*lhs, *rhs)
            })
            .enumerate()
            .fold(
                (stack, links),
                |(mut stack, mut links), (idx, (lhs, rhs))| {
                    if lhs > 0 {
                        stack.push((idx, rhs));
                    } else {
                        let (a, b) = stack.pop().unwrap();
                        links.insert(idx, (a, b + lhs));
                    }

                    (stack, links)
                },
            );

        let min = BTreeMap::new();
        let max = BTreeMap::new();
        let (min, max) = links
            .into_iter()
            .fold((min, max), |(mut min, mut max), (a, (b, v))| {
                min.insert(a, std::cmp::max(1, 1 + v));
                min.insert(b, std::cmp::max(1, 1 - v));

                max.insert(a, std::cmp::min(9, 9 + v));
                max.insert(b, std::cmp::min(9, 9 - v));

                (min, max)
            });

        let min = (0..blocks).map(|x| min[&x] as u8).collect::<Vec<_>>();
        let alu = ArithmeticLogicUnit::new(self, &min).unwrap();
        assert_eq!(alu.z, 0);

        let min = min
            .into_iter()
            .map(|v| v as usize)
            .fold(0, |prev, next| (prev * 10) + next);

        let max = (0..blocks).map(|x| max[&x] as u8).collect::<Vec<_>>();
        let alu = ArithmeticLogicUnit::new(self, &max).unwrap();
        assert_eq!(alu.z, 0);

        let max = max
            .into_iter()
            .map(|v| v as usize)
            .fold(0, |prev, next| (prev * 10) + next);

        (min, max)
    }
}

#[derive(Default, Debug, Clone, Copy)]
struct ArithmeticLogicUnit {
    w: isize,
    x: isize,
    y: isize,
    z: isize,
}

impl Index<Variable> for ArithmeticLogicUnit {
    type Output = isize;

    fn index(&self, index: Variable) -> &Self::Output {
        match index {
            Variable::W => &self.w,
            Variable::X => &self.x,
            Variable::Y => &self.y,
            Variable::Z => &self.z,
        }
    }
}

impl IndexMut<Variable> for ArithmeticLogicUnit {
    fn index_mut(&mut self, index: Variable) -> &mut Self::Output {
        match index {
            Variable::W => &mut self.w,
            Variable::X => &mut self.x,
            Variable::Y => &mut self.y,
            Variable::Z => &mut self.z,
        }
    }
}

impl ArithmeticLogicUnit {
    fn eval(&self, value: Value) -> isize {
        match value {
            Value::Variable(v) => self[v],
            Value::Literal(v) => v,
        }
    }

    fn exec(
        &mut self,
        ins: &Instruction,
        inp: &mut impl Iterator<Item = u8>,
    ) -> Result<(), anyhow::Error> {
        match *ins {
            Instruction::Input(v) => {
                self[v] = inp.next().context("insufficient input length")? as isize
            }
            Instruction::Add(a, b) => self[a] += self.eval(b),
            Instruction::Multiply(a, b) => self[a] *= self.eval(b),
            Instruction::Divide(a, b) => self[a] /= self.eval(b),
            Instruction::Modulo(a, b) => self[a] %= self.eval(b),
            Instruction::Equals(a, b) => self[a] = if self[a] == self.eval(b) { 1 } else { 0 },
        }
        Ok(())
    }

    pub fn new(ins: &Instructions, inp: &[u8]) -> Result<Self, anyhow::Error> {
        let mut inp = inp.iter().copied();
        let mut alu = ArithmeticLogicUnit::default();
        for i in ins.0.iter() {
            alu.exec(i, &mut inp)?;
        }
        Ok(alu)
    }
}

#[cfg(test)]
mod tests {
    use super::{ArithmeticLogicUnit, Instructions};

    #[test]
    fn test_inverter() {
        const INPUT: &str = "inp x\nmul x -1\n";
        let v: Instructions = INPUT.parse().unwrap();

        let alu = ArithmeticLogicUnit::new(&v, &[0]).unwrap();
        assert_eq!(alu.x, 0);

        let alu = ArithmeticLogicUnit::new(&v, &[1]).unwrap();
        assert_eq!(alu.x, -1);

        let alu = ArithmeticLogicUnit::new(&v, &[9]).unwrap();
        assert_eq!(alu.x, -9);
    }

    #[test]
    fn test_3x_check() {
        const INPUT: &str = "inp z\ninp x\nmul z 3\neql z x\n";
        let v: Instructions = INPUT.parse().unwrap();

        let alu = ArithmeticLogicUnit::new(&v, &[3, 6]).unwrap();
        assert_eq!(alu.z, 0);

        let alu = ArithmeticLogicUnit::new(&v, &[2, 6]).unwrap();
        assert_eq!(alu.z, 1);

        let alu = ArithmeticLogicUnit::new(&v, &[3, 9]).unwrap();
        assert_eq!(alu.z, 1);
    }

    #[test]
    fn test_binary_converter() {
        const INPUT: &str = "inp w\nadd z w\nmod z 2\ndiv w 2\nadd y w\nmod y 2\ndiv w 2\nadd x w\nmod x 2\ndiv w 2\nmod w 2\n";
        let v: Instructions = INPUT.parse().unwrap();

        let alu = ArithmeticLogicUnit::new(&v, &[0]).unwrap();
        assert_eq!(alu.w, 0);
        assert_eq!(alu.x, 0);
        assert_eq!(alu.y, 0);
        assert_eq!(alu.z, 0);

        let alu = ArithmeticLogicUnit::new(&v, &[1]).unwrap();
        assert_eq!(alu.w, 0);
        assert_eq!(alu.x, 0);
        assert_eq!(alu.y, 0);
        assert_eq!(alu.z, 1);

        let alu = ArithmeticLogicUnit::new(&v, &[2]).unwrap();
        assert_eq!(alu.w, 0);
        assert_eq!(alu.x, 0);
        assert_eq!(alu.y, 1);
        assert_eq!(alu.z, 0);

        let alu = ArithmeticLogicUnit::new(&v, &[3]).unwrap();
        assert_eq!(alu.w, 0);
        assert_eq!(alu.x, 0);
        assert_eq!(alu.y, 1);
        assert_eq!(alu.z, 1);

        let alu = ArithmeticLogicUnit::new(&v, &[4]).unwrap();
        assert_eq!(alu.w, 0);
        assert_eq!(alu.x, 1);
        assert_eq!(alu.y, 0);
        assert_eq!(alu.z, 0);

        let alu = ArithmeticLogicUnit::new(&v, &[5]).unwrap();
        assert_eq!(alu.w, 0);
        assert_eq!(alu.x, 1);
        assert_eq!(alu.y, 0);
        assert_eq!(alu.z, 1);

        let alu = ArithmeticLogicUnit::new(&v, &[6]).unwrap();
        assert_eq!(alu.w, 0);
        assert_eq!(alu.x, 1);
        assert_eq!(alu.y, 1);
        assert_eq!(alu.z, 0);

        let alu = ArithmeticLogicUnit::new(&v, &[7]).unwrap();
        assert_eq!(alu.w, 0);
        assert_eq!(alu.x, 1);
        assert_eq!(alu.y, 1);
        assert_eq!(alu.z, 1);

        let alu = ArithmeticLogicUnit::new(&v, &[8]).unwrap();
        assert_eq!(alu.w, 1);
        assert_eq!(alu.x, 0);
        assert_eq!(alu.y, 0);
        assert_eq!(alu.z, 0);

        let alu = ArithmeticLogicUnit::new(&v, &[9]).unwrap();
        assert_eq!(alu.w, 1);
        assert_eq!(alu.x, 0);
        assert_eq!(alu.y, 0);
        assert_eq!(alu.z, 1);

        let alu = ArithmeticLogicUnit::new(&v, &[10]).unwrap();
        assert_eq!(alu.w, 1);
        assert_eq!(alu.x, 0);
        assert_eq!(alu.y, 1);
        assert_eq!(alu.z, 0);

        let alu = ArithmeticLogicUnit::new(&v, &[11]).unwrap();
        assert_eq!(alu.w, 1);
        assert_eq!(alu.x, 0);
        assert_eq!(alu.y, 1);
        assert_eq!(alu.z, 1);

        let alu = ArithmeticLogicUnit::new(&v, &[12]).unwrap();
        assert_eq!(alu.w, 1);
        assert_eq!(alu.x, 1);
        assert_eq!(alu.y, 0);
        assert_eq!(alu.z, 0);

        let alu = ArithmeticLogicUnit::new(&v, &[13]).unwrap();
        assert_eq!(alu.w, 1);
        assert_eq!(alu.x, 1);
        assert_eq!(alu.y, 0);
        assert_eq!(alu.z, 1);

        let alu = ArithmeticLogicUnit::new(&v, &[14]).unwrap();
        assert_eq!(alu.w, 1);
        assert_eq!(alu.x, 1);
        assert_eq!(alu.y, 1);
        assert_eq!(alu.z, 0);

        let alu = ArithmeticLogicUnit::new(&v, &[15]).unwrap();
        assert_eq!(alu.w, 1);
        assert_eq!(alu.x, 1);
        assert_eq!(alu.y, 1);
        assert_eq!(alu.z, 1);
    }
}
