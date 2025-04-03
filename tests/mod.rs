use std::usize;

use Operations::*;
use segtree_rs::{ModifyOp, SegTree};
use serial_test::parallel;

#[derive(Clone, PartialEq)]
enum Operations {
    SetTo(usize),
    Add1,
    Mul(usize),
}

impl ModifyOp<usize> for Operations {
    fn modify_range_ntimes(
        &self,
        orig_data: &mut usize,
        seg_size: usize,
        n: usize,
    ) {
        if n == 0 {
            return;
        }
        match self {
            SetTo(x) => *orig_data = x * seg_size,
            Add1 => *orig_data += n * seg_size,
            Mul(x) => *orig_data *= x.pow(n.try_into().unwrap()),
        }
    }
}

#[test]
#[parallel]
fn test_simple_add() {
    let mut seg = SegTree::new(10, 1);
    seg.modify(&(0..0), &Add1, 2);
    assert_eq!(seg.query(&(0..10)), 10);
    seg.modify(&(0..10), &Add1, 2);
    assert_eq!(seg.query(&(0..10)), 30);
    seg.modify(&(0..5), &Add1, 2);
    assert_eq!(seg.query(&(0..10)), 40);
    seg.modify_point(2, &Add1, 4);
    assert_eq!(seg.query(&(0..10)), 44);
    assert_eq!(seg.query(&(2..6)), 22);
    assert_eq!(seg.query(&(0..10)), 44);
    assert_eq!(seg.query_point(9), 3);
    assert_eq!(seg.query_point(0), 5);
    assert_eq!(seg.query_point(2), 9);
}

#[test]
#[parallel]
fn test_with_points() {
    let mut seg = SegTree::with_points(&(1..=10).collect::<Vec<_>>());
    seg.modify(&(0..0), &Add1, 2);
    assert_eq!(seg.query(&(0..10)), 10 + 45);
    seg.modify(&(0..10), &Add1, 2);
    assert_eq!(seg.query(&(0..10)), 30 + 45);
    seg.modify(&(0..5), &Add1, 2);
    assert_eq!(seg.query(&(0..10)), 40 + 45);
    seg.modify_point(2, &Add1, 4);
    assert_eq!(seg.query(&(0..10)), 44 + 45);
    assert_eq!(seg.query(&(2..6)), 22 + 14);
    assert_eq!(seg.query(&(0..10)), 44 + 45);
    assert_eq!(seg.query_point(9), 3 + 9);
    assert_eq!(seg.query_point(0), 5 + 0);
    assert_eq!(seg.query_point(2), 9 + 2);
}

#[test]
#[parallel]
fn test_op_order() {
    let mut seg = SegTree::new(10, 1);
    seg.modify(&(0..10), &SetTo(0), 0);
    assert_eq!(seg.query(&(0..10)), 10);
    seg.modify(&(0..10), &SetTo(0), 1);
    assert_eq!(seg.query(&(0..7)), 0);
    seg.modify(&(1..10), &Mul(10), 3);
    seg.modify(&(7..9), &Add1, 2);
    seg.modify(&(1..8), &Add1, 1);
    seg.modify(&(5..7), &Mul(2), 2);
    seg.modify(&(5..7), &Add1, 0);
    seg.modify(&(2..10), &Mul(2), 2);
    seg.modify(&(4..7), &Add1, 1);
    assert_eq!(seg.query(&(0..6)), 31);
    assert_eq!(seg.query(&(0..10)), 68);
    assert_eq!(
        vec![0, 1, 4, 4, 5, 17, 17, 12, 8, 0],
        (0..10).map(|i| seg.query_point(i)).collect::<Vec<_>>(),
    )
}

#[test]
#[parallel]
fn test_large_seg() {
    let mut seg = SegTree::new(usize::MAX, 0);
    seg.modify(&(0..usize::MAX / 3), &SetTo(1), 1);
    seg.modify(&(usize::MAX / 5..usize::MAX / 3 + 121), &Add1, 3);
    assert_eq!(seg.query_point(0), 1);
    assert_eq!(seg.query_point(usize::MAX / 3), 3);
    assert_eq!(seg.query_point(usize::MAX / 3 - 1), 4);
    assert_eq!(seg.query_point(usize::MAX / 3 + 121), 0);
}
