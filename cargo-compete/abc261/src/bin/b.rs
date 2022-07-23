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
        A: [Chars;N],
    }

    for i in 0..N{
        for j in 0..N{
            if i == j {
                continue;
            }
            let a = A[i][j];
            let b = A[j][i];
            let ok = 
                (a, b) == ('W', 'L')
                || (a, b) == ('D', 'D')
                || (a, b) == ('L', 'W');
            if !ok {
                println!("{}", "incorrect");
                return;
            }
        }
    }

    println!("{}", "correct");
}
