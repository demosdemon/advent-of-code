/*
    --- Day 3: Binary Diagnostic ---
    The submarine has been making some odd creaking noises, so you ask it to produce a
    diagnostic report just in case.

    The diagnostic report (your puzzle input) consists of a list of binary numbers
    which, when decoded properly, can tell you many useful things about the conditions
    of the submarine. The first parameter to check is the power consumption.

    You need to use the binary numbers in the diagnostic report to generate two new
    binary numbers (called the gamma rate and the epsilon rate). The power consumption
    can then be found by multiplying the gamma rate by the epsilon rate.

    Each bit in the gamma rate can be determined by finding the most common bit in the
    corresponding position of all numbers in the diagnostic report. For example, given
    the following diagnostic report:

        00100
        11110
        10110
        10111
        10101
        01111
        00111
        11100
        10000
        11001
        00010
        01010

    Considering only the first bit of each number, there are five 0 bits and seven 1
    bits. Since the most common bit is 1, the first bit of the gamma rate is 1.

    The most common second bit of the numbers in the diagnostic report is 0, so the
    second bit of the gamma rate is 0.

    The most common value of the third, fourth, and fifth bits are 1, 1, and 0,
    respectively, and so the final three bits of the gamma rate are 110.

    So, the gamma rate is the binary number 10110, or 22 in decimal.

    The epsilon rate is calculated in a similar way; rather than use the most common
    bit, the least common bit from each position is used. So, the epsilon rate is 01001,
     or 9 in decimal. Multiplying the gamma rate (22) by the epsilon rate (9) produces
     the power consumption, 198.

    Use the binary numbers in your diagnostic report to calculate the gamma rate and
    epsilon rate, then multiply them together. What is the power consumption of the
    submarine? (Be sure to represent your answer in decimal, not binary.)
*/

use itertools::Itertools;

use crate::IntoAnswer;

use super::bit::Bit;
use super::line::Line;

#[derive(Default, Debug)]
struct Answer {
    pub zeros: Vec<usize>,
    pub ones: Vec<usize>,
}

crate::derive_FromIterator_for_Extend!(Answer, Line);
crate::derive_FromStr_for_FromIterator!(Answer, Line);

impl Answer {
    fn assert_len(&mut self, len: usize) {
        if self.zeros.is_empty() {
            self.zeros.resize(len, 0);
        }
        assert_eq!(self.zeros.len(), len);

        if self.ones.is_empty() {
            self.ones.resize(len, 0);
        }
        assert_eq!(self.ones.len(), len);
    }
}

impl IntoAnswer for Answer {
    type Output = isize;

    fn into_answer(self) -> isize {
        let gamma: Line = itertools::zip(self.zeros, self.ones)
            .map(|(zeros, ones)| zeros <= ones)
            .map_into::<Bit>()
            .collect();
        let epsilon = !gamma.clone();
        let gamma: usize = (&gamma).into();
        let epsilon: usize = (&epsilon).into();
        (gamma * epsilon) as isize
    }
}

impl Extend<Line> for Answer {
    fn extend<T: IntoIterator<Item = Line>>(&mut self, iter: T) {
        for line in iter {
            self.assert_len(line.len());
            for (idx, b) in line.into_iter().enumerate() {
                match b {
                    Bit::Zero => self.zeros[idx] += 1,
                    Bit::One => self.ones[idx] += 1,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    crate::tests_for_problem!(super::Answer, {
        example => 198,
        live => 4103154,
    });
}
