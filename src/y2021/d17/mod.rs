pub(crate) mod part1;
pub(crate) mod part2;

pub(crate) mod parser;

use std::ops::RangeInclusive;

#[derive(derive_more::Display)]
#[display(fmt = "target area: x={min_x}..{max_x}, y={min_y}..{max_y}\n")]
pub struct TargetArea {
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
}

::aoc::derive_FromStr_for_nom!(TargetArea, parser::target_area);

fn pos(dx: isize, dy: isize, t: isize) -> (isize, isize) {
    let y = dy * t - (t - 1) * t / 2;
    let x = if t < dx {
        (2 * dx - t + 1) * t / 2
    } else {
        dx * (dx + 1) / 2
    };
    (x, y)
}

impl TargetArea {
    pub fn range_x(&self) -> RangeInclusive<isize> {
        self.min_x..=self.max_x
    }

    pub fn range_y(&self) -> RangeInclusive<isize> {
        self.min_y..=self.max_y
    }

    pub fn intersects(&self, dx: isize, dy: isize) -> bool {
        let tmin = (((dy * dy - 2 * self.max_y) as f32).sqrt() + dy as f32).floor() as isize;
        let tmax = (((dy * dy - 2 * self.min_y) as f32).sqrt() + dy as f32).floor() as isize;
        for t in tmin..=tmax + 1 {
            let (x, y) = pos(dx, dy, t);
            if self.range_x().contains(&x) && self.range_y().contains(&y) {
                return true;
            }
        }
        false
    }

    pub fn range(&self) -> impl Iterator<Item = (isize, isize)> + '_ {
        let min_dy = self.min_y;
        let max_dy = -self.min_y;
        let min_dx = ((2.0 * self.min_x as f32).sqrt() - 1.0).floor() as isize;
        let max_dx = self.max_x;

        (min_dx..=max_dx).flat_map(move |dx| {
            (min_dy..=max_dy).filter_map(move |dy| self.intersects(dx, dy).then_some((dx, dy)))
        })
    }
}

#[cfg(test)]
mod tests {
    fn solve(s: &super::TargetArea) -> String {
        s.to_string()
    }

    ::aoc::tests_for_problem!(solve, {
        example => include_str!("inputs/example"),
        live => include_str!("inputs/live"),
    });
}
