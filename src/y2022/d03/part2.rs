// --- Part Two ---
// As you finish identifying the misplaced items, the Elves come to you with
// another issue.
//
// For safety, the Elves are divided into groups of three. Every Elf carries a
// badge that identifies their group. For efficiency, within each group of three
// Elves, the badge is the only item type carried by all three Elves. That is,
// if a group's badge is item type B, then all three Elves will have item type B
// somewhere in their rucksack, and at most two of the Elves will be carrying
// any other item type.
//
// The problem is that someone forgot to put this year's updated authenticity
// sticker on the badges. All of the badges need to be pulled out of the
// rucksacks so the new authenticity stickers can be attached.
//
// Additionally, nobody wrote down which item type corresponds to each group's
// badges. The only way to tell which item type is the right one is by finding
// the one item type that is common between all three Elves in each group.
//
// Every set of three lines in your list corresponds to a single group, but each
// group can have a different badge item type. So, in the above example, the
// first group's rucksacks are the first three lines:
//
// vJrwpWtwJgWrhcsFMMfFFhFp
// jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
// PmmdzqPrVvPwwTWBwg
// And the second group's rucksacks are the next three lines:
//
// wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
// ttgJtRGJQctTZtZT
// CrZsJsPPZsGzwwsLwLmpwMDw
// In the first group, the only item type that appears in all three rucksacks is
// lowercase r; this must be their badges. In the second group, their badge item
// type must be Z.
//
// Priorities for these items must still be found to organize the sticker
// attachment efforts: here, they are 18 (r) for the first group and 52 (Z) for
// the second group. The sum of these is 70.
//
// Find the item type that corresponds to the badges of each three-Elf group.
// What is the sum of the priorities of those item types?
//

use std::cmp::Ordering;

use itertools::Itertools;

use super::Sack;

fn solve_chunk((mut s1, mut s2, mut s3): (Sack, Sack, Sack)) -> usize {
    s1.0.sort_unstable();
    s2.0.sort_unstable();
    s3.0.sort_unstable();

    let mut s1_iter = s1.0.into_iter();
    let mut s2_iter = s2.0.into_iter();
    let mut s3_iter = s3.0.into_iter();

    let mut s1 = s1_iter.next();
    let mut s2 = s2_iter.next();
    let mut s3 = s3_iter.next();

    loop {
        match (s1, s2, s3) {
            (Some(s1_), Some(s2_), Some(s3_)) => match (s1_.cmp(&s2_), s2_.cmp(&s3_)) {
                (Ordering::Equal, Ordering::Equal) => return s1_.score(),
                (Ordering::Less, _) => s1 = s1_iter.next(),
                (_, Ordering::Greater) => s3 = s3_iter.next(),
                _ => s2 = s2_iter.next(),
            },
            _ => unreachable!("no common item"),
        }
    }
}

fn solve(sacks: &super::Sacks) -> usize {
    sacks.0.iter().cloned().tuples().map(solve_chunk).sum()
}

#[cfg(test)]
mod test {
    ::aoc::tests_for_problem!(super::solve, {
        example => 70,
        live => 2683,
    });
}
