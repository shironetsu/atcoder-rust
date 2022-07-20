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
        A: [i64;N],
        B: [i64;N],
    }

    let mut a = 0;
    let mut b = 0;
    for i in 0..N{
        for j in 0..N{
            if A[i] == B[j]{
                if i == j {
                    a += 1;
                } else {
                    b += 1;
                }
            }
        }
    }
    println!("{}", a);
    println!("{}", b);

    
}
