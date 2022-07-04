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
        K: usize,
        S: [String;N],
    }

    let mut has = vec![vec![false; 26]; N];
    let mut atoi = BTreeMap::new();
    for i in 0..26 {
        let c = ('a' as u8 + i as u8) as char;
        atoi.insert(c, i);
    }

    for i in 0..N {
        for c in S[i].chars() {
            has[i][atoi[&c]] = true;
        }
    }

    let mut ans = 0;

    for s in 0..1 << N {
        let mut count = vec![0; 26];
        for i in 0..N {
            if (s >> i) & 1 == 1 {
                for j in 0..26 {
                    if has[i][j] {
                        count[j] += 1;
                    }
                }
            }
        }
        let a = count.iter().filter(|&&c| c == K).count();
        ans.chmax(a);
    }

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
