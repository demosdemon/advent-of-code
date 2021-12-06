pub mod part1;
pub mod part2;

struct Ocean(Vec<u8>);

impl Ocean {
    pub fn count(self, days: usize) -> i128 {
        let mut lanterns = [0i128; 9];
        for lantern in self.0 {
            assert!(lantern < 9);
            lanterns[lantern as usize] += 1;
        }
        for _ in 0..days {
            let finished = lanterns[0];
            for idx in 0..8 {
                lanterns[idx] = lanterns[idx + 1];
            }
            lanterns[8] = finished;
            lanterns[6] += finished;
        }
        lanterns.into_iter().sum()
    }
}
