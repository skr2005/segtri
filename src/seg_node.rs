use std::ops::{Add, Mul, Range};

use crate::{lazy_ops::LazyOps, modify_op::ModifyOp};

#[cfg(test)]
use std::sync::atomic::{AtomicUsize, Ordering};
#[cfg(test)]
static CHILD_CREATED_CNT: AtomicUsize = AtomicUsize::new(0);

pub struct DivergedSegNode<T, Op> {
    data_acc: T,
    pending_ops_for_children: LazyOps<Op>,
    l_child: Box<SegNode<T, Op>>,
    r_child: Box<SegNode<T, Op>>,
}

pub enum SegNode<T, Op> {
    Same(T),
    Diverged(DivergedSegNode<T, Op>),
}

use SegNode::*;

impl<T, Op> SegNode<T, Op> {
    fn modify_whole_with<'a>(
        &mut self,
        node_range_len: usize,
        op: &Op,
        times: usize,
    ) where
        Op: ModifyOp<T> + 'a,
    {
        match self {
            Same(s) => op.modify_range_ntimes(s, 1, times),
            Diverged(d) => {
                op.modify_range_ntimes(
                    &mut d.data_acc,
                    node_range_len,
                    times,
                );
                d.pending_ops_for_children.push_back(op.clone(), times);
            }
        }
    }
}

fn range_mid(range: &Range<usize>) -> usize {
    (range.start + range.end) / 2
}

/// right child seg may be larger
fn split_lr_range(range: &Range<usize>) -> (Range<usize>, Range<usize>) {
    let mid = range_mid(range);
    (range.start..mid, mid..range.end)
}

fn range_intersect(
    range1: &Range<usize>,
    range2: &Range<usize>,
) -> Range<usize> {
    range1.start.max(range2.start)..range1.end.min(range2.end)
}

impl<T, Op> DivergedSegNode<T, Op>
where
    Op: PartialEq + ModifyOp<T>,
{
    fn resolve_pending_ops(
        &mut self,
        l_child_range_len: usize,
        r_child_range_len: usize,
    ) {
        if self.pending_ops_for_children.inner().is_empty() {
            return;
        }

        for (op, times) in self.pending_ops_for_children.inner() {
            self.l_child
                .modify_whole_with(l_child_range_len, op, *times);
            self.r_child
                .modify_whole_with(r_child_range_len, op, *times);
        }
        self.pending_ops_for_children.clear();
    }
}

