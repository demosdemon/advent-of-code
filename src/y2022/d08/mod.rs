pub(crate) mod part1;
pub(crate) mod part2;

use ::aoc::matrix::Matrix;
use aoc::matrix::Position;
use aoc::matrix::RelativePosition;

struct Tree {
    height: u8,
    visible: bool,
}

struct Camp {
    height: u8,
    score: usize,
}

impl From<u8> for Tree {
    fn from(value: u8) -> Self {
        Self {
            height: value,
            visible: false,
        }
    }
}

impl From<u8> for Camp {
    fn from(value: u8) -> Self {
        Self {
            height: value,
            score: 0,
        }
    }
}

struct Forest<T>(Matrix<T>);

impl From<&str> for Forest<u8> {
    fn from(value: &str) -> Self {
        Self(
            value
                .lines()
                .map(|s| s.bytes().map(::aoc::chardigit))
                .collect(),
        )
    }
}

impl From<Forest<u8>> for Forest<Tree> {
    fn from(value: Forest<u8>) -> Self {
        Self(value.0.map(Tree::from))
    }
}

impl From<Forest<u8>> for Forest<Camp> {
    fn from(value: Forest<u8>) -> Self {
        Self(value.0.map(Camp::from))
    }
}

impl Forest<u8> {
    fn visible(self) -> usize {
        Forest::<Tree>::from(self).visible()
    }

    fn score(self) -> usize {
        Forest::<Camp>::from(self).score()
    }
}

const POS: [RelativePosition; 4] = [
    RelativePosition::TopCenter,
    RelativePosition::MiddleRight,
    RelativePosition::BottomCenter,
    RelativePosition::MiddleLeft,
];

impl Forest<Tree> {
    fn visible(&mut self) -> usize {
        let rows = self.0.depth();
        let cols = self.0.width();

        for row in 0..rows {
            for col in 0..cols {
                let pos = Position::new(row, col);
                let &Tree { height, visible } = self.0.get(pos).unwrap();
                if visible {
                    continue;
                }

                for rel in POS {
                    let mut ipos = self.0.relative_pos(pos, rel);
                    let mut visible = true;

                    while let Some(pos) = ipos {
                        if let Some(t) = self.0.get(pos) {
                            if t.height >= height {
                                visible = false;
                                break;
                            }

                            ipos = self.0.relative_pos(pos, rel);
                        }
                    }

                    if visible {
                        self.0.get_mut(pos).unwrap().visible = true;
                        break;
                    }
                }
            }
        }

        self.0.iter().map(|(_, t)| t.visible as usize).sum()
    }
}

impl Forest<Camp> {
    fn score(&mut self) -> usize {
        let rows = self.0.depth();
        let cols = self.0.width();

        for row in 0..rows {
            for col in 0..cols {
                let pos = Position::new(row, col);
                let &Camp { height, .. } = self.0.get(pos).unwrap();

                let mut score = 1;
                for rel in POS {
                    let mut ipos = self.0.relative_pos(pos, rel);
                    let mut seen = 0;

                    while let Some(pos) = ipos {
                        if let Some(t) = self.0.get(pos) {
                            seen += 1;
                            if t.height >= height {
                                break;
                            }
                            ipos = self.0.relative_pos(pos, rel);
                        }
                    }

                    score *= seen;
                    if score == 0 {
                        break;
                    }
                }

                self.0.get_mut(pos).unwrap().score = score;
            }
        }

        self.0.iter().map(|(_, t)| t.score).max().unwrap()
    }
}
