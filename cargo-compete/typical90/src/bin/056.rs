#![allow(unused_imports)]
#![allow(non_snake_case)]
use bitset_fixed::BitSet;
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
        S: usize,
        AB: [(usize, usize);N],
    }

    // let mut dp = vec![];
    // dp.push(btreemap!(0 => BitSet::new(N)));
    // for (i, &(a, b)) in AB.iter().enumerate() {
    //     let mut s = BTreeMap::<usize, BitSet>::new();
    //     for (c, bs) in dp[i].iter() {
    //         if c + a <= S {
    //             let d = c + a;
    //             s.entry(d).or_insert(bs.clone());
    //         }
    //         if c + b <= S {
    //             let d = c + b;
    //             s.entry(d).or_insert({let mut bs = bs.clone(); bs.set(i, true); bs});
    //         }
    //     }
    //     dp.push(s);
    // }
    // if let Some((_, bs)) = dp[N].iter().find(|(c, _)|{**c==S}){
    //     let ans = (0..N).map(|i|{if bs[i] { 'B' } else { 'A' }}).collect::<String>();
    //     println!("{}", ans);
    // } else {
    //     println!("Impossible");
    // }

    // let mut dp = vec![vec![false; S + 1]; N + 1];
    // dp[0][0] = true;
    // for (i, &(a, b)) in AB.iter().enumerate() {
    //     for j in 0..=S {
    //         if j + a <= S {
    //             dp[i + 1][j + a] |= dp[i][j];
    //         }
    //         if j + b <= S {
    //             dp[i + 1][j + b] |= dp[i][j];
    //         }
    //     }
    // }
    // let mut v = vec![];
    // let mut h = S;
    // if dp[N][S] {
    //     for (i, &(a, b)) in AB.iter().enumerate().rev() {
    //         if h >= a && dp[i][h - a] {
    //             v.push('A');
    //             h -= a;
    //         } else {
    //             v.push('B');
    //             h -= b;
    //         }
    //     }
    //     let ans = v.iter().rev().collect::<String>();
    //     println!("{}", ans);
    // } else {
    //     println!("Impossible");
    // }

    let mut dp = vec![BitSet::new(S + 1); N + 1];
    dp[0].set(0, true);
    for (i, &(a, b)) in AB.iter().enumerate() {
        let (a, b) = (a.min(b), a.max(b));
        dp[i + 1] = dp[i].clone() << a;
        dp[i + 1].shl_or(b - a);
        //dp[i + 1] = (dp[i].clone() << a) | &(dp[i].clone() << b);
    }

    if dp[N][S] {
        let mut s = S;
        let mut v = Vec::with_capacity(N);
        for (i, &(a, b)) in AB.iter().enumerate().rev() {
            if s >= a && dp[i][s - a] {
                s -= a;
                v.push('A');
            } else {
                s -= b;
                v.push('B');
            }
        }
        let ans = v.iter().rev().collect::<String>();
        println!("{}", ans);
    } else {
        println!("Impossible");
    }
}
