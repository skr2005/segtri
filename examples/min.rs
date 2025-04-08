//! Customize [Add] and [Mul] trait for your data type so queries are customized.
//! This example demonstrates how to implement min queries.

use std::ops::{Add, Mul};

use segtri::{ModifyOp, SegTree};

#[derive(Clone)]
struct Data(i32);

impl Add for &Data {
    type Output = Data;
    fn add(self, rhs: Self) -> Self::Output {
        Data(self.0.min(rhs.0))
    }
}

impl Mul<usize> for &Data {
    type Output = Data;
    fn mul(self, _rhs: usize) -> Self::Output {
        self.clone()
    }
}

#[derive(Clone, PartialEq)]
struct Sub1;

impl ModifyOp<Data> for Sub1 {
    fn modify_range_ntimes(
        &self,
        orig_seg_data: &mut Data,
        seg_len: usize,
        n: isize,
    ) {
        orig_seg_data.0 -= n as i32 * seg_len as i32;
    }
}

fn main() {
    let mut seg = SegTree::new(5, Data(0));
    seg.modify(&(0..5), &Sub1, 2);
    assert_eq!(seg.query(&(0..4)).0, -2);
    seg.modify(&(0..2), &Sub1, 1);
    seg.modify(&(1..3), &Sub1, 1);
    seg.modify(&(2..4), &Sub1, 1);
    assert_eq!(seg.query(&(0..5)).0, -4);
}
