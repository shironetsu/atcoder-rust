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
        L: i64,
        K: usize,
        mut A: [i64;N],
    }

    A.push(L);

    let mut l = 0;
    let mut r = L;
    loop {
        if (r - l) == 1 {
            break;
        }
        let m = (l + r) / 2;
        let mut res = true;
        let mut h = 0;
        for _ in 1..=K + 1 {
            let i = A.lower_bound(&(h + m));
            if i >= A.len() {
                res = false;
                break;
            }
            h = A[i];
        }
        if res {
            l = m;
        } else {
            r = m;
        }
    }
    println!("{}", l);
}
