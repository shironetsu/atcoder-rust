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
        AB: [(f64, f64);N],
    }

    let dt = AB.iter().copied().map(|(a, b)| a/b).collect_vec();
    let mut t = vec![0f64;N+1];
    for i in 0..N{
        t[i+1] = t[i] + dt[i];
    }
    let t2 = t[N]/2.0;
    let i = t.lower_bound_by(|x| x.partial_cmp(&t2).unwrap());
    let res = t2-t[i];
    let ans = AB.iter().copied().take(i).map(|(a, _)|a).sum::<f64>() + if i < N { res * AB[i].1 } else { 0 };
    println!("{}", ans);
}
