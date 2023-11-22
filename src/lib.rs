pub mod operation;

use std::ops::{Bound, Range, RangeBounds, RangeInclusive};

use operation::Monoid;

pub struct SegTree<T, M> {
    store: Vec<T>,
    len: usize,
    monoid: M,
}

impl<T: Clone, M: Monoid<T>> SegTree<T, M> {
    pub fn new(v: Vec<T>, monoid: M) -> Self {
        let len = if v.is_empty() {
            return Self {
                store: Vec::new(),
                len: 0,
                monoid,
            };
        } else {
            2_f64.powf((v.len() as f64).log2() + 1.0).ceil() as usize - 1
        };
        let mut store = vec![monoid.unit(); len];
        Self::build(&v, &mut store, &monoid, 1, v.len(), 1);

        Self {
            store,
            len: v.len(),
            monoid,
        }
    }

    fn build(v: &[T], store: &mut [T], monoid: &M, s: usize, t: usize, p: usize) {
        if s == t {
            store[p - 1] = v[s - 1].clone();
        } else {
            let m = s + (t - s) / 2;
            Self::build(v, store, monoid, s, m, p * 2);
            Self::build(v, store, monoid, m + 1, t, p * 2 + 1);
            store[p - 1] = monoid.combine(&store[p * 2 - 1], &store[p * 2]);
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
            let mut sum = self.monoid.unit();
            if range.start <= m {
                sum = self.monoid.combine(
                    &sum,
                    &self.get_range_sum(*cur.start()..=m, range.clone(), p * 2),
                );
            }
            if range.end > m + 1 {
                sum = self.monoid.combine(
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
            self.store[p - 1] = self.monoid.combine(&self.store[p - 1], val);
            return;
        }
        let m = cur.start() + (cur.end() - cur.start()) / 2;
        if m >= range.start {
            self.update_impl(*cur.start()..=m, range.clone(), p * 2, val);
        }
        if m + 1 < range.end {
            self.update_impl(m + 1..=*cur.end(), range, p * 2 + 1, val);
        }
        self.store[p - 1] = self
            .monoid
            .combine(&self.store[p * 2 - 1], &self.store[p * 2]);
    }
}

#[cfg(test)]
mod test {

    use crate::{
        operation::{MonoidType, Sum},
        SegTree,
    };

    #[test]
    fn build() {
        let tree = SegTree::new(vec![10, 11, 12, 13, 14], Sum);
        assert_eq!(tree.store, vec![60, 33, 27, 21, 12, 13, 14, 10, 11]);
        let tree = SegTree::new(vec![4, 3, 2, 1, 2, 3, 4], Sum);
        assert_eq!(tree.store, vec![19, 10, 9, 7, 3, 5, 4, 4, 3, 2, 1, 2, 3]);
        let _ = SegTree::<i32, _>::new(vec![], Sum);
    }

    #[test]
    fn range_sum() {
        let tree = SegTree::new(vec![10, 11, 12, 13, 14], Sum);
        assert_eq!(tree.query(3..5), Some(27));
        assert_eq!(tree.query(1..5), Some(50));
        assert_eq!(tree.query(..5), Some(60));
        assert_eq!(tree.query(..), Some(60));
        assert_eq!(tree.query(1..=3), Some(36));
        let tree = SegTree::<i32, _>::new(vec![], Sum);
        assert_eq!(tree.query(..), None);
    }

    #[test]
    fn update() {
        let mut tree = SegTree::new(vec![10, 11, 12, 13, 14], Sum);
        tree.update(.., 3);
        assert_eq!(tree.query(..), Some(13 + 14 + 15 + 16 + 17));
        tree.update(..3, 1);
        assert_eq!(tree.query(..), Some(14 + 15 + 16 + 16 + 17));
    }

    #[test]
    fn monoid_type() {
        let add = MonoidType {
            zero: 0u32,
            op: |a: &'_ u32, b: &'_ u32| a + b,
        };

        let tree = SegTree::new(vec![1, 2, 1876, 911, 11], add);
        assert_eq!(tree.query(..3), Some(1 + 2 + 1876));
        assert_eq!(
            tree.query(..),
            Some([1, 2, 1876, 911, 11].into_iter().sum())
        )
    }
}
