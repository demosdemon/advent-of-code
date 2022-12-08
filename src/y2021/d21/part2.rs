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

// --- Part Two ---
// Now that you're warmed up, it's time to play the real game.
//
// A second compartment opens, this time labeled Dirac dice. Out of it falls a
// single three-sided die.
//
// As you experiment with the die, you feel a little strange. An informational
// brochure in the compartment explains that this is a quantum die: when you
// roll it, the universe splits into multiple copies, one copy for each possible
// outcome of the die. In this case, rolling the die always splits the universe
// into three copies: one where the outcome of the roll was 1, one where it was
// 2, and one where it was 3.
//
// The game is played the same as before, although to prevent things from
// getting too far out of hand, the game now ends when either player's score
// reaches at least 21.
//
// Using the same starting positions as in the example above, player 1 wins in
// 444356092776315 universes, while player 2 merely wins in 341960390180808
// universes.
//
// Using your given starting positions, determine every possible outcome. Find
// the player that wins in more universes; in how many universes does that
// player win?

use std::collections::BTreeMap;
use std::ops::Not;

use itertools::iproduct;

use super::Input;
use super::State;

#[derive(Debug, Clone, Copy, derive_more::Add)]
struct Wins {
    p1: usize,
    p2: usize,
}

impl Wins {
    fn p1() -> Self {
        Self { p1: 1, p2: 0 }
    }

    fn p2() -> Self {
        Self { p1: 0, p2: 1 }
    }

    fn nil() -> Self {
        Self { p1: 0, p2: 0 }
    }

    fn max(self) -> usize {
        std::cmp::max(self.p1, self.p2)
    }
}

impl Not for Wins {
    type Output = Self;

    fn not(self) -> Self {
        Self {
            p1: self.p2,
            p2: self.p1,
        }
    }
}

#[derive(Default)]
struct Dirac {
    win_cache: BTreeMap<State, Wins>,
}

impl Dirac {
    const ROLLS: [usize; 3] = [1, 2, 3];
    const WIN: usize = 21;

    fn from_input(input: &Input) -> Wins {
        Self::default().roll(input.into())
    }

    fn roll(&mut self, state: State) -> Wins {
        if state.p1.score >= Self::WIN {
            Wins::p1()
        } else if state.p2.score >= Self::WIN {
            Wins::p2()
        } else if let Some(&wins) = self.win_cache.get(&state) {
            wins
        } else if let Some(&wins) = self.win_cache.get(&!state) {
            !wins
        } else {
            let wins = iproduct!(Self::ROLLS, Self::ROLLS, Self::ROLLS)
                .map(|(d1, d2, d3)| d1 + d2 + d3)
                .fold(Wins::nil(), |wins, roll| wins + self.roll(state + roll));
            self.win_cache.insert(state, wins);
            wins
        }
    }
}

pub fn solve(input: Input) -> usize {
    Dirac::from_input(&input).max()
}

#[cfg(test)]
mod tests {
    ::aoc::tests_for_problem!(super::solve, {
        example => 444356092776315,
        live => 152587196649184,
    });
}
