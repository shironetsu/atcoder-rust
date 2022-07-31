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
        a: [Usize1;N],
    }

    let mut ans = 0i64;

    let mut n = 0;
    for (i, &x) in a.iter().enumerate(){
        if i == x {
            n += 1;
        }
    }

    ans += n * (n-1) / 2 ;

    let mut m = 0;
    for (i, &j) in a.iter().enumerate(){
        if 0<=j && j < N && a[j] == i && i < j {
            ans += 1;
        }
    }

    println!("{}", ans);


    
}
