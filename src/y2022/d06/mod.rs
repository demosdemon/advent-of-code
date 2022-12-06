use itertools::Itertools;

pub(crate) mod part1;
pub(crate) mod part2;

fn find_unique_len(s: &[u8], len: usize) -> usize {
    s.windows(len).position(|w| w.iter().all_unique()).unwrap() + len
}
