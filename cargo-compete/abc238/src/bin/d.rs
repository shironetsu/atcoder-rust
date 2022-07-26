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
        T: usize,
        cases: [(i64, i64);T],
    }

    for (a, s) in cases {
        let t = s - a * 2;
        let mut ok = true;
        if t < 0 {
            ok = false;
        } else {
            for k in 0..60{
                let p = (a>>k) & 1;
                let q = (t>>k) & 1;
                if p == 1 && q == 1 {
                    ok = false;
                }
            }
        }

        if ok {
            println!("Yes");
        } else {
            println!("No");
        }
    }


}
