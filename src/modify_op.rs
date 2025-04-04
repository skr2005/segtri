/// A trait representing a range modification operation for segment data.
/// Any type properly implemented it can be used as operations for methods
/// such as [crate::SegTree::modify].
pub trait ModifyOp<T>: PartialEq + Clone {
    /// The modification function.
    /// 
    /// The given `orig_seg_data` is the reference to the sum of data 
    /// in a segment of length `seg_len`.
    /// 
    /// This function should modify `orig_seg_data`, 
    /// behaving like applying the operation multiple times (`n` times)
    /// to data of every point of the segment. 
    /// It should does nothing when patameter `n` is zero.
    fn modify_range_ntimes(
        &self,
        orig_seg_data: &mut T,
        seg_len: usize,
        n: usize,
    );
}
