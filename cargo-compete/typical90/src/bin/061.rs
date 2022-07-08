#![allow(unused_imports)]
#![allow(non_snake_case)]
use itertools::*;
use maplit::*;
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use proconio::{fastout, input};
use std::collections::*;
use std::fmt::Write;
use superslice::{Ext, Ext2};

#[fastout]
fn main() {
    input! {
        Q: usize,
    }
    let mut a = vec![];
    let mut b = vec![];
    for i in 0..Q{
        input!{
            t: i32,
        }
        if t == 1{
            input!{x:i32}
            a.push(x);
        }else if t==2{
            input!{x:i32}
            b.push(x);
        }else{
            input!{x:Usize1}
            if x < a.len(){
                println!("{}", a[a.len()-x-1]);
            } else {
                println!("{}", b[x-a.len()]);
            }
        }
    }
}
