pub struct LazyOps<Op>(Vec<(Op, usize)>);

impl<Op> LazyOps<Op>
where
    Op: PartialEq,
{
    pub fn new() -> Self {
        // no allocation
        LazyOps(vec![])
    }

    pub fn push_back(&mut self, new_op: Op, times: usize) {
        if times == 0 {
            return;
        }
        let queue = &mut self.0;
        if let Some(back) = queue.last_mut() {
            if back.0 == new_op {
                back.1 += times;
                return;
            }
        }
        queue.push((new_op, times));
    }

    // pub fn extend(&mut self, iter: impl IntoIterator<Item = (Op, usize)>) {
    //     for (op, times) in iter {
    //         self.push_back(op, times);
    //     }
    // }

    pub fn clear(&mut self) {
        self.0 = vec![];
    }

    pub fn inner(&self) -> &Vec<(Op, usize)> {
        &self.0
    }
}
