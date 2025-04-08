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

struct Multiply(i32);

impl ModifyOp<I32> for Multiply {
    fn nop() -> Self {
        Multiply(1)
    }

    fn apply(&self, orig_seg_data: &mut I32, _seg_len: usize) {
        orig_seg_data.0 *= self.0
    }

    fn combine(&mut self, another_op: &Self) {
        self.0 *= another_op.0
    }
}

fn main() {
    let mut seg = SegTree::new(7, I32(-1));
    seg.modify(&(1..4), &Multiply(4));
    assert_eq!(seg.query(&(0..2)).0, -5);
}
