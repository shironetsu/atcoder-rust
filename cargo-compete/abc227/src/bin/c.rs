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
        n: i64,
    }

    let mut ans = 0i64;
    for a in 1..=num_integer::cbrt(n){
        for b in a..{
            let cmax = n / (a*b);
            if cmax < b {
                break;
            }
            let m = cmax - b + 1;
            ans += m;
        }
    }

    println!("{}", ans);

    
}
