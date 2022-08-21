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
        mut T: i64,
        A: [i64;N-1],
        XY: [(Usize1, i64);M],
    }

    let mut P = vec![0;N];
    for &(x, y) in XY.iter(){
        P[x] = y;
    }

    for i in 0..N{
        if T <= 0 {
            println!("No");
            return;
        }

        T += P[i];

        if i < N - 1 {
            T -= A[i];
        }
    }
    println!("Yes");

    
}
