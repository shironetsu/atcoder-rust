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
        N: usize,
        Q: usize,
        mut A: [i32;N],
    }

    let mut c = 0; //cursor
    for _ in 0..Q {
        input! {
            t: i32,
            mut x: i32,
            mut y: i32,
        }
        x -= 1;
        y -= 1;

        if t == 1 {
            let i = (x - c).rem_euclid(N as i32) as usize;
            let j = (y - c).rem_euclid(N as i32) as usize;
            let tmp = A[i];
            A[i] = A[j];
            A[j] = tmp;
            //std::mem::swap(&mut A[i], &mut A[j]);
        } else if t == 2 {
            c += 1;
        } else if t == 3 {
            let i = (x - c).rem_euclid(N as i32) as usize;
            println!("{}", A[i]);
        }
    }
}
