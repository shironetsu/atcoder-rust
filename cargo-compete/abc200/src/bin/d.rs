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
    let mut dp = vec![vec![0; m as usize]; N + 1];
    dp[0][0] = 1;
    for i in 0..N {
        dp[i + 1] = dp[i].clone();
        for j in 0..m as usize {
            let k = ((j as i64 + A[i]) % m) as usize;
            dp[i + 1][k] += dp[i][j];
        }

        for j in 0..m as usize {
            if (j == 0 && dp[i + 1][j] >= 3) || (j > 0 && dp[i + 1][j] >= 2) {
                let mut b = vec![false; i + 1];
                let mut c = vec![false; i + 1];
                let mut jj = j;
                for ii in (0..=i).rev() {
                    let jj1 = (jj as i64 - A[ii]).rem_euclid(m) as usize;
                    if dp[ii][jj1] > 0 {
                        jj = jj1;
                        b[ii] = true;
                    } else {
                        b[ii] = false;
                    }
                }

                let mut jj = j;
                let mut f = false;
                for ii in (0..=i).rev() {
                    let jj1 = (jj as i64 - A[ii]).rem_euclid(m) as usize;
                    if !f {
                        if dp[ii][jj1] >= 2 || dp[ii][jj] == 0 {
                            jj = jj1;
                            c[ii] = true;
                            f = true;
                        } else {
                            c[ii] = false;
                        }
                    } else {
                        if dp[ii][jj] > 0 {
                            c[ii] = false;
                        } else {
                            c[ii] = true;
                            jj = jj1;
                        }
                    }
                }

                let b = (0..=i).filter(|ii| b[*ii]).map(|i| i + 1).collect_vec();
                let c = (0..=i).filter(|ii| c[*ii]).map(|i| i + 1).collect_vec();
                println!("Yes");
                println!("{} {}", b.len(), b.iter().map(|i| i.to_string()).join(" "));
                println!("{} {}", c.len(), c.iter().map(|i| i.to_string()).join(" "));
                return;
            }
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
