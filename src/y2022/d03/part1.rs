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

// --- Day 3: Rucksack Reorganization ---
// One Elf has the important job of loading all of the rucksacks with supplies
// for the jungle journey. Unfortunately, that Elf didn't quite follow the
// packing instructions, and so a few items now need to be rearranged.
//
// Each rucksack has two large compartments. All items of a given type are meant
// to go into exactly one of the two compartments. The Elf that did the packing
// failed to follow this rule for exactly one item type per rucksack.
//
// The Elves have made a list of all of the items currently in each rucksack
// (your puzzle input), but they need your help finding the errors. Every item
// type is identified by a single lowercase or uppercase letter (that is, a and
// A refer to different types of items).
//
// The list of items for each rucksack is given as characters all on a single
// line. A given rucksack always has the same number of items in each of its two
// compartments, so the first half of the characters represent items in the
// first compartment, while the second half of the characters represent items in
// the second compartment.
//
// For example, suppose you have the following list of contents from six
// rucksacks:
//
// vJrwpWtwJgWrhcsFMMfFFhFp
// jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
// PmmdzqPrVvPwwTWBwg
// wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
// ttgJtRGJQctTZtZT
// CrZsJsPPZsGzwwsLwLmpwMDw
//
// The first rucksack contains the items vJrwpWtwJgWrhcsFMMfFFhFp, which means
// its first compartment contains the items vJrwpWtwJgWr, while the second
// compartment contains the items hcsFMMfFFhFp. The only item type that appears
// in both compartments is lowercase p. The second rucksack's compartments
// contain jqHRNqRjqzjGDLGL and rsFMfFZSrLrFZsSL. The only item type that
// appears in both compartments is uppercase L. The third rucksack's
// compartments contain PmmdzqPrV and vPwwTWBwg; the only common item type is
// uppercase P. The fourth rucksack's compartments only share item type v.
// The fifth rucksack's compartments only share item type t.
// The sixth rucksack's compartments only share item type s.
// To help prioritize item rearrangement, every item type can be converted to a
// priority:
//
// Lowercase item types a through z have priorities 1 through 26.
// Uppercase item types A through Z have priorities 27 through 52.
// In the above example, the priority of the item type that appears in both
// compartments of each rucksack is 16 (p), 38 (L), 42 (P), 22 (v), 20 (t), and
// 19 (s); the sum of these is 157.
//
// Find the item type that appears in both compartments of each rucksack. What
// is the sum of the priorities of those item types?
//

use std::cmp::Ordering;

use super::Sack;
use super::Sacks;

fn solve_sack(mut sack: Sack) -> usize {
    let (left, right) = sack.partition_mut();
    left.sort_unstable();
    right.sort_unstable();

    let mut left_iter = left.iter();
    let mut right_iter = right.iter();

    let mut left_item = left_iter.next();
    let mut right_item = right_iter.next();

    loop {
        match (left_item.as_ref(), right_item.as_ref()) {
            (Some(&left), Some(&right)) => match left.cmp(right) {
                Ordering::Less => {
                    left_item = left_iter.next();
                }
                Ordering::Equal => {
                    return left.score();
                }
                Ordering::Greater => {
                    right_item = right_iter.next();
                }
            },
            _ => unreachable!("no common item"),
        }
    }
}

fn solve(sacks: Sacks) -> usize {
    sacks.0.iter().cloned().map(solve_sack).sum()
}

#[cfg(test)]
mod test {
    ::aoc::tests_for_problem!(super::solve, {
        example => 157,
        live => 7831,
    });
}
