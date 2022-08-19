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
        K: usize,
        A: [i64;N],
    }

    let mut pq = BinaryHeap::new();
    for &a in A.iter(){
        pq.push(a);
    }

    let mut ans = 0;
    for _ in 0..K{
        if let Some(a) = pq.pop() {
            ans += a;
            if a > 1 {
                pq.push(a-1);
            }
        } else {
            break;
        }
    }

    println!("{}", ans);
}
