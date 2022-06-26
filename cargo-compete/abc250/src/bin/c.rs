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
        x: [Usize1;Q],
    }

    let mut pos = (0..N).collect_vec(); //pos[ball] = position
    let mut ball = (0..N).collect_vec();

    for &x in x.iter() {
        if pos[x] == N - 1 {
            let r = ball[N - 2]; //N>=2
            pos[x] = N - 2;
            pos[r] = N - 1;
            ball[N - 2] = x;
            ball[N - 1] = r
        } else {
            let l = ball[pos[x] + 1];
            ball[pos[x]] = l;
            ball[pos[l]] = x;
            pos[x] += 1;
            pos[l] -= 1;
        }
    }

    let ans = ball.iter().map(|&x| (x + 1).to_string()).join(" ");

    println!("{}", ans);
}
