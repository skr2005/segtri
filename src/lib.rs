/*!
    This crate provides a simple implementation of a segment tree with lazy propagation.
    It supports efficient updates and queries over ranges of data.

    # Features
    - **Customizable Data Type**:
        Any type `T` can be used as the data in the segment tree, as long as:
        - It implements [Sized] and [Clone].
        - Its reference implements [`Add<Output = T>`] and [`Mul<usize, Output = T>`].

        The multiplication trait is used to efficiently compute the sum of repeated data
        and is assumed to be faster than adding multiple components individually.

    - **Customizable Update Operations**:
        Any type `Op` can be used as an update operation, provided it implements [ModifyOp].

    - **Lazy Node Creation**:
        Nodes in the segment tree are created lazily. This ensures the tree remains small
        when built with [SegTree::new] and when updates are applied to large ranges.

    # Performance

    The segment tree achieves O(log(n)) for updates and queries, provided:
    - Customized update operations are O(1).
    - Data type addition [Add::add] is O(1).
    - Data type multiplication [Mul::mul] is O(log(k)), where `k` is the multiplier.

    # Example
    ```
    use segtree_rs::{SegTree, ModifyOp};

    #[derive(Clone, PartialEq)]
    enum Operations {
        Add1,
        Mul(usize),
    }

    use Operations::*;

    impl ModifyOp<usize> for Operations {
        fn modify_range_ntimes(
            &self,
            orig_data: &mut usize,
            seg_size: usize,
            n: usize,
        ) {
            match self {
                Add1 => *orig_data += n * seg_size,
                Mul(x) => *orig_data *= x.pow(n.try_into().unwrap()),
            }
        }
    }

    // Segment tree of length 10 with initial point value 1
    let mut seg = SegTree::new(10, 1);
    // query the sum of segment 2..4
    assert_eq!(seg.query(&(2..4)), 2);
    // multiply segment 0..10 by 3 one time.
    seg.modify(&(0..10), &Mul(3), 1);
    // query the value of point 1
    assert_eq!(seg.query_point(1), 3);
    // add 1 to point 0 two times
    seg.modify_point(0, &Add1, 2);
    assert_eq!(seg.query(&(0..2)), 5 + 3);
    ```
*/

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
    /// Creates a new [SegTree] with `point_cnt` points,
    /// all initialized to `default_data_for_single_point`.
    /// This is O(1) and doesn't allocate on the heap, with nodes lazily created.
    /// # Panics
    /// Panics if `point_cnt == 0`.
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

    /// Creates a fully built [SegTree] from the provided slice of point data, which is O(n).
    /// Use [Self::new] if all points are identical.
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

    /// Modifies the data of a single point at `point_idx`.
    /// # Panics
    /// Panics if `point_idx >= self.point_cnt()`.
    pub fn modify_point(
        &mut self,
        point_idx: usize,
        op: &Op,
        times: usize,
    ) {
        self.modify(&(point_idx..point_idx + 1), op, times);
    }

    /// Modifies the data for all points in `target_range`, repeated `ntimes`.
    /// Does nothing if the range is empty or `ntimes` is zero.
    /// # Panics
    /// May panic if the range end exceeds `self.point_cnt()`.
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

    /// Retrieves the data at a single point `point_idx`.
    /// # Panics
    /// Panics if `point_idx >= self.point_cnt()`.
    pub fn query_point(&mut self, point_idx: usize) -> T {
        self.query(&(point_idx..point_idx + 1))
    }

    /// Retrieves the sum of data for all points in `target_range`.
    /// # Panics
    /// Panics if the range is empty or out of bounds.
    pub fn query(&mut self, target_range: &Range<usize>) -> T {
        assert!(target_range.end <= self.point_cnt);
        assert!(!target_range.is_empty());
        self.root.query(&(0..self.point_cnt), target_range)
    }
}
