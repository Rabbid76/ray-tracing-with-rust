use crate::iterator::ViewportIterator;
use std::sync::{Arc, Mutex};

pub struct IteratorExp2 {
    cx: usize,
    cy: usize,
    tile_size: usize,
    lix: Vec<usize>,
    liy: Vec<usize>,
    size: usize,
    tx: isize,
    ty: isize,
}

impl IteratorExp2 {
    pub fn new(cx: usize, cy: usize) -> IteratorExp2 {
        IteratorExp2 {
            cx,
            cy,
            tile_size: IteratorExp2::tile_size(cx, cy),
            lix: Vec::new(),
            liy: Vec::new(),
            size: 0,
            tx: -1,
            ty: -1,
        }
    }

    fn tile_size(cx: usize, cy: usize) -> usize {
        let mut max_s = usize::max(cx, cy);
        let mut p2 = 0;
        while max_s > 0 {
            p2 += 1;
            max_s >>= 1;
        }
        1 << p2 - 1
    }

    fn no_of_tiles(size: usize, tile_size: usize) -> usize {
        (size - 1) / tile_size + 1
    }

    fn distribute_indices(no: usize) -> Vec<usize> {
        let mut li: Vec<usize> = Vec::with_capacity(no);
        li.resize(no, 0);
        let m = (no - 1) / 2;
        for i in 0..no {
            li[i] = if i % 2 == 0 { m - i / 2 } else { m + 1 + i / 2 }
        }
        li
    }

    fn distribute_indices_reversed(no: usize) -> Vec<usize> {
        let mut li: Vec<usize> = Vec::with_capacity(no);
        li.resize(no, 0);
        let m = no / 2;
        for i in 0..no {
            li[i] = if i % 2 == 0 { m + i / 2 } else { m - 1 - i / 2 }
        }
        li
    }
}

impl Iterator for IteratorExp2 {
    type Item = (usize, usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.tx < 0 && self.ty < 0 {
            self.tx = 0;
            self.ty = 0;
            self.size = self.tile_size;
            return Some((0, 0, self.size));
        } else {
            while self.size > 0 {
                if self.tx <= 0 && self.ty <= 0 {
                    let no_x = IteratorExp2::no_of_tiles(self.cx, self.size);
                    let no_y = IteratorExp2::no_of_tiles(self.cy, self.size);
                    self.lix = IteratorExp2::distribute_indices(no_x);
                    self.liy = IteratorExp2::distribute_indices_reversed(no_y);
                    self.tx = 0;
                    self.ty = 0;
                }

                let x = self.lix[self.tx as usize];
                let y = self.liy[self.ty as usize];
                let size = self.size;

                self.tx += 1;
                if self.tx >= self.lix.len() as isize {
                    self.tx = 0;
                    self.ty += 1;
                }
                if self.ty >= self.liy.len() as isize {
                    self.ty = 0;
                    self.size >>= 1;
                }

                if size > 128 || x % 2 != 0 || y % 2 != 0 {
                    return Some((x * size, y * size, size));
                }
            }
        }
        None
    }
}

impl ViewportIterator for IteratorExp2 {
    fn create_new(&self) -> Arc<Mutex<dyn ViewportIterator>> {
        Arc::new(Mutex::new(IteratorExp2 {
            cx: self.cx,
            cy: self.cy,
            tile_size: self.tile_size,
            lix: Vec::new(),
            liy: Vec::new(),
            size: 0,
            tx: -1,
            ty: -1,
        }))
    }
}

#[cfg(test)]
mod iterator_exp2_test {
    use super::*;

    #[test]
    fn next_test() {
        let mut i = IteratorExp2::new(2, 2);
        assert_eq!((0, 0, 2), i.next().unwrap());
        assert_eq!((0, 1, 1), i.next().unwrap());
        assert_eq!((1, 1, 1), i.next().unwrap());
        assert_eq!((1, 0, 1), i.next().unwrap());
        assert_eq!(None, i.next());
    }
}
