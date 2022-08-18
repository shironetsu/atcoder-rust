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
        mut N: i64,
    }

    let mut ans = vec![];
    loop {
        if N & 1 == 0 {
            N /= 2;
            ans.push('B');
        } else {
            N -= 1;
            ans.push('A');
        }
        if N == 0 {
            break;
        }
    }

    ans.reverse();
    let ans = ans.iter().collect::<String>();
    println!("{}", ans);
}
