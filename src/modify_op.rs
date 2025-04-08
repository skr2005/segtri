/// A trait representing a range modification operation for segment tree data.
/// Any type that implements this trait can be used as an operation
/// in methods such as [crate::SegTree::modify].
pub trait ModifyOp<T> {
    /// Returns a no-op (do-nothing) operation.
    fn nop() -> Self;

    /// Applies the modification operation.
    ///
    /// The given `orig_seg_data` is a reference to the sum of data
    /// in a segment of length `seg_len`.
    ///
    /// This method should modify `orig_seg_data` as if the operation
    /// were applied to each point in the segment individually.
    /// It should do nothing if the operation is [`ModifyOp::nop()`].
    fn apply(&self, orig_seg_data: &mut T, seg_len: usize);

    /// Combines this operation with another operation.
    ///
    /// This method should update `self` so that it becomes the combination
    /// of `self` and `another_op`.
    ///
    /// Specifically, after calling this method,
    /// the result of `self.apply(orig_seg_data, seg_len);`
    /// should be equivalent to:
    /// `self.apply(orig_seg_data, seg_len); another_op.apply(orig_seg_data, seg_len);`
    /// as if the two operations were applied in sequence.
    fn combine(&mut self, another_op: &Self);
}
