use std::collections::VecDeque;

pub struct OpDeque<Op>(pub VecDeque<(Op, usize)>);

impl<Op> OpDeque<Op>
where
    Op: PartialEq,
{
    pub fn new() -> Self {
        // no allocation
        OpDeque(VecDeque::from(vec![]))
    }

    pub fn push_back(&mut self, new_op: Op, times: usize) {
        if times == 0 {
            return;
        }
        let deque = &mut self.0;
        if let Some(back) = deque.back_mut() {
            if back.0 == new_op {
                back.1 += times;
                return;
            }
        }
        deque.push_back((new_op, times));
    }
}

