#![doc = include_str!("./../README.md")]

mod modify_op;
mod seg_node;

use seg_node::SegNode;
use std::ops::{Add, Mul, Range};

pub use modify_op::ModifyOp;

/// Segment tree that supports efficient range queries and range updates.
pub struct SegTree<T, Op> {
    point_cnt: usize,
    root: SegNode<T, Op>,
}

impl<T, Op> SegTree<T, Op>
where
    T: Clone,
    for<'x> &'x T: Add<Output = T> + Mul<usize, Output = T>,
    Op: ModifyOp<T>,
{
    /// Creates a new [SegTree] with `point_cnt` points,
    /// all initialized to `default_data_for_single_point`.
    ///
    /// This is an O(1) operation that does not allocate on the heap.
    /// Tree nodes are created lazily.
    ///
    /// # Panics
    /// Panics if `point_cnt == 0`.
    pub fn new(
        point_cnt: usize,
        default_data_for_single_point: T,
    ) -> Self {
        assert!(point_cnt > 0);
        Self {
            point_cnt,
            root: SegNode::from_same_point_data(
                default_data_for_single_point,
            ),
        }
    }

    /// Creates a fully built [SegTree] from the provided slice of point data.
    ///
    /// This is an O(n) operation.
    /// Use [Self::new] if all points are initialized to the same value.
    ///
    /// # Panics
    /// Panics if `point_data.is_empty()`.
    pub fn with_points(point_data: &[T]) -> Self {
        assert!(!point_data.is_empty());
        Self {
            point_cnt: point_data.len(),
            root: SegNode::with_points(point_data),
        }
    }

    /// Returns the total number of points in the whole segment.
    pub fn point_cnt(&self) -> usize {
        self.point_cnt
    }

    /// Applies an update operation to the data at a single point `point_idx`.
    ///
    /// # Panics
    /// Panics if `point_idx >= self.point_cnt()`.
    pub fn modify_point(&mut self, point_idx: usize, op: &Op) {
        self.modify(&(point_idx..point_idx + 1), op);
    }

    /// Applies an update operation to all points in the given `target_range`.
    ///
    /// This method does nothing if the range is empty.
    ///
    /// # Panics
    /// Panics if the range is not empty and its end exceeds `self.point_cnt()`.
    pub fn modify(&mut self, target_range: &Range<usize>, op: &Op) {
        if target_range.is_empty() {
            return;
        }
        assert!(target_range.end <= self.point_cnt);
        self.root.modify(&(0..self.point_cnt), target_range, op);
    }

    /// Retrieves the data at the point `point_idx`.
    ///
    /// # Panics
    /// Panics if `point_idx >= self.point_cnt()`.
    pub fn query_point(&mut self, point_idx: usize) -> T {
        self.query(&(point_idx..point_idx + 1))
    }

    /// Retrieves the sum of data for all points in `target_range`.
    ///
    /// # Panics
    /// Panics if the range is empty or out of bounds.
    pub fn query(&mut self, target_range: &Range<usize>) -> T {
        assert!(target_range.end <= self.point_cnt);
        assert!(!target_range.is_empty());
        self.root.query(&(0..self.point_cnt), target_range)
    }
}
