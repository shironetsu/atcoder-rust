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
        M: usize,
        UV: [(Usize1,Usize1);M],
    }

    let mut ad = vec![vec![false;N];N];
    for &(u, v) in UV.iter(){
        ad[u][v] = true;
        ad[v][u] = true;
    }

    let mut ans = 0;
    for i in 0..N{
        for j in i+1..N{
            for k in j+1..N{
                if ad[i][j] && ad[j][k] && ad[i][k] {
                    ans+=1;
                }
            }
        }
    }
    println!("{}", ans);



    
}
