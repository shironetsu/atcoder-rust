#![allow(unused_imports)]
#![allow(non_snake_case)]
use itertools::*;
use maplit::*;
use num_complex::Complex;
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use proconio::{fastout, input};
use std::collections::*;
use std::fmt::Write;
use superslice::{Ext, Ext2};

#[fastout]
fn main() {
    input! {
        a: f64,
        b: f64,
        d: f64,
    }

    // let t = d * std::f64::consts::PI / 180.0;
    // let c = t.cos();
    // let s = t.sin();
    // let x = c * a - s * b;
    // let y = s * a + c * b;
    // println!("{} {}", x, y);

    let x = Complex::new(a, b) * Complex::new(0.0, d.to_radians()).exp();
    println!("{} {}", x.re, x.im);
}
