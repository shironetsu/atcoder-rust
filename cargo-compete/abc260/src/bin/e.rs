#![allow(unused_imports)]
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
        n: usize,
        m: i32,
        mut ab: [(i32,i32);n],
    }

    ab.sort();
    let mut imos = vec![0; m as usize + 2];
    let amax = ab.iter().map(|t| t.0).max().unwrap();
    let bmin = ab.iter().map(|t| t.1).min().unwrap();
    let mut bmax = vec![0; n];
    for i in 0..n {
        bmax[i] = if i == 0 {
            ab[i].1
        } else {
            bmax[i - 1].max(ab[i].1)
        };
    }

    for l in 1..=bmin {
        let i = ab.lower_bound(&(l, 0));
        let rmin = if i == 0 { amax } else { bmax[i - 1].max(amax) };
        let rmax = m;
        let inn = rmin + 1 - l;
        let out = rmax + 2 - l;
        if inn >= out {
            break;
        }
        imos[inn as usize] += 1;
        imos[out as usize] -= 1;
    }

    for i in 1..=m as usize {
        imos[i] += imos[i - 1];
    }

    let ans = imos[1..=m as usize].iter().map(|x| x.to_string()).join(" ");
    println!("{}", ans);
}
