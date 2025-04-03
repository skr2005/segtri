mod lazy_ops;
mod modify_op;
mod seg_node;

use seg_node::SegNode;
use std::ops::{Add, Mul, Range};

pub use modify_op::ModifyOp;

pub struct SegTree<T, Op> {
    point_cnt: usize,
    root: SegNode<T, Op>,
}

impl<T, Op> SegTree<T, Op>
where
    T: Sized + Clone,
    for<'x> &'x T: Add<Output = T> + Mul<usize, Output = T>,
    Op: ModifyOp<T>,
{
    /// Create [SegTree] by giving count of points and default data all the same in each single point.
    ///
    /// This operation is O(1), and does not allocate in heap.
    /// Nodes in tree will be lazily created later.
    /// # Panics
    /// Panics when `point_cnt == 0`.
    pub fn new(
        point_cnt: usize,
        default_data_for_single_point: T,
    ) -> Self {
        Self {
            point_cnt,
            root: SegNode::from_same_point_data(
                default_data_for_single_point,
            ),
        }
    }

    /// Create [SegTree] by giving initial data in each point.
    /// # Performance
    /// This operation is O(n), and all nodes in [SegTree] will be created.
    /// If all points are the same initially, please consider using [Self::new] instead.
    /// # Panics
    /// Panics if `point_data.is_empty()`
    pub fn with_points(point_data: &[T]) -> Self {
        assert!(!point_data.is_empty());
        Self {
            point_cnt: point_data.len(),
            root: SegNode::with_points(point_data),
        }
    }

    /// Return the length of the whole segment.
    pub fn point_cnt(&self) -> usize {
        self.point_cnt
    }

    /// # Panics
    /// Panics when `point_idx >= self.point_cnt()`
    pub fn modify_point(
        &mut self,
        point_idx: usize,
        op: &Op,
        times: usize,
    ) {
        self.modify(&(point_idx..point_idx + 1), op, times);
    }

    /// Does nothing when `target_range.is_empty() || ntimes == 0`
    /// # Panics
    /// May panic when `!target_range.is_empty() && target_range.end > self.point_cnt()`
    pub fn modify(
        &mut self,
        target_range: &Range<usize>,
        op: &Op,
        ntimes: usize,
    ) {
        if target_range.is_empty() || ntimes == 0 {
            return;
        }
        assert!(target_range.end <= self.point_cnt);
        self.root
            .modify(&(0..self.point_cnt), target_range, op, ntimes);
    }

    /// # Panics
    /// Panics when `point_idx >= self.point_cnt()`
    pub fn query_point(&mut self, point_idx: usize) -> T {
        self.query(&(point_idx..point_idx + 1))
    }

    /// # Panics
    /// Panics when `target_range.is_empty() || target_range.end > self.point_cnt()`
    pub fn query(&mut self, target_range: &Range<usize>) -> T {
        assert!(target_range.end <= self.point_cnt);
        assert!(!target_range.is_empty());
        self.root.query(&(0..self.point_cnt), target_range)
    }
}
