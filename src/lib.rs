mod modify_op;
mod op_deque;
mod seg_node;

use seg_node::SegNode;
use std::ops::{Add, Mul, Range};

pub use modify_op::ModifyOp;

pub struct SegTree<T, Op> {
    point_cnt: usize,
    default_data: T,
    root: Option<Box<SegNode<T, Op>>>,
}

impl<T, Op> SegTree<T, Op>
where
    T: Sized + Clone,
    for<'x> &'x T: Add<Output = T> + Mul<usize, Output = T>,
    Op: ModifyOp<T>,
{
    /// Create [SegTree] by giving count of points and default data all the same in each single point.
    ///
    /// This method does not allocate. Nodes in tree will be lazily created later.
    pub fn new(point_cnt: usize, default_data: T) -> Self {
        Self {
            point_cnt,
            default_data,
            root: None,
        }
    }

    // /// Create [SegTree] by giving initial data in each point.
    // /// # Performance
    // /// This operation is O(n), and all nodes in [SegTree] will be created.
    // fn with_points(points: impl IntoIterator<Item = T>) -> Self {
    //     todo!()
    // }

    /// Return the length of the whole segment.
    pub fn point_cnt(&self) -> usize {
        self.point_cnt
    }

    /// Return default data for points in segment.
    pub fn default_data(&self) -> &T {
        &self.default_data
    }

    /// # Panics
    /// Panics when `point >= self.point_cnt()`
    pub fn modify_point(&mut self, point: usize, op: &Op, times: usize) {
        self.modify(&(point..point + 1), op, times);
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
        let root = self.root.get_or_insert_with(|| {
            Box::new(SegNode::from_same_point_data(
                self.default_data.clone(),
            ))
        });
        root.modify(
            &(0..self.point_cnt),
            target_range,
            op,
            ntimes,
            &self.default_data,
        );
    }

    /// # Panics
    /// Panics when `point >= self.point_cnt()`
    pub fn query_point(&mut self, point: usize) -> T {
        self.query(&(point..point + 1))
    }

    /// # Panics
    /// Panics when `target_range.is_empty() || target_range.end > self.point_cnt()`
    pub fn query(&mut self, target_range: &Range<usize>) -> T {
        assert!(target_range.end <= self.point_cnt);
        assert!(!target_range.is_empty());
        let root = self.root.get_or_insert_with(|| {
            Box::new(SegNode::from_same_point_data(
                self.default_data.clone(),
            ))
        });
        root.query(
            &(0..self.point_cnt),
            target_range,
            &self.default_data,
            false,
        )
    }
}
