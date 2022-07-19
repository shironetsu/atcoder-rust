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
        T: Chars,
    }

    let mut x = 0;
    let mut y = 0;
    let mut vx = 1;
    let mut vy = 0;
    for t in T {
        if t == 'S' {
            x += vx;
            y += vy;
        } else {
            let tmp = vx;
            vx = vy;
            vy = -tmp;
        }
    }

    println!("{} {}", x, y);
    
}
