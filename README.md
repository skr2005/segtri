# segtri

This crate provides a simple implementation of a segment tree with lazy propagation.
It supports efficient updates and queries over ranges of data.

# Features
- **Customizable Data Type and Query Method**:
    Any type `T` can be used as the data in the segment tree, as long as:
    - It implements [Clone].
    - Its reference implements [`Add<Output = T>`](Add) and [`Mul<usize, Output = T>`](Mul).

    To customize queries, you can redefine how the data are *summed* and *multiplied*.

    The multiplication trait is used to efficiently compute the "sum" of repeated data
    and is assumed to be faster than "adding" multiple components individually.

- **Customizable Update Operations**:
    Any type `Op` can be used as an update operation, provided it implements [`ModifyOp<T>`].

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

struct Add(usize);
impl ModifyOp<usize> for Add {
    fn nop() -> Self {
        Add(0)
    }

    fn combine(&mut self, another_op: &Self) {
        self.0 += another_op.0
    }

    fn apply(&self, orig_seg_data: &mut usize, seg_len: usize) {
        *orig_seg_data += seg_len * self.0
    }
}

// Segment tree of length 10 with initial point value 1
let mut seg = SegTree::new(10, 1);
// Query the sum of segment 2..4
assert_eq!(seg.query(&(2..4)), 2);
// Add 2 to the segment 0..10
seg.modify(&(0..10), &Add(2));
// Query the value at point 1
assert_eq!(seg.query_point(1), 3);
// Add 2 to point 0
seg.modify_point(0, &Add(2));
assert_eq!(seg.query(&(0..2)), 5 + 3);
```

For more examples please see `examples` directory.