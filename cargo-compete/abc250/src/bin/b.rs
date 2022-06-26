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
        A: usize,
        B: usize,
    }

    //let mut ans = vvec![' ';A*N,B*N];
    for i in 0..A*N{
        for j in 0..B*N{
            let a = i / A;
            let b = j / B;
            if (a+b)&1 == 0 {
                print!(".");
                //ans[i][j] = '.';
            } else {
                print!("#");
                //ans[i][j] = '#';
            }
        }
        println!();
    }
    
}

#[macro_export]
macro_rules! vvec {
    ($ val : expr ; $ a : expr , $ b : expr ) => {
        vec![vec![$val; $b]; $a]
    };
}
