//! Iterators over [`WeightedPicker`]s.

use itertools::Itertools;

use crate::WeightedPicker;

/// Borrowing iterator over the elements of a [`WeightedPicker`].
///
/// See [`WeightedPicker::iter`].
pub struct Iter<'a, T> {
    inner: &'a WeightedPicker<T>,
    idx_list: Vec<usize>,
    front_idx: usize,
    back_idx: usize,
    exhausted_back: bool,
}

impl<'a, T> Iter<'a, T> {
    pub fn new(inner: &'a WeightedPicker<T>) -> Self {
        let mut idx_list = (0..inner.len()).collect_vec();
        idx_list.sort_unstable_by(|ai, bi| {
            let a = inner.items[*ai].1;
            let b = inner.items[*bi].1;
            a.total_cmp(&b)
        });
        Self {
            inner,
            idx_list,
            front_idx: 0,
            back_idx: inner.len() - 1,
            exhausted_back: false,
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        // this also serves to be our normal forward case ender, cause it's set to len-1
        if self.front_idx > self.back_idx {
            None
        } else {
            let idx = self.front_idx;
            self.front_idx += 1;
            let real_idx = self.idx_list[idx];
            Some(self.inner.get_by_idx(real_idx).unwrap())
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len(), Some(self.len()))
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.exhausted_back || self.back_idx < self.front_idx {
            None
        } else {
            let idx = self.back_idx;
            if self.back_idx == 0 {
                self.exhausted_back = true;
            } else {
                self.back_idx -= 1;
            }
            let real_idx = self.idx_list[idx];
            Some(self.inner.get_by_idx(real_idx).unwrap())
        }
    }
}

impl<'a, T> ExactSizeIterator for Iter<'a, T> {
    fn len(&self) -> usize {
        1 + self.back_idx - self.front_idx
    }
}
