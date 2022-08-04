#![allow(unused_imports)]
#![allow(non_snake_case)]
use proconio::{input, fastout};
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use std::fmt::Write;
use std::collections::*;
use maplit::*;
use itertools::*;
use superslice::{Ext, Ext2};

#[fastout]
fn main() {
    input!{
        N: usize,
        XY: [(i64,i64);N],
    }

    let mut ans = 0;
    for i in 0..N{
        for j in i+1..N{
            for k in j+1..N{
                let (mut a, mut b) = XY[i];
                let (mut c, mut d) = XY[j];
                let (e, f) = XY[k];
                a -= e;
                c -= e;
                b -= f;
                d -= f;
                let s = (a*d-b*c).abs();
                if s > 0 {
                    ans += 1;
                }
            }
        }
    }

    println!("{}", ans);
}
