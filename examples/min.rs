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
struct Sub(usize);

impl ModifyOp<Data> for Sub {
    fn nop() -> Self {
        Sub(0)
    }

    fn apply(&self, orig_seg_data: &mut Data, seg_len: usize) {
        orig_seg_data.0 -= (seg_len * self.0) as i32
    }

    fn combine(&mut self, another_op: &Self) {
        self.0 += another_op.0
    }
}

fn main() {
    let mut seg = SegTree::new(5, Data(0));
    seg.modify(&(0..5), &Sub(2));
    assert_eq!(seg.query(&(0..4)).0, -2);
    seg.modify(&(0..2), &Sub(1));
    seg.modify(&(1..3), &Sub(1));
    seg.modify(&(2..4), &Sub(1));
    assert_eq!(seg.query(&(0..5)).0, -4);
}
