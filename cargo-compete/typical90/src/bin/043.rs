#![allow(unused_imports)]
#![allow(non_snake_case)]
use itertools::*;
use maplit::*;
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use proconio::{fastout, input};
use std::collections::*;
use std::fmt::Write;
use superslice::{Ext, Ext2};

#[fastout]
fn main() {
    input! {
        H: usize,
        W: usize,
        a: Usize1,
        b: Usize1,
        c: Usize1,
        d: Usize1,
        S: [Chars;H],
    }

    let mut min = vec![vec![vec![1_000_000; 4]; W]; H];
    let dirs = vec![(1, 0), (!0, 0), (0, 1), (0, !0)];

    min[a][b][0] = 0;
    min[a][b][1] = 0;
    min[a][b][2] = 0;
    min[a][b][3] = 0;

    let mut todo = VecDeque::<(usize, usize)>::new();
    todo.push_back((a, b));
    loop {
        if let Some((x, y)) = todo.pop_front() {
            for (i, &(dx, dy)) in dirs.iter().enumerate() {
                let x1 = x + dx;
                let y1 = y + dy;
                if x1 < H && y1 < W && S[x1][y1] == '.' {
                    let m = min[x][y]
                        .iter()
                        .enumerate()
                        .map(|(j, m)| m + if i == j { 0 } else { 1 })
                        .min()
                        .unwrap();
                    if min[x1][y1][i].chmin(m){
                        todo.push_back((x1, y1));
                    }
                }
            }
        } else {
            break;
        }
    }
    let ans = min[c][d].iter().min().unwrap();
    println!("{}", ans);
}
pub trait Change<T: PartialOrd> {
    fn chmin(&mut self, rhs: Self) -> bool;
    fn chmax(&mut self, rhs: Self) -> bool;
}
impl<T: PartialOrd> Change<T> for T {
    fn chmax(&mut self, rhs: Self) -> bool {
        if *self < rhs {
            *self = rhs;
            true
        } else {
            false
        }
    }
    fn chmin(&mut self, rhs: Self) -> bool {
        if *self > rhs {
            *self = rhs;
            true
        } else {
            false
        }
    }
}