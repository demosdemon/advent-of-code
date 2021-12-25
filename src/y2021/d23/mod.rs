pub(crate) mod part1;
pub(crate) mod part2;

use std::{
    collections::{BTreeMap, BinaryHeap},
    ops::Range,
    str::FromStr,
};

use anyhow::Context;
use arrayvec::ArrayVec;
use itertools::Itertools;

const HALLWAY_COLUMNS: [usize; 7] = [1, 2, 4, 6, 8, 10, 11];
const ROOM_COLUMNS: [usize; 4] = [3, 5, 7, 9];

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl Amphipod {
    fn fuel_cost(&self) -> usize {
        match self {
            Self::Amber => 1,
            Self::Bronze => 10,
            Self::Copper => 100,
            Self::Desert => 1000,
        }
    }

    fn desired_room(&self) -> usize {
        ROOM_COLUMNS[*self as usize]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, derive_more::IsVariant)]
enum Tile {
    Wall,
    Vacant,
    Amphipod(Amphipod),
    Void,
}

impl Tile {
    fn amphipod(&self) -> Option<&Amphipod> {
        match self {
            Self::Amphipod(v) => Some(v),
            _ => None,
        }
    }
}

impl TryFrom<u8> for Tile {
    type Error = anyhow::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'#' => Ok(Self::Wall),
            b'.' => Ok(Self::Vacant),
            b'A' => Ok(Self::Amphipod(Amphipod::Amber)),
            b'B' => Ok(Self::Amphipod(Amphipod::Bronze)),
            b'C' => Ok(Self::Amphipod(Amphipod::Copper)),
            b'D' => Ok(Self::Amphipod(Amphipod::Desert)),
            b' ' => Ok(Self::Void),
            _ => Err(anyhow::anyhow!("invalid character: {}", value)),
        }
    }
}

const HALLWAY_ROW: usize = 1;
const AMPHIPOD_COUNT: usize = 4;
const ROW_WIDTH: usize = 13;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Maze<const ROWS: usize> {
    tiles: [[Tile; ROW_WIDTH]; ROWS],
}

impl<const ROWS: usize> Maze<ROWS> {
    pub fn shortest_path(&self) -> usize {
        let mut dist = BTreeMap::new();
        let mut heap = BinaryHeap::new();
        heap.push((0isize, self.to_owned()));
        while let Some((cost, next)) = heap.pop() {
            let ucost = (-cost) as usize;
            if next.is_complete() {
                return ucost;
            }
            if let Some(&c) = dist.get(&next) {
                if c < ucost {
                    continue;
                }
            }
            for (next, next_cost) in next.moves() {
                let next_cost = ucost + next_cost;
                match dist.entry(next.clone()) {
                    std::collections::btree_map::Entry::Vacant(o) => {
                        o.insert(next_cost);
                        heap.push((-(next_cost as isize), next));
                    }
                    std::collections::btree_map::Entry::Occupied(o) => {
                        let prev = o.into_mut();
                        if next_cost < *prev {
                            *prev = next_cost;
                            heap.push((-(next_cost as isize), next));
                        }
                    }
                }
            }
        }
        unreachable!()
    }

    fn valid(&self) -> bool {
        let rows = Self::room_row_range().len();
        self.tiles
            .iter()
            .flatten()
            .flat_map(Tile::amphipod)
            .counts()
            .into_values()
            .flat_map(|v| (v == rows).then(|| v))
            .sum::<usize>()
            == rows * AMPHIPOD_COUNT
    }

