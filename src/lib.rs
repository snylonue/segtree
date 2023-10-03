use std::ops::{Range, RangeBounds, Bound};

pub struct SegTree {
    store: Vec<i64>,
    len: usize
}

impl SegTree {
    pub fn new(v: Vec<i64>) -> Self {
        let len = if v.is_empty() {
            return Self { store: Vec::new(), len: 0 };
        } else {
            2_f64.powf((v.len() as f64).log2() + 1.0).ceil() as usize - 1
        };
        let mut store = vec![0; len];
        Self::build(&v, &mut store, 1, v.len(), 1);

        Self { store, len: v.len() }
    }

    fn build(v: &[i64], store: &mut [i64], s: usize, t: usize, p: usize) {
        if s == t {
            store[p - 1] = v[s - 1];
        } else {
            let m = s + (t - s) / 2;
            Self::build(v, store, s, m, p * 2);
            Self::build(v, store, m + 1, t, p * 2 + 1);
            store[p - 1] = store[p * 2 - 1] + store[p * 2];
        }
    }

    /// Returns the sum of range (zero-indexed)
    pub fn query(&self, range: impl RangeBounds<usize>) -> Option<i64> {
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
            Some(self.get_range_sum(0, self.len - 1, range, 1))
        }
    }

    fn get_range_sum(&self, start: usize, end: usize, range: Range<usize>, p: usize) -> i64 {
        if range.contains(&start) && range.contains(&end) {
            self.store[p - 1]
        } else {
            let m = start + (end - start) / 2;
            let mut sum = 0;
            if range.start <= m {
                sum += self.get_range_sum(start, m, range.clone(), p * 2);
            }
            if range.end > m + 1 {
                sum += self.get_range_sum(m + 1, end, range, p * 2 + 1);
            }

            sum
        }
    }
}

#[cfg(test)]
mod test {
    use crate::SegTree;

    #[test]
    fn build() {
        let tree = SegTree::new(vec![10, 11, 12, 13, 14]);
        assert_eq!(tree.store, vec![60, 33, 27, 21, 12, 13, 14, 10, 11]);
        let tree = SegTree::new(vec![4, 3, 2, 1, 2, 3, 4]);
        assert_eq!(tree.store, vec![19, 10, 9, 7, 3, 5, 4, 4, 3, 2, 1, 2, 3]);
        let _ = SegTree::new(vec![]);
    }

    #[test]
    fn range_sum() {
        let tree = SegTree::new(vec![10, 11, 12, 13, 14]);
        assert_eq!(tree.query(3..5), Some(27));
        assert_eq!(tree.query(1..5), Some(50));
        assert_eq!(tree.query(..5), Some(60));
        assert_eq!(tree.query(..), Some(60));
        assert_eq!(tree.query(1..=3), Some(36));
        let tree = SegTree::new(vec![]);
        assert_eq!(tree.query(..), None);
    }
}