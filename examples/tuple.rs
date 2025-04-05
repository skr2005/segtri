//! This example demonstrates how to use tuple as data type.

use std::ops::{Add, Mul};

use segtri::{ModifyOp, SegTree};

/// New type is needed as we cannot impl on (i32, i32)
#[derive(Clone, PartialEq, Debug)]
struct Tu(i32, i32);

impl Add for &Tu {
    type Output = Tu;
    fn add(self, rhs: Self) -> Self::Output {
        Tu(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Mul<usize> for &Tu {
    type Output = Tu;
    fn mul(self, rhs: usize) -> Self::Output {
        let rhs: i32 = rhs.try_into().unwrap();
        Tu(self.0 * rhs, self.1 * rhs)
    }
}

/// Our operations to modify [Tu]
#[derive(Clone, PartialEq)]
enum Op {
    SetTo(Tu),
    Swap,
}

use Op::*;

impl ModifyOp<Tu> for Op {
    fn modify_range_ntimes(
        &self,
        orig_seg_data: &mut Tu,
        seg_len: usize,
        n: usize,
    ) {
        match self {
            Swap => {
                if n % 2 == 1 {
                    std::mem::swap(
                        &mut orig_seg_data.0,
                        &mut orig_seg_data.1,
                    );
                }
            }
            SetTo(tu) => {
                if n > 0 {
                    *orig_seg_data = tu * seg_len;
                }
            }
        }
    }
}

fn main() {
    let mut seg = SegTree::new(7, Tu(0, 0));
    seg.modify(&(0..4), &SetTo(Tu(1, -1)), 1);
    seg.modify(&(3..6), &Swap, 1);
    seg.modify(&(2..5), &Swap, 1);
    seg.modify(&(1..4), &Swap, 1);
    assert_eq!(
        (0..7).map(|i| seg.query_point(i)).collect::<Vec<_>>(),
        vec![
            Tu(1, -1),
            Tu(-1, 1),
            Tu(1, -1),
            Tu(-1, 1),
            Tu(0, 0),
            Tu(0, 0),
            Tu(0, 0)
        ]
    );
    assert_eq!(seg.query(&(0..7)), Tu(0, 0));
}
