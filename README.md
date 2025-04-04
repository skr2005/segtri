# segtri

This crate provides a simple implementation of a segment tree with lazy propagation.
It supports efficient updates and queries over ranges of data.

# Features
- **Customizable Data Type**:
    Any type `T` can be used as the data in the segment tree, as long as:
    - It implements [Sized] and [Clone].
    - Its reference implements [`Add<Output = T>`] and [`Mul<usize, Output = T>`].

    The multiplication trait is used to efficiently compute the sum of repeated data
    and is assumed to be faster than adding multiple components individually.

- **Customizable Update Operations**:
    Any type `Op` can be used as an update operation, provided it implements [ModifyOp].

- **Lazy Node Creation**:
    Nodes in the segment tree are created lazily. This ensures the tree remains small
    when built with [SegTree::new] and when updates are applied to large ranges.

# Performance

The segment tree achieves O(log(n)) for updates and queries, provided:
- Customized update operations are O(1).
- Data type addition [Add::add] is O(1).
- Data type multiplication [Mul::mul] is O(log(k)), where `k` is the multiplier.

# Example
```rust
use segtri::{SegTree, ModifyOp};

#[derive(Clone, PartialEq)]
enum Operations {
    Add1,
    Mul(usize),
}

use Operations::*;

impl ModifyOp<usize> for Operations {
    fn modify_range_ntimes(
        &self,
        orig_data: &mut usize,
        seg_size: usize,
        n: usize,
    ) {
        match self {
            Add1 => *orig_data += n * seg_size,
            Mul(x) => *orig_data *= x.pow(n.try_into().unwrap()),
        }
    }
}

// Segment tree of length 10 with initial point value 1
let mut seg = SegTree::new(10, 1);
// query the sum of segment 2..4
assert_eq!(seg.query(&(2..4)), 2);
// multiply segment 0..10 by 3 one time.
seg.modify(&(0..10), &Mul(3), 1);
// query the value of point 1
assert_eq!(seg.query_point(1), 3);
// add 1 to point 0 two times
seg.modify_point(0, &Add1, 2);
assert_eq!(seg.query(&(0..2)), 5 + 3);
```