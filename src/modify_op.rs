pub trait ModifyOp<T>: PartialEq + Clone {
    fn modify_range_ntimes(
        &self,
        orig_seg_data: &mut T,
        seg_len: usize,
        n: usize,
    );
}
