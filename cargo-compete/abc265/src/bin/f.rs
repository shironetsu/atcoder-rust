#![allow(unused_imports)]
#![allow(non_snake_case)]
use proconio::{input, fastout};
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use std::fmt::Write;
use std::collections::*;
use maplit::*;
use itertools::*;
use superslice::{Ext, Ext2};

const M: i64 = 998244353;

#[fastout]
fn main() {
    input!{
        N: usize,
        D: i64,
        p: [i64;N],
        mut q: [i64;N],
    }

    for i in 0..N{
        q[i] -= p[i];
    }

    let mut dp = vec![vec![0;(D+1) as usize];(D+1) as usize];
    dp[0][0] = 1;
    for i in 0..N-1{
        for d0 in 0..=D {
            for d1 in 0..=D{
                let a = D - d0;
                let b = D - d1;
                let l = (-a).max(q-b);
                let r = a.min(q+b);
                
            }
        }
    }


    
}
