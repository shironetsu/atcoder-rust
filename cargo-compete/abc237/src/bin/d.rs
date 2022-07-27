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
        S: Chars,
    }

    let mut a = VecDeque::<i32>::new();
    a.push_back(N as i32);
    for (i, &d) in S.iter().rev().enumerate(){
        let n = (N - i - 1) as i32;
        if d == 'L' {
            a.push_back(n);
        } else {
            a.push_front(n);
        }
    }

    let ans = a.iter().map(|x|x.to_string()).join(" ");

    println!("{}", ans);

    
}
