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
        c: [Chars;H],
    }

    let mut ans = 0;

    for i in 0..H {
        for j in 0..W {
            if c[i][j] == '#' {
                continue;
            }
            let mut todo = VecDeque::<(usize, usize, i32)>::new();
            let t = i * W + j;
            todo.push_back((i, j, 1 << t));
            loop {
                if let Some((x0, y0, n)) = todo.pop_front() {
                    if [(1, 0), (!0, 0), (0, 1), (0, !0)]
                        .iter()
                        .map(|(dx, dy)| (x0 + dx, y0 + dy))
                        .find(|&p| p == (i, j))
                        .is_some()
                    {
                        let l = n.count_ones();
                        if l >= 4 {
                            //println!("{} {} {} {}", i, j, x0, y0);
                            ans.chmax(l);
                        }
                    }

                    for &(dx, dy) in [(1, 0), (!0, 0), (0, 1), (0, !0)].iter() {
                        let x1 = x0 + dx;
                        let y1 = y0 + dy;
                        if x1 >= H || y1 >= W {
                            continue;
                        }
                        let u = x1 * W + y1;
                        if u < t {
                            continue;
                        }
                        if c[x1][y1] == '.' && (n >> u) & 1 == 0 {
                            todo.push_back((x1, y1, n ^ (1 << u)));
                        }
                    }
                } else {
                    break;
                }
            }
        }
    }

    if ans == 0 {
        println!("-1");
    } else {
        println!("{}", ans);
    }
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
