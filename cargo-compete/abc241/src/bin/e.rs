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
        K: i64,
        A: [i64;N],
    }

    let mut v = vec![None;N];
    let mut a = vec![];
    let mut h = 0usize;
    let mut head = vec![];
    let mut circ = vec![];
    loop {
        if let Some(k) = v[h] {
            head = a[..k].iter().copied().collect_vec();
            circ = a[k..].iter().copied().collect_vec();
            break;
        } else {
            v[h] = Some(a.len());
            a.push(h);
            h = (h as i64 + A[h]).rem_euclid(N as i64) as usize;
        }
    }

    let ans = 
    if K as usize <= head.len(){
        head.iter().take(K as usize).map(|&i|A[i]).sum::<i64>()
    } else {
        let p = head.iter().map(|&i|A[i]).sum::<i64>();
        let q = circ.iter().map(|&i|A[i]).sum::<i64>();
        let div = (K-head.len() as i64) / circ.len() as i64;
        let rem = (K-head.len() as i64) % circ.len() as i64;
        let r = circ.iter().take(rem as usize).map(|&i|A[i]).sum::<i64>();
        p + div * q + r
    };

    println!("{}", ans);    
}
