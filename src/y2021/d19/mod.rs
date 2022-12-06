pub(crate) mod part1;
pub(crate) mod part2;

pub(crate) mod coordinate;
pub(crate) mod parser;

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt::Display;

use coordinate::Coordinate;
use itertools::Itertools;

#[derive(Default)]
struct BeaconSet {
    scanners: Vec<Coordinate>,
    beacons: HashSet<Coordinate>,
}

impl BeaconSet {
    fn merge(&mut self, scanner: &Scanner, angle: usize) -> bool {
        if self.beacons.is_empty() {
            self.beacons.extend(scanner.rotate(angle));
            self.scanners.push(Coordinate::new(0, 0, 0));
            return true;
        }
        let rotation = scanner.rotate(angle).collect::<Vec<_>>();
        let mut distances = self
            .beacons
            .iter()
            .cartesian_product(&rotation)
            .map(|(lhs, rhs)| lhs - rhs)
            .fold(HashMap::<Coordinate, usize>::new(), |mut map, dist| {
                *map.entry(dist).or_default() += 1;
                map
            })
            .into_iter()
            .filter(|(_, c)| *c >= 12)
            .collect::<Vec<_>>();
        distances.sort_unstable_by_key(|(_, c)| *c);
        if let Some((delta, _)) = distances.pop() {
            self.beacons.extend(rotation.iter().map(|b| b + &delta));
            self.scanners.push(delta);
            true
        } else {
            false
        }
    }
}

impl<'a> From<&'a [Scanner]> for BeaconSet {
    fn from(reports: &'a [Scanner]) -> BeaconSet {
        let mut new = BeaconSet::default();
        let mut reports = reports.iter().collect::<VecDeque<_>>();
        'reports: while let Some(report) = reports.pop_front() {
            for angle in 0..coordinate::MAX_ROTATIONS {
                if new.merge(report, angle) {
                    continue 'reports;
                }
            }
            reports.push_back(report);
        }
        new
    }
}

#[derive(Debug, macros::TryFromStr)]
struct Scanner {
    idx: usize,
    beacons: Vec<Coordinate>,
}

impl Scanner {
    fn rotate(&self, angle: usize) -> impl Iterator<Item = Coordinate> + '_ {
        self.beacons.iter().map(move |b| b.rotate(angle))
    }
}

impl Display for Scanner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "--- scanner {} ---", self.idx)?;
        writeln!(
            f,
            "{}",
            self.beacons.iter().map(ToString::to_string).join("\n")
        )?;
        Ok(())
    }
}

::aoc::derive_FromStr_for_nom!(Scanner, parser::scanner);

#[derive(macros::TryFromStr)]
pub struct Report(Vec<Scanner>);

impl Display for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.iter().map(ToString::to_string).join("\n"))
    }
}

::aoc::derive_FromStr_for_nom!(Report, parser::report);

#[cfg(test)]
mod tests {
    macros::test_roundtrip!(
        super::Report,
        include_str!("inputs/example"),
        include_str!("inputs/live")
    );
}
