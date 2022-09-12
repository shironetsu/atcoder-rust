#![allow(unused_imports)]
#![allow(non_snake_case)]
use itertools::*;
use maplit::*;
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use proconio::{fastout, input};
use std::collections::*;
use std::fmt::{Display, Write};
use superslice::{Ext, Ext2};

#[fastout]
fn main() {
    input! {
        N: usize,
        A: [i64;N],
    }

    let m = 200i64;
    let mut dp = vec![vec![false; m as usize]; N + 1];
    dp[0][0] = true;
    for i in 0..N {
        dp[i + 1] = dp[i].clone();
        for j in 0..m as usize {
            let k = ((j as i64 + A[i]) % m) as usize;
            if k == 0 && dp[i][j] && i < N - 1 {
                let mut b = vec![false; i + 2];
                b[i+1] = true;
                let mut c = vec![false; i + 2];
                let mut jj = j;
                c[i+1] = true;
                c[i] = true;
                for ii in (0..i).rev() {
                    let jj1 = (jj as i64 - A[ii] as i64).rem_euclid(m) as usize;
                    if dp[ii][jj1] {
                        jj = jj1;
                        c[ii] = true;
                    } else {
                        c[ii] = false;
                    }
                }
                let b = (0..=i).filter(|i| b[*i]).map(|i| i + 1).collect_vec();
                let c = (0..=i).filter(|i| c[*i]).map(|i| i + 1).collect_vec();
                if b.len().min(c.len()) > 0 {
                    println!("Yes");
                    let bns = b.iter().map(|i|i.to_string()).join(" ");
                    println!("{} {}", b.len(), bns);
                    let cns = c.iter().map(|i|i.to_string()).join(" ");
                    println!("{} {}", c.len(), cns);
                    return;
                }
            }

            if dp[i + 1][k] && dp[i][j] {
                let mut b = vec![false; i + 1];
                let mut c = vec![false; i + 1];

                let mut kk = k;
                b[i] = false;
                for ii in (0..i).rev() {
                    let kk1 = (kk as i64 - A[ii] as i64).rem_euclid(m) as usize;
                    if dp[ii][kk1] {
                        kk = kk1;
                        b[ii] = true;
                    } else {
                        b[ii] = false;
                    }
                }

                let mut jj = j;
                c[i] = true;
                for ii in (0..i).rev() {
                    let jj1 = (jj as i64 - A[ii] as i64).rem_euclid(m) as usize;
                    if dp[ii][jj1] {
                        jj = jj1;
                        c[ii] = true;
                    } else {
                        c[ii] = false;
                    }
                }
                let b = (0..=i).filter(|i| b[*i]).map(|i| i + 1).collect_vec();
                let c = (0..=i).filter(|i| c[*i]).map(|i| i + 1).collect_vec();
                if b.len().min(c.len()) > 0 {
                    println!("Yes");
                    let bns = b.iter().map(|i|i.to_string()).join(" ");
                    println!("{} {}", b.len(), bns);
                    let cns = c.iter().map(|i|i.to_string()).join(" ");
                    println!("{} {}", c.len(), cns);
                    return;
                }

            }
            dp[i + 1][k] |= dp[i][j];
        }
    }
    println!("No");
}
//______________________________________________________________________________
//
pub trait Answer {
    fn ans(&self);
    fn ansl(&self);
}

impl<T: Display> Answer for Vec<T> {
    fn ans(&self) {
        let ans = self
            .iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<_>>()
            .join(" ");
        println!("{}", ans);
    }

    fn ansl(&self) {
        let ans = self
            .iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<_>>()
            .join("\n");
        println!("{}", ans);
    }
}
//______________________________________________________________________________
//
#[macro_export]
macro_rules! input_edges {
    ($n: expr, $m: expr, $edges: tt, $ad: tt) => {
        input! {
            $edges: [(Usize1, Usize1); $m],
        }

        let mut $ad = vec![vec![]; $n];
        for &(a, b) in $edges.iter() {
            $ad[a].push(b);
            $ad[b].push(a);
        }
        let $ad = $ad;
    };
}
