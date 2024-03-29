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

mod parser;
pub(crate) mod part1;
pub(crate) mod part2;

#[derive(derive_more::Display, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[display(fmt = "move {quantity} from {from} to {to}")]
struct Instruction {
    quantity: u8,
    from: u8,
    to: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Slot(Option<char>);

#[derive(macros::TryFromStr)]
struct Input {
    rows: Vec<Vec<Slot>>,
    instructions: Vec<Instruction>,
}

::aoc::derive_FromStr_for_nom!(Input, parser::input);

#[derive(Debug)]
struct State {
    columns: Vec<Vec<char>>,
}

impl State {
    fn top(&self) -> String {
        self.columns
            .iter()
            .map(|column| *column.last().unwrap_or(&' '))
            .collect()
    }

    fn execute(&mut self, inst: Instruction, maintain_order: bool) {
        let Instruction {
            mut quantity,
            from,
            to,
        } = inst;

        if from == to {
            return;
        }

        if maintain_order {
            let mut buf = Vec::with_capacity(quantity as usize);
            while quantity > 0 {
                let c = self.columns[(from - 1) as usize]
                    .pop()
                    .expect("invalid instruction");
                buf.push(c);
                quantity -= 1;
            }

            buf.reverse();
            self.columns[(to - 1) as usize].extend(buf);
        } else {
            while quantity > 0 {
                let c = self.columns[(from - 1) as usize]
                    .pop()
                    .expect("invalid instruction");
                self.columns[(to - 1) as usize].push(c);
                quantity -= 1;
            }
        }
    }
}

impl From<Vec<Vec<Slot>>> for State {
    fn from(rows: Vec<Vec<Slot>>) -> Self {
        // input rows are top to bottom, left to right
        // output columns are left to right, bottom to top

        if rows.is_empty() {
            return Self {
                columns: Vec::new(),
            };
        }

        let mut columns: Vec<Vec<char>> = vec![Vec::new(); rows[0].len()];

        for row in rows.into_iter().rev() {
            for (Slot(slot), column) in row.into_iter().zip(columns.iter_mut()) {
                if let Some(slot) = slot {
                    column.push(slot);
                }
            }
        }

        Self { columns }
    }
}
