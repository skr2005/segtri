/// A trait representing a range modification operation for segment data.
/// Any type properly implemented it can be used as operations for methods
/// such as [crate::SegTree::modify].
pub trait ModifyOp<T> {
    /// This method should return an operation that does nothing.
    fn nop() -> Self;

    /// The modification method.
    ///
    /// The given `orig_seg_data` is the reference to the sum of data
    /// in a segment of length `seg_len`.
    ///
    /// This method should modify `orig_seg_data`,
    /// behaving like applying the operation
    /// to data of every point of the segment.
    /// It should do nothing when the operation is [`ModifyOp::nop()`].
    fn apply(&self, orig_seg_data: &mut T, seg_len: usize);

    /// Combine two operations.
    ///
    /// This method should perform modifications to the operation
    /// through `&mut self` to make it
    /// become the 'combination' of `self` and `another_op`.
    /// Specifically, after this method is called,
    /// calling `self.apply(orig_seg_data, seg_len);`
    /// should have the same effect as calling
    /// `self.apply(orig_seg_data, seg_len); another_op.apply(orig_seg_data, seg_len);` before.
    fn combine(&mut self, another_op: &Self);
}
