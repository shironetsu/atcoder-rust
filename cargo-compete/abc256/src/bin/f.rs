#![allow(unused_imports)]
#![allow(non_snake_case)]
use maplit::*;
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use proconio::{fastout, input};
use std::collections::*;
use std::fmt::Write;

#[fastout]
fn main() {
    input! {
        N: usize,
        Q: usize,
        A: [i64;N],
    }

    let mut ans = String::new();
    for _ in 0..Q {
        input! {
            t: i32,
        }
        if t == 1 {
            input! {
                x: Usize1,
                v: i64,
            }
        } else {
            input! {
                x: Usize1,
            }
        }
    }

    print!("{}", ans);
}
