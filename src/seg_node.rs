use std::ops::{Add, Mul, Range};

use crate::{modify_op::ModifyOp, op_deque::OpDeque};

#[cfg(test)]
use std::sync::atomic::{AtomicUsize, Ordering};
#[cfg(test)]
static CHILD_CREATED_CNT: AtomicUsize = AtomicUsize::new(0);

enum NodeData<T> {
    Same(T),
    Acc(T),
}

impl<T> NodeData<T>
where
    for<'a> &'a T: Mul<usize, Output = T>,
    T: Clone,
{
    fn calc_acc(&self, len: usize) -> T {
        match self {
            Same(s) => s * len,
            Acc(s) => s.clone(),
        }
    }

    fn modify_whole_with<'a, I, O>(
        &mut self,
        my_range_len: usize,
        op_times: I,
    ) where
        I: IntoIterator<Item = &'a (O, usize)>,
        O: ModifyOp<T> + 'a,
    {
        match self {
            Same(s) => {
                for (op, times) in op_times {
                    op.modify_range_ntimes(s, 1, *times)
                }
            }
            Acc(a) => {
                for (op, times) in op_times {
                    op.modify_range_ntimes(a, my_range_len, *times)
                }
            }
        }
    }
}

use NodeData::*;

pub struct SegNode<T, Op> {
    data: NodeData<T>,
    pending_ops: OpDeque<Op>,
    l_child: Option<Box<Self>>,
    r_child: Option<Box<Self>>,
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

impl<T, Op> SegNode<T, Op>
where
    T: Clone,
    for<'x> &'x T: Mul<usize, Output = T> + Add<Output = T>,
    Op: ModifyOp<T>,
{
    pub fn from_same_point_data(same_point_data: T) -> Self {
        Self {
            data: Same(same_point_data),
            pending_ops: OpDeque::new(),
            l_child: None,
            r_child: None,
        }
    }

    fn ensure_child<'a>(
        child: &'a mut Option<Box<Self>>,
        default_data: &T,
    ) -> &'a mut Self {
        child.get_or_insert_with(|| {
            #[cfg(test)]
            {
                CHILD_CREATED_CNT.fetch_add(1, Ordering::SeqCst);
            }
            Box::new(Self::from_same_point_data(default_data.clone()))
        })
    }

    fn resolve_pending_ops(
        &mut self,
        node_range: &Range<usize>,
        default_data: &T,
    ) {
        #[cfg(debug_assertions)]
        {
            assert!(!node_range.is_empty())
        }

        let OpDeque(ref mut ops) = self.pending_ops;
        if ops.is_empty() {
            return;
        }

        self.data.modify_whole_with(node_range.len(), ops.iter());

        if node_range.len() > 1 {
            let l_child =
                Self::ensure_child(&mut self.l_child, default_data);
            let r_child =
                Self::ensure_child(&mut self.r_child, default_data);
            for (op, times) in ops.iter() {
                l_child.pending_ops.push_back(op.clone(), *times);
                r_child.pending_ops.push_back(op.clone(), *times);
            }
        }

        ops.clear();
        ops.shrink_to_fit();
    }

    pub fn query(
        &mut self,
        node_range: &Range<usize>,
        target_range: &Range<usize>,
        default_data: &T,
        force_refetch: bool,
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
        self.resolve_pending_ops(node_range, default_data);
        if !force_refetch {
            if target_range == node_range {
                return self.data.calc_acc(node_range.len());
            }
            if let Same(s) = &self.data {
                return s * target_range.len();
            }
        };

        let (l_child_range, r_child_range) = split_lr_range(node_range);
        let l_target_range = range_intersect(&l_child_range, target_range);
        let r_target_range = range_intersect(&r_child_range, target_range);

        let query_child = |child: &mut Option<Box<Self>>,
                           child_range: &Range<usize>,
                           target_range: &Range<usize>,
                           default_data: &T|
         -> T {
            #[cfg(debug_assertions)]
            {
                assert!(
                    !target_range.is_empty(),
                    "target: {:?} child: {:?}",
                    target_range,
                    child_range
                );
                assert!(
                    target_range.start >= child_range.start
                        && target_range.end <= child_range.end,
                    "target: {:?} child: {:?}",
                    target_range,
                    child_range
                );
            }
            child.as_mut().map_or_else(
                || default_data * target_range.len(),
                |ch| {
                    ch.query(
                        child_range,
                        target_range,
                        default_data,
                        false,
                    )
                },
            )
        };
        let mut query_l_child = || {
            query_child(
                &mut self.l_child,
                &l_child_range,
                &l_target_range,
                default_data,
            )
        };
        let mut query_r_child = || {
            query_child(
                &mut self.r_child,
                &r_child_range,
                &r_target_range,
                default_data,
            )
        };

        if l_target_range.is_empty() {
            query_r_child()
        } else if r_target_range.is_empty() {
            query_l_child()
        } else {
            &query_l_child() + &query_r_child()
        }
    }

    pub fn modify(
        &mut self,
        node_range: &Range<usize>,
        target_range: &Range<usize>,
        op: &Op,
        times: usize,
        default_data: &T,
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
            self.pending_ops.push_back(op.clone(), times);
            return;
        }

        let (l_child_range, r_child_range) = split_lr_range(node_range);
        let l_target_range = range_intersect(&l_child_range, target_range);
        let r_target_range = range_intersect(&r_child_range, target_range);

        self.resolve_pending_ops(node_range, default_data);
        if !r_target_range.is_empty() {
            Self::ensure_child(&mut self.r_child, default_data).modify(
                &r_child_range,
                &r_target_range,
                op,
                times,
                default_data,
            );
        }
        if !l_target_range.is_empty() {
            Self::ensure_child(&mut self.l_child, default_data).modify(
                &l_child_range,
                &l_target_range,
                op,
                times,
                default_data,
            );
        }
        self.data = Acc(self
            .query(node_range, node_range, default_data, true)
            .clone());
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
        assert_eq!(CHILD_CREATED_CNT.load(Ordering::Acquire), 8);
        assert_eq!(seg.query_point(len / 4-1), 1);
        assert_eq!(seg.query_point(len / 4), 2);
        assert_eq!(seg.query_point(len / 4 * 3 - 1), 2);
        assert_eq!(seg.query_point(len / 4 * 3), 1);
        assert_eq!(CHILD_CREATED_CNT.load(Ordering::Acquire), 8);
    }
}
