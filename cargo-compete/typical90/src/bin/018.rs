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
        T: f64,
        L: f64,
        X: f64,
        Y: f64,
        Q: usize,
        E: [f64;Q],
    }

    for e in E {
        let t = e / T * std::f64::consts::PI * 2.0;
        let vx = X - 0.0;
        let vy = Y + L / 2.0 * t.sin();
        let vz = 0.0 - (L / 2.0 - L / 2.0 * t.cos());
        let inner = (vx * vx + vy * vy);
        let vv = vx.powi(2) + vy.powi(2) + vz.powi(2);
        let ww = vx.powi(2) + vy.powi(2);
        let cos = inner / (vv * ww).sqrt();
        let ans = cos.acos().to_degrees();
        println!("{}", ans);
    }
}
