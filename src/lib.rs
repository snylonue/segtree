pub mod operation;

use std::{
    marker::PhantomData,
    ops::{Bound, Range, RangeBounds, RangeInclusive},
};

use operation::Operation;

pub struct SegTree<T, O> {
    store: Vec<T>,
    len: usize,
    _op: PhantomData<O>,
}

impl<T: Clone, O: Operation<T>> SegTree<T, O> {
    pub fn new(v: Vec<T>) -> Self {
        let len = if v.is_empty() {
            return Self {
                store: Vec::new(),
                len: 0,
                _op: PhantomData,
            };
        } else {
            2_f64.powf((v.len() as f64).log2() + 1.0).ceil() as usize - 1
        };
        let mut store = vec![O::zero(); len];
        Self::build(&v, &mut store, 1, v.len(), 1);

        Self {
            store,
            len: v.len(),
            _op: PhantomData,
        }
    }

    fn build(v: &[T], store: &mut [T], s: usize, t: usize, p: usize) {
        if s == t {
            store[p - 1] = v[s - 1].clone();
        } else {
            let m = s + (t - s) / 2;
            Self::build(v, store, s, m, p * 2);
            Self::build(v, store, m + 1, t, p * 2 + 1);
            store[p - 1] = O::combine(&store[p * 2 - 1], &store[p * 2]);
        }
    }

    /// Returns the sum of range (zero-indexed)
    pub fn query(&self, range: impl RangeBounds<usize>) -> Option<T> {
        let range = {
            let start = match range.start_bound() {
                Bound::Included(s) => *s,
                Bound::Excluded(s) => s + 1,
                Bound::Unbounded => 0,
            };
            let end = match range.end_bound() {
                Bound::Included(e) => e + 1,
                Bound::Excluded(e) => *e,
                Bound::Unbounded => self.len,
            };
            start..end
        };

        if range.is_empty() || range.end > self.len {
            None
        } else {
            Some(self.get_range_sum(0..=self.len - 1, range, 1))
        }
    }

    fn get_range_sum(&self, cur: RangeInclusive<usize>, range: Range<usize>, p: usize) -> T {
        if range.contains(cur.start()) && range.contains(cur.end()) {
            self.store[p - 1].clone()
        } else {
            let m = cur.start() + (cur.end() - cur.start()) / 2;
            let mut sum = O::zero();
            if range.start <= m {
                sum = O::combine(
                    &sum,
                    &self.get_range_sum(*cur.start()..=m, range.clone(), p * 2),
                );
            }
            if range.end > m + 1 {
                sum = O::combine(
                    &sum,
                    &self.get_range_sum(m + 1..=*cur.end(), range.clone(), p * 2 + 1),
                );
            }

            sum
        }
    }

    pub fn update(&mut self, range: impl RangeBounds<usize>, val: T) {
        let range = {
            let start = match range.start_bound() {
                Bound::Included(s) => *s,
                Bound::Excluded(s) => s + 1,
                Bound::Unbounded => 0,
            };
            let end = match range.end_bound() {
                Bound::Included(e) => e + 1,
                Bound::Excluded(e) => *e,
                Bound::Unbounded => self.len,
            };
            start..end
        };

        if !(range.is_empty() || range.end > self.len) {
            self.update_impl(0..=self.len - 1, range, 1, &val)
        }
    }

    fn update_impl(&mut self, cur: RangeInclusive<usize>, range: Range<usize>, p: usize, val: &T) {
        if cur.start() == cur.end() {
            self.store[p - 1] = O::combine(&self.store[p - 1], val);
            return;
        }
        let m = cur.start() + (cur.end() - cur.start()) / 2;
        if m >= range.start {
            self.update_impl(*cur.start()..=m, range.clone(), p * 2, val);
        }
        if m + 1 < range.end {
            self.update_impl(m + 1..=*cur.end(), range, p * 2 + 1, val);
        }
        self.store[p - 1] = O::combine(&self.store[p * 2 - 1], &self.store[p * 2]);
    }
}

#[cfg(test)]
mod test {
    use crate::{operation::Add, SegTree};

    #[test]
    fn build() {
        let tree = SegTree::<_, Add>::new(vec![10, 11, 12, 13, 14]);
        assert_eq!(tree.store, vec![60, 33, 27, 21, 12, 13, 14, 10, 11]);
        let tree = SegTree::<_, Add>::new(vec![4, 3, 2, 1, 2, 3, 4]);
        assert_eq!(tree.store, vec![19, 10, 9, 7, 3, 5, 4, 4, 3, 2, 1, 2, 3]);
        let _ = SegTree::<i32, Add>::new(vec![]);
    }

    #[test]
    fn range_sum() {
        let tree = SegTree::<_, Add>::new(vec![10, 11, 12, 13, 14]);
        assert_eq!(tree.query(3..5), Some(27));
        assert_eq!(tree.query(1..5), Some(50));
        assert_eq!(tree.query(..5), Some(60));
        assert_eq!(tree.query(..), Some(60));
        assert_eq!(tree.query(1..=3), Some(36));
        let tree = SegTree::<i32, Add>::new(vec![]);
        assert_eq!(tree.query(..), None);
    }

    #[test]
    fn update() {
        let mut tree = SegTree::<_, Add>::new(vec![10, 11, 12, 13, 14]);
        tree.update(.., 3);
        assert_eq!(tree.query(..), Some(13 + 14 + 15 + 16 + 17));
        tree.update(..3, 1);
        assert_eq!(tree.query(..), Some(14 + 15 + 16 + 16 + 17));
    }
}
