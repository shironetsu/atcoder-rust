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
    }

    // let mut s = vec![vec![];N+1];
    // s[1] = vec![1];
    // for i in 2..=N{
    //     let mut v = vec![];
    //     for (j, &x) in s[i-1].iter().enumerate(){
    //         v.push(x);
    //     }
    //     v.push(i);
    //     for (j, &x) in s[i-1].iter().enumerate(){
    //         v.push(x);
    //     }
    //     s[i] = v;
    // }

    // let ans = s[N].iter().map(|x|x.to_string()).join(" ");

    let m = 1 << N;
    let mut S = vec![1; m];

    for i in 1..N {
        let mut j = 1 << i;
        while j < m {
            S[j] += 1;
            j += 1 << i;
        }
    }

    let ans = S.iter().skip(1).map(|s| s.to_string()).join(" ");

    println!("{}", ans);
}
