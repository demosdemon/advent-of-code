// Copyright (c) 2021-2022 Brandon LeBlanc <brandon@leblanc.codes>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use std::fmt::Display;

use itertools::Itertools;

mod parser;
pub(crate) mod part1;
pub(crate) mod part2;

enum Operation {
    Add,
    Multiply,
}

struct Item(usize);

struct Monkey {
    #[allow(dead_code)]
    id: usize,
    items: Vec<Item>,
    operation: Operation,
    operand: Option<usize>,
    divisor: usize,
    toss_true: usize,
    toss_false: usize,
    inspected: usize,
}

::aoc::derive_FromStr_for_nom!(Monkey, parser::monkey);

impl Monkey {
    fn tick(&mut self, mut func: impl FnMut(usize) -> usize) -> Vec<(usize, Item)> {
        let mut tosses = Vec::new();
        for Item(item) in self.items.drain(..) {
            self.inspected += 1;
            let item = match self.operation {
                Operation::Add => item + self.operand.unwrap_or(item),
                Operation::Multiply => item * self.operand.unwrap_or(item),
            };
            let item = func(item);
            if item % self.divisor == 0 {
                tosses.push((self.toss_true, Item(item)));
            } else {
                tosses.push((self.toss_false, Item(item)));
            }
        }
        tosses
    }
}

struct MonkeyState(Vec<Monkey>);

impl Display for MonkeyState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (idx, monkey) in self.0.iter().enumerate() {
            let s = monkey.items.iter().map(|Item(item)| item).join(", ");
            writeln!(f, "Monkey {idx}: {s}")?;
        }
        Ok(())
    }
}

impl MonkeyState {
    fn tick(&mut self, mut func: impl FnMut(usize) -> usize) {
        for idx in 0..self.0.len() {
            let tosses = self.0[idx].tick(&mut func);
            for (id, item) in tosses {
                self.0[id].items.push(item);
            }
        }
    }

    fn level(&self) -> usize {
        self.0
            .iter()
            .map(|m| m.inspected)
            .sorted()
            .rev()
            .take(2)
            .product()
    }
}

::aoc::derive_FromStr_for_nom!(MonkeyState, parser::monkey_state);

impl TryFrom<&str> for MonkeyState {
    type Error = <MonkeyState as ::std::str::FromStr>::Err;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        s.parse()
    }
}
