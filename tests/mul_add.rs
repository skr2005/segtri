use std::usize;

use segtri::{ModifyOp, SegTree};

struct MulAdd(usize, usize);

impl MulAdd {
    fn add(b: usize) -> Self {
        Self(1, b)
    }

    fn mul(k: usize) -> Self {
        Self(k, 0)
    }
}

impl ModifyOp<usize> for MulAdd {
    fn nop() -> Self {
        MulAdd(1, 0)
    }

    fn combine(&mut self, another_op: &Self) {
        *self = Self(
            another_op.0 * self.0,
            another_op.0 * self.1 + another_op.1,
        )
    }

    fn apply(&self, orig_seg_data: &mut usize, seg_len: usize) {
        *orig_seg_data *= self.0;
        *orig_seg_data += seg_len * self.1
    }
}

#[test]
fn test_simple_addition() {
    let mut seg = SegTree::new(10, 1);
    seg.modify(&(0..0), &MulAdd::add(2));
    assert_eq!(seg.query(&(0..10)), 10);
    seg.modify(&(0..10), &MulAdd::add(2));
    assert_eq!(seg.query(&(0..10)), 30);
    seg.modify(&(0..5), &MulAdd::add(2));
    assert_eq!(seg.query(&(0..10)), 40);
    seg.modify_point(2, &MulAdd::add(4));
    assert_eq!(seg.query(&(0..10)), 44);
    assert_eq!(seg.query(&(2..6)), 22);
    assert_eq!(seg.query(&(0..10)), 44);
    assert_eq!(seg.query_point(9), 3);
    assert_eq!(seg.query_point(0), 5);
    assert_eq!(seg.query_point(2), 9);
}

#[test]
fn test_with_points() {
    let mut seg = SegTree::with_points(&(1..=10).collect::<Vec<_>>());
    seg.modify(&(0..0), &MulAdd::add(2));
    assert_eq!(seg.query(&(0..10)), 10 + 45);
    seg.modify(&(0..10), &MulAdd::add(2));
    assert_eq!(seg.query(&(0..10)), 30 + 45);
    seg.modify(&(0..5), &MulAdd::add(2));
    assert_eq!(seg.query(&(0..10)), 40 + 45);
    seg.modify_point(2, &MulAdd::add(4));
    assert_eq!(seg.query(&(0..10)), 44 + 45);
    assert_eq!(seg.query(&(2..6)), 22 + 14);
    assert_eq!(seg.query(&(0..10)), 44 + 45);
    assert_eq!(seg.query_point(9), 3 + 9);
    assert_eq!(seg.query_point(0), 5 + 0);
    assert_eq!(seg.query_point(2), 9 + 2);
}

#[test]
fn test_op_order() {
    let mut seg = SegTree::new(10, 0);
    assert_eq!(seg.query(&(0..7)), 0);
    seg.modify(&(1..10), &MulAdd::mul(1000));
    seg.modify(&(7..9), &MulAdd::add(2));
    seg.modify(&(1..8), &MulAdd::add(1));
    seg.modify(&(5..7), &MulAdd::mul(4));
    seg.modify(&(5..7), &MulAdd::add(0));
    seg.modify(&(2..10), &MulAdd::mul(4));
    seg.modify(&(4..7), &MulAdd::add(1));
    assert_eq!(seg.query(&(0..6)), 31);
    assert_eq!(seg.query(&(0..10)), 68);
    assert_eq!(
        vec![0, 1, 4, 4, 5, 17, 17, 12, 8, 0],
        (0..10).map(|i| seg.query_point(i)).collect::<Vec<_>>(),
    )
}

#[test]
fn test_large_seg() {
    let mut seg = SegTree::new(usize::MAX, 0);
    seg.modify(&(0..usize::MAX / 3), &MulAdd::add(1));
    seg.modify(&(usize::MAX / 5..usize::MAX / 3 + 121), &MulAdd::add(3));
    assert_eq!(seg.query_point(0), 1);
    assert_eq!(seg.query_point(usize::MAX / 3), 3);
    assert_eq!(seg.query_point(usize::MAX / 3 - 1), 4);
    assert_eq!(seg.query_point(usize::MAX / 3 + 121), 0);
}
