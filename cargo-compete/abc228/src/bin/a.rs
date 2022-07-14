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
        S: i32,
        T: i32,
        X: i32,
    }

    if S < T {
        if S <= X && X < T {
            println!("Yes");
        } else {
            println!("No");
        }
    } else {
        if X < T || S <= X {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
