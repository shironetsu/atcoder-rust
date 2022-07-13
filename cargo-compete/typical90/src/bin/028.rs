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

    let mut n = 1000;
    let mut uu = vec![vec![0i32;n+1];n+1];
    for i in 0..N{
        input!{
            a: usize,
            b: usize,
            c: usize,
            d: usize,
        }
        uu[a][b] += 1;
        uu[c][b] += -1;
        uu[a][d] += -1;
        uu[c][d] += 1;
    }

    for i in 0..=n{
        for j in 0..=n{
            uu[i][j] += if j + !0 <= n { uu[i][j+!0] } else { 0 };
        }
    }

    for j in 0..=n{
        for i in 0..=n{
            uu[i][j] += if i + !0 <= n { uu[i+!0][j] } else { 0 };
        }
    }

    let mut ans = vec![0;N+1];
    for i in 0..n{
        for j in 0..n{            
            ans[uu[i][j] as usize] += 1;
        }
    }

    for i in 1..=N{
        println!("{}", ans[i]);
    }
    

}