    fn iter_hallway(&self, src: usize, dst: usize) -> impl Iterator<Item = (usize, &Tile)> + '_ {
        let (_, hwc, _) = trisect(&HALLWAY_COLUMNS, src, dst);
        hwc.iter()
            .map(move |&col| (col, &self.tiles[HALLWAY_ROW][col]))
    }

    fn iter_occupied_hallway(&self) -> impl Iterator<Item = (usize, &Amphipod)> + '_ {
        HALLWAY_COLUMNS
            .into_iter()
            .filter_map(|col| match &self.tiles[HALLWAY_ROW][col] {
                Tile::Amphipod(p) => Some((col, p)),
                _ => None,
            })
    }

    fn iter_accessible_hallway(&self, src: usize) -> impl Iterator<Item = &'static usize> + '_ {
        let mid = HALLWAY_COLUMNS.binary_search(&src).unwrap_err();
        let (lhs, rhs) = HALLWAY_COLUMNS.split_at(mid);
        let lhs = lhs
            .iter()
            .rev()
            .take_while(|&&col| self.tiles[HALLWAY_ROW][col].is_vacant());
        let rhs = rhs
            .iter()
            .take_while(|&&col| self.tiles[HALLWAY_ROW][col].is_vacant());
        itertools::interleave(lhs, rhs)
    }

    fn is_hallway_open(&self, src: usize, dst: usize) -> bool {
        self.iter_hallway(src, dst).all(|(_, t)| t.is_vacant())
    }

    const fn room_row_range() -> Range<usize> {
        HALLWAY_ROW + 1..ROWS - 1
    }

    fn iter_room(&self, room: usize) -> impl Iterator<Item = (usize, &Tile)> + '_ {
        Self::room_row_range().map(move |row| (row, &self.tiles[row][room]))
    }

    fn room_vacancy(&self, room: usize) -> Option<usize> {
        // return the index of the last vacant cell within a room iff the remaining
        // cells (if any) are occuped by valid pods
        let mut last_vacant = None;
        for (row, t) in self.iter_room(room) {
            match t {
                Tile::Vacant => {
                    last_vacant = Some(row);
                }
                Tile::Amphipod(p) => {
                    if last_vacant.is_none() || p.desired_room() != room {
                        return None;
                    }
                }
                _ => unreachable!(),
            }
        }
        last_vacant
    }

    fn room_eviction(&self, room: usize) -> Option<(usize, &Amphipod)> {
        // return the index of the first non-vacant cell within a room if the cell or
        // any of following cells are occupied by invalid pods
        let mut first_occupied = None;
        for (row, t) in self.iter_room(room) {
            match t {
                Tile::Vacant => {
                    // there should not be any bubbles
                    debug_assert!(first_occupied.is_none());
                }
                Tile::Amphipod(p) => {
                    if first_occupied.is_none() {
                        first_occupied = Some((row, p));
                    }
                    if p.desired_room() != room {
                        return first_occupied;
                    }
                }
                _ => unreachable!(),
            }
        }
        // we fell through the loop meaning the occupied tiles we found (if any) are
        // valid and should not be evicted
        None
    }

    fn is_complete(&self) -> bool {
        ROOM_COLUMNS
            .into_iter()
            .flat_map(|room| self.iter_room(room).map(move |(_, t)| (room, t)))
            .all(|(room, t)| match t {
                Tile::Amphipod(p) => p.desired_room() == room,
                _ => false,
            })
    }

    fn moves(&self) -> Vec<(Self, usize)> {
        let mut states = Vec::new();
        // move from hallway into room
        for (col, p) in self.iter_occupied_hallway() {
            let room = p.desired_room();
            // is hallway betwen {col} and {room} occupied?
            if self.is_hallway_open(col, room) {
                // can amphipod move into room
                if let Some(row) = self.room_vacancy(room) {
                    states.push((
                        self.swap((HALLWAY_ROW, col), (row, room)),
                        (h_dist(room, col) + v_dist(row)) * p.fuel_cost(),
                    ));
                }
            }
        }

        // move out of room
        for col in ROOM_COLUMNS {
            if let Some((o_row, p)) = self.room_eviction(col) {
                let room = p.desired_room();
                let cost = p.fuel_cost();

                // can pod move into room
                if let Some(n_row) = self.room_vacancy(room) {
                    if self.is_hallway_open(col, room) {
                        states.push((
                            self.swap((o_row, col), (n_row, room)),
                            (h_dist(room, col) + v_dist(o_row) + v_dist(n_row)) * cost,
                        ));
                        continue;
                    }
                }

                // move into hallway
                for &hw in self.iter_accessible_hallway(col) {
                    states.push((
                        self.swap((o_row, col), (HALLWAY_ROW, hw)),
                        (h_dist(col, hw) + v_dist(o_row)) * cost,
                    ));
                }
            }
        }
        states
    }

    fn swap(&self, lhs: (usize, usize), rhs: (usize, usize)) -> Self {
        let ((lhs_row, lhs_col), (rhs_row, rhs_col)) = ::aoc::ordered!(lhs, rhs);
        let mut new = self.to_owned();
        if lhs_row == rhs_row {
            let row = &mut new.tiles[lhs_row];
            row.swap(lhs_col, rhs_col);
        } else {
            let (lhs, rhs) = new.tiles.split_at_mut(rhs_row);
            std::mem::swap(&mut lhs[lhs_row][lhs_col], &mut rhs[0][rhs_col]);
        }
        debug_assert!(new.valid());
        new
    }
}

impl<const ROWS: usize> FromStr for Maze<ROWS> {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = Self {
            tiles: s
                .lines()
                .map(|line| {
                    if line.len() <= ROW_WIDTH {
                        Ok(line
                            .bytes()
                            .chain((0..ROW_WIDTH - line.len()).map(|_| b' '))
                            .map(Tile::try_from)
                            .collect::<Result<ArrayVec<_, ROW_WIDTH>, _>>()?
                            .into_inner()
                            .unwrap())
                    } else {
                        Err(anyhow::anyhow!("invalid input: {}", line))
                    }
                })
                .collect::<Result<ArrayVec<_, ROWS>, _>>()?
                .into_inner()
                .ok()
                .context("short read on input")?,
        };
        if v.valid() {
            Ok(v)
        } else {
            Err(anyhow::anyhow!("maze is not valid: {:?}", v))
        }
    }
}

const fn h_dist(room: usize, col: usize) -> usize {
    if room < col {
        col - room
    } else {
        room - col
    }
}

const fn v_dist(row: usize) -> usize {
    row - HALLWAY_ROW
}

fn trisect<V: Ord>(slice: &[V], src: V, dst: V) -> (&[V], &[V], &[V]) {
    let (src, dst) = ::aoc::ordered!(src, dst);
    let mid = slice.binary_search(&dst).unwrap_or_else(|v| v);
    let (ab, c) = slice.split_at(mid);
    let mid = ab.binary_search(&src).map(|v| v + 1).unwrap_or_else(|v| v);
    let (a, b) = ab.split_at(mid);
    (a, b, c)
}
