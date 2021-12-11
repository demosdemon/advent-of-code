use std::collections::{HashSet, LinkedList};

const AROUND_THE_BLOCK: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

#[derive(Debug, Default)]
pub struct Ocean {
    width: usize,
    matrix: Vec<u8>,
}

impl Ocean {
    fn depth(&self) -> usize {
        self.matrix.len() / self.width
    }

    fn idx_to_pos(&self, idx: usize) -> (usize, usize) {
        (idx % self.width, idx / self.width)
    }

    fn pos_to_idx(&self, (x, y): (usize, usize)) -> usize {
        (y * self.width) + x
    }

    pub fn basins(&self) -> Vec<(usize, usize)> {
        let mut basins = Vec::new();
        for (root, &root_value) in self.iter_low_points() {
            let mut basin = HashSet::new();
            let mut queue = LinkedList::new();
            queue.push_back((root, root_value));
            while let Some((point, point_value)) = queue.pop_front() {
                if basin.insert(point) {
                    for (idx, &adj_value) in self.iter_surrounding(point) {
                        if adj_value < 9 && point_value < adj_value {
                            queue.push_back((idx, adj_value));
                        }
                    }
                }
            }
            basins.push((root, basin.len()));
        }

        basins
    }

    pub fn iter_low_points(&self) -> impl Iterator<Item = (usize, &u8)> {
        self.matrix
            .iter()
            .enumerate()
            .filter(|&(idx, value)| self.is_low_point(idx, *value))
    }

    fn iter_surrounding(&self, idx: usize) -> impl Iterator<Item = (usize, &u8)> {
        let (x, y) = self.idx_to_pos(idx);
        AROUND_THE_BLOCK
            .iter()
            .map(move |&(dx, dy)| (x as isize + dx, y as isize + dy))
            .filter(|&(x, y)| {
                x >= 0 && x < self.width as isize && y >= 0 && y < self.depth() as isize
            })
            .map(|(x, y)| self.pos_to_idx((x as usize, y as usize)))
            .map(|idx| (idx, &self.matrix[idx]))
    }

    fn is_low_point(&self, idx: usize, value: u8) -> bool {
        if value == 9 {
            return false;
        }
        if value == 0 {
            return true;
        }
        self.iter_surrounding(idx).all(|(_, &v)| value < v)
    }
}

impl<S: AsRef<str>> Extend<S> for Ocean {
    fn extend<T: IntoIterator<Item = S>>(&mut self, iter: T) {
        for line in iter {
            let line = line.as_ref();
            if self.width == 0 {
                self.width = line.len();
            } else {
                assert_eq!(self.width, line.len());
            }
            let iter = line.chars().map(|c| (c as u8) - 48).map(|v| {
                assert!(v < 10);
                v
            });
            self.matrix.extend(iter)
        }
    }
}

impl<S: AsRef<str>> FromIterator<S> for Ocean {
    fn from_iter<T: IntoIterator<Item = S>>(iter: T) -> Self {
        let mut ocean = Ocean::default();
        ocean.extend(iter);
        ocean
    }
}
