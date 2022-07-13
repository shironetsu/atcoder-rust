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
        mut W: i64,
        AB: [(i64, i64);N],
    }

    let mut h = AB.into_iter().collect::<BinaryHeap::<_>>();
    let mut ans = 0;
    loop {
        if let Some((a, b)) = h.pop(){
            ans += a * b.min(W);
            W -= b;
            if W <= 0 {
                break;
            }
        } else {
            break;
        }
    }

    println!("{}", ans);
    
}
