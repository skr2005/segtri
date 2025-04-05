//! This example demonstrates how to use an integer type other than usize

use std::ops::{Add, Mul};

use segtri::{ModifyOp, SegTree};

/// Create a new type.
/// This indirection is zero-cost.
#[derive(Clone)]
struct I32(i32);

impl Add for &I32 {
    type Output = I32;
    fn add(self, rhs: Self) -> Self::Output {
        I32(self.0 + rhs.0)
    }
}

impl Mul<usize> for &I32 {
    type Output = I32;
    fn mul(self, rhs: usize) -> Self::Output {
        I32(self.0 * rhs as i32)
    }
}

/// Our update operation.
#[derive(Clone, PartialEq)]
struct Mul2;

impl ModifyOp<I32> for Mul2 {
    fn modify_range_ntimes(
        &self,
        orig_seg_data: &mut I32,
        _seg_len: usize,
        n: usize,
    ) {
        orig_seg_data.0 *= 2i32.pow(n.try_into().unwrap())
    }
}

fn main() {
    let mut seg = SegTree::new(7, I32(-1));
    seg.modify(&(1..4), &Mul2, 2);
    assert_eq!(seg.query(&(0..2)).0, -3);
}
