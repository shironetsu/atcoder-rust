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
        h1: i32,
        h2: i32,
        h3: i32,
        w1: i32,
        w2: i32,
        w3: i32,
    }

    let mut ans = 0;
    for a in 1..=28 {
        for b in 1..=28 {
            for d in 1..=28 {
                for e in 1..=28 {
                    let g = h1 - a - d;
                    let h = h2 - b - e;
                    let c = w1 - a - b;
                    let f = w2 - d - e;
                    if w3 - g - h == h3 - c - f {
                        let i = w3 - g - h;
                        if g > 0 && h > 0 && c > 0 && f > 0 && i > 0 {
                            ans += 1;
                        }
                    }
                }
            }
        }
    }
    println!("{}", ans);
}