impl<T, Op> SegNode<T, Op>
where
    T: Clone,
    for<'x> &'x T: Mul<usize, Output = T> + Add<Output = T>,
    Op: ModifyOp<T>,
{
    pub fn from_same_point_data(same_point_data: T) -> Self {
        Self::Same(same_point_data)
    }

    pub fn with_points(point_data: &[T]) -> Self {
        #[cfg(debug_assertions)]
        {
            assert!(!point_data.is_empty())
        }
        if point_data.len() == 1 {
            return Self::Same(point_data[0].clone());
        }
        let (l_range, r_range) = split_lr_range(&(0..point_data.len()));
        let mut l_child =
            Box::new(Self::with_points(&point_data[l_range.clone()]));
        let mut r_child =
            Box::new(Self::with_points(&point_data[r_range.clone()]));
        let data_acc = &l_child.query(&l_range, &l_range)
            + &r_child.query(&r_range, &r_range);
        Self::Diverged(DivergedSegNode {
            data_acc,
            pending_ops_for_children: LazyOps::new(),
            l_child,
            r_child,
        })
    }

    pub fn query(
        &mut self,
        node_range: &Range<usize>,
        target_range: &Range<usize>,
    ) -> T {
        #[cfg(debug_assertions)]
        {
            assert!(!target_range.is_empty());
            assert!(
                target_range.start >= node_range.start
                    && target_range.end <= node_range.end,
                "target: {:?} node: {:?}",
                target_range,
                node_range
            );
        }

        let diverged = match self {
            Same(s) => return &*s * target_range.len(),
            Diverged(d) => {
                if target_range == node_range {
                    return d.data_acc.clone();
                }
                d
            }
        };

        let (l_child_range, r_child_range) = split_lr_range(node_range);
        let l_target_range = range_intersect(&l_child_range, target_range);
        let r_target_range = range_intersect(&r_child_range, target_range);

        diverged
            .resolve_pending_ops(l_child_range.len(), r_child_range.len());

        let mut query_l_child =
            || diverged.l_child.query(&l_child_range, &l_target_range);
        let mut query_r_child =
            || diverged.r_child.query(&r_child_range, &r_target_range);

        if l_target_range.is_empty() {
            query_r_child()
        } else if r_target_range.is_empty() {
            query_l_child()
        } else {
            &query_l_child() + &query_r_child()
        }
    }

    fn ensure_diverged(
        &mut self,
        my_node_range_len: usize,
    ) -> &mut DivergedSegNode<T, Op> {
        match self {
            Same(s) => {
                #[cfg(test)]
                CHILD_CREATED_CNT.fetch_add(2, Ordering::SeqCst);

                *self = Diverged(DivergedSegNode {
                    data_acc: &*s * my_node_range_len,
                    pending_ops_for_children: LazyOps::new(),
                    l_child: Box::new(SegNode::Same(s.clone())),
                    r_child: Box::new(SegNode::Same(s.clone())),
                });
                let Diverged(d) = self else {
                    panic!("Impossible")
                };
                d
            }
            Diverged(d) => d,
        }
    }

    pub fn modify(
        &mut self,
        node_range: &Range<usize>,
        target_range: &Range<usize>,
        op: &Op,
        times: usize,
    ) {
        #[cfg(debug_assertions)]
        {
            assert!(!target_range.is_empty());
            assert!(!range_intersect(node_range, target_range).is_empty());
            assert!(
                target_range.start >= node_range.start
                    && target_range.end <= node_range.end,
                "target: {:?} node: {:?}",
                target_range,
                node_range
            );
        }

        if node_range == target_range {
            self.modify_whole_with(node_range.len(), op, times);
            return;
        }

        let (l_child_range, r_child_range) = split_lr_range(node_range);
        let l_target_range = range_intersect(&l_child_range, target_range);
        let r_target_range = range_intersect(&r_child_range, target_range);

        let diverged = self.ensure_diverged(node_range.len());
        diverged
            .resolve_pending_ops(l_child_range.len(), r_child_range.len());

        if !r_target_range.is_empty() {
            diverged.r_child.modify(
                &r_child_range,
                &r_target_range,
                op,
                times,
            );
        }
        if !l_target_range.is_empty() {
            diverged.l_child.modify(
                &l_child_range,
                &l_target_range,
                op,
                times,
            );
        }
        diverged.data_acc =
            &diverged.l_child.query(&l_child_range, &l_child_range)
                + &diverged.r_child.query(&r_child_range, &r_child_range);
    }
}

#[cfg(test)]
mod test {
    use std::usize;

    use serial_test::{parallel, serial};

    use crate::SegTree;

    use super::*;

    #[test]
    #[parallel]
    fn test_split_lr_range() {
        assert_eq!(split_lr_range(&(4..6)), (4..5, 5..6))
    }

    #[test]
    #[serial]
    fn test_laziness() {
        let len = usize::MAX / 4 + 1;
        #[derive(Clone, PartialEq)]
        struct Add1;
        impl ModifyOp<usize> for Add1 {
            fn modify_range_ntimes(
                &self,
                orig_seg_data: &mut usize,
                seg_len: usize,
                n: usize,
            ) {
                *orig_seg_data += seg_len * n;
            }
        }
        let mut seg = SegTree::new(len, 1);
        seg.modify(&(len / 4 - 1..0), &Add1, 1);
        assert_eq!(seg.query_point(len / 5), 1);
        assert_eq!(CHILD_CREATED_CNT.load(Ordering::Acquire), 0);
        seg.modify(&(len / 4..len / 4 * 3), &Add1, 1);
        assert_eq!(CHILD_CREATED_CNT.load(Ordering::Acquire), 6);
        assert_eq!(seg.query_point(len / 4 - 1), 1);
        assert_eq!(seg.query_point(len / 4), 2);
        assert_eq!(seg.query_point(len / 4 * 3 - 1), 2);
        assert_eq!(seg.query_point(len / 4 * 3), 1);
        assert_eq!(CHILD_CREATED_CNT.load(Ordering::Acquire), 6);
        seg.modify(&(len / 16 * 7..len / 2), &Add1, 1);
        assert_eq!(CHILD_CREATED_CNT.load(Ordering::Acquire), 10);
        assert_eq!(seg.query(&(len / 2 - 89..len / 2)), 267);
        assert_eq!(CHILD_CREATED_CNT.load(Ordering::Acquire), 10);
    }
}
