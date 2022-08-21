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
        P: i64,
        Q: i64,
        R: i64,
        A: [i64;N],
    }

    let mut B = vec![0;N+1];
    for i in 0..N{
        B[i+1] = B[i] + A[i];
    }

    for x in 0..N{
        let a = B[x] + P + Q + R;
        let w = B.lower_bound(&a);
        if w > N || B[w] != a {
            continue;
        }
        let b = B[x] + P;
        let y = B.lower_bound(&b);
        if y >= N || B[y] != b {
            continue;
        }
        let c = B[y] + Q;
        let z = B.lower_bound(&c);
        if z >= N || B[z] != c {
            continue;
        }
        if 0 <= x && x < y && y < z && z < w && w <= N{
            //println!("{} {} {} {}", x, y, z, w);
            println!("Yes");
            return;
        }
    }
    println!("No");
}
