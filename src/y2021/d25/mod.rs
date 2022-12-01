pub(crate) mod part1;
// pub(crate) mod part2;

use std::borrow::Cow;
use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, Eq, derive_more::IsVariant)]
enum Tile {
    Vacant,
    Eastward,
    Southward,
}

impl Tile {
    fn tick(&self) -> Position {
        match self {
            Tile::Vacant => (0, 0),
            Tile::Eastward => (1, 0),
            Tile::Southward => (0, 1),
        }
    }
}

impl TryFrom<u8> for Tile {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'.' => Ok(Self::Vacant),
            b'>' => Ok(Self::Eastward),
            b'v' => Ok(Self::Southward),
            _ => Err(anyhow::anyhow!("invalid character: {}", value)),
        }
    }
}

type Position = (usize, usize);

#[derive(Clone)]
pub struct OceanFloor {
    width: usize,
    tiles: Box<[Tile]>,
}

impl OceanFloor {
    pub fn tick_to_deadlock(&self) -> (usize, Self) {
        let mut tick = 0;
        let mut next = self.to_owned();
        loop {
            tick += 1;
            match next.tick() {
                Some(v) => next = v,
                None => return (tick, next),
            }
        }
    }

    pub fn tick(&self) -> Option<Self> {
        let east = self.tick_impl(Tile::Eastward);
        match east {
            Some(ref new) => new,
            None => self,
        }
        .tick_impl(Tile::Southward)
        .or(east)
    }

    fn tick_impl(&self, needle: Tile) -> Option<Self> {
        let mut new = Cow::Borrowed(self);
        for (lhs, rhs) in self
            .tiles
            .iter()
            .enumerate()
            .filter_map(|(idx, t)| (*t == needle).then(|| self.idx_to_pos(idx)))
            .filter_map(|idx| {
                let (pos, t) = self.peek(needle, idx);
                t.is_vacant().then_some((idx, pos))
            })
        {
            // cow will clone self the first time we claim a mutable ref
            new.to_mut().swap(lhs, rhs);
        }
        // if we never took a mutable ref, then we made no moves
        match new {
            Cow::Owned(v) => Some(v),
            Cow::Borrowed(_) => None,
        }
    }

    fn depth(&self) -> usize {
        self.tiles.len() / self.width
    }

    fn idx_to_pos(&self, idx: usize) -> Position {
        (idx % self.width, idx / self.width)
    }

    fn pos_to_idx(&self, (x, y): Position) -> usize {
        (y * self.width) + x
    }

    fn peek(&self, needle: Tile, (x, y): Position) -> (Position, Tile) {
        let (dx, dy) = needle.tick();
        let new_pos = ((x + dx) % self.width, (y + dy) % self.depth());
        let new_idx = self.pos_to_idx(new_pos);
        (new_pos, self.tiles[new_idx])
    }

    fn swap(&mut self, lhs: Position, rhs: Position) {
        let lhs = self.pos_to_idx(lhs);
        let rhs = self.pos_to_idx(rhs);
        self.tiles.swap(lhs, rhs);
    }
}

impl FromStr for OceanFloor {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut width = 0;
        let mut tiles = Vec::with_capacity(s.len());
        for line in s.lines() {
            if width == 0 {
                width = line.len();
            }
            if line.len() != width {
                return Err(anyhow::anyhow!("invalid input line: {}", s));
            }
            for c in line.bytes() {
                let t: Tile = c.try_into()?;
                tiles.push(t)
            }
        }
        Ok(Self {
            width,
            tiles: tiles.into_boxed_slice(),
        })
    }
}
