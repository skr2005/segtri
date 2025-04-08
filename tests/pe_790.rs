use std::{
    mem::swap,
    ops::{Add, Mul, Range},
};

use segtri::{ModifyOp, SegTree};

const S0: usize = 290797;
const N: usize = 50515093;

fn next_s(prev_s: usize) -> usize {
    prev_s * prev_s % N
}

struct ModifRecord {
    add: bool,
    x: usize,
    y_range: Range<usize>,
}

fn modif_records(t: usize) -> Vec<ModifRecord> {
    let mut res = Vec::with_capacity(2 * t);
    let mut s = S0;
    for _ in 0..t {
        let mut s1 = next_s(s);
        let mut s2 = next_s(s1);
        let mut s3 = next_s(s2);
        let s_ = next_s(s3);
        if s > s1 {
            swap(&mut s, &mut s1);
        }
        if s2 > s3 {
            swap(&mut s2, &mut s3);
        }
        let y_range = s2..s3 + 1;
        res.push(ModifRecord {
            add: true,
            x: s,
            y_range: y_range.clone(),
        });
        res.push(ModifRecord {
            add: false,
            x: s1 + 1,
            y_range,
        });
        s = s_
    }
    res.sort_unstable_by(|r1, r2| r1.x.cmp(&r2.x));
    res
}

#[derive(Clone)]
struct ClockCnt([u32; 12]);

impl Add for &ClockCnt {
    type Output = ClockCnt;
    fn add(self, rhs: Self) -> Self::Output {
        let mut res = ClockCnt([0; 12]);
        for i in 0..res.0.len() {
            res.0[i] = self.0[i] + rhs.0[i];
        }
        res
    }
}

impl Mul<usize> for &ClockCnt {
    type Output = ClockCnt;
    fn mul(self, rhs: usize) -> Self::Output {
        let mut res = ClockCnt([0; 12]);
        for i in 0..res.0.len() {
            res.0[i] = self.0[i] * rhs as u32;
        }
        res
    }
}

struct Rotate(isize);

impl ModifyOp<ClockCnt> for Rotate {
    fn apply(&self, orig_seg_data: &mut ClockCnt, _seg_len: usize) {
        if self.0 > 0 {
            orig_seg_data.0.rotate_right(self.0 as usize % 12);
        } else {
            orig_seg_data.0.rotate_left((12 - self.0) as usize % 12);
        }
    }

    fn nop() -> Self {
        Rotate(0)
    }

    fn combine(&mut self, another_op: &Self) {
        self.0 += another_op.0
    }
}

fn sum_hours(cnt: &ClockCnt) -> usize {
    cnt.0[0] as usize * 12
        + &cnt.0[1..12]
            .iter()
            .zip(1..)
            .map(|(e, i)| *e as usize * i)
            .sum()
}

fn c(t: usize) -> usize {
    let mut res = 0;
    let mut seg = SegTree::new(
        N as usize,
        ClockCnt([1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    );
    let mut prev_x = 0;
    for r in modif_records(t) {
        let diff = (r.x - prev_x) * sum_hours(&seg.query(&(0..N)));
        res += diff;
        seg.modify(&r.y_range, &Rotate(if r.add { 1 } else { -1 }));
        prev_x = r.x;
    }
    let diff_last = (N - prev_x) * sum_hours(&seg.query(&(0..N)));
    res += diff_last;
    res
}

#[test]
fn test_c_0_1_10() {
    assert_eq!(c(0), 30621295449583788);
    assert_eq!(c(1), 30613048345941659);
    assert_eq!(c(10), 21808930308198471);
    assert_eq!(c(100), 16190667393984172);
}

#[test]
fn test_c_100000() {
    assert_eq!(c(100000), 16585056588495119);
}
