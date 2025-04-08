pub struct LazyOps<Op>(Vec<(Op, isize)>);

impl<Op> LazyOps<Op>
where
    Op: PartialEq,
{
    pub fn new() -> Self {
        // no allocation
        LazyOps(vec![])
    }

    pub fn push_back(&mut self, new_op: Op, times: isize) {
        if times == 0 {
            return;
        }
        let queue = &mut self.0;
        if let Some(back) = queue.last_mut() {
            if back.0 == new_op {
                back.1 += times;
                if back.1 == 0 {
                    queue.pop();
                }
                return;
            }
        }
        queue.push((new_op, times));
    }

    pub fn clear(&mut self) {
        self.0 = vec![];
    }

    pub fn inner(&self) -> &Vec<(Op, isize)> {
        &self.0
    }
}
