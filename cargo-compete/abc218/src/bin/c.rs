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
        N: usize,
        S: [Chars;N],
        mut T: [Chars;N],
    }
    let mut s = (N - 1, 0, N - 1, 0);
    for i in 0..N {
        for j in 0..N {
            if S[i][j] == '#' {
                s.0.chmin(i);
                s.1.chmax(i);
                s.2.chmin(j);
                s.3.chmax(j);
            }
        }
    }

    for r in 0..4 {
        let mut t = (N - 1, 0, N - 1, 0);
        for i in 0..N {
            for j in 0..N {
                if T[i][j] == '#' {
                    t.0.chmin(i);
                    t.1.chmax(i);
                    t.2.chmin(j);
                    t.3.chmax(j);
                }
            }
        }

        // println!("{:?}", s);
        // println!("{:?}", t);

        let dx = s.0 - t.0;
        let dy = s.2 - t.2;
        if (s.1 - t.1, s.3 - t.3) == (dx, dy) {
            let mut ok = true;
            for i in s.0..=s.1 {
                for j in s.2..=s.3 {
                    ok &= S[i][j] == T[i - dx][j - dy];
                }
            }
            if ok {
                println!("Yes");
                return;
            }
        }

        //rotate
        T = {
            let mut tmp = vec![vec!['.'; N]; N];
            for i in 0..N {
                for j in 0..N {
                    tmp[i][j] = T[j][N - i - 1];
                }
            }
            tmp
        };
    }

    println!("No");
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
