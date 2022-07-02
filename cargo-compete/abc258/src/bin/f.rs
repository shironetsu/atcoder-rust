#![allow(unused_imports)]
#![allow(non_snake_case)]
use itertools::*;
use maplit::*;
use num_integer::{div_ceil, div_floor};
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use proconio::{fastout, input};
use std::collections::*;
use std::fmt::Write;
use superslice::{Ext, Ext2};

fn dist(x0: i128, y0: i128, x1: i128, y1: i128) -> i128 {
    (x0 - x1).abs() + (y0 - y1).abs()
}

#[fastout]
fn main() {
    input! {
        T: usize,
    }

    for _ in 0..T {
        input! {
            B: i128,
            K: i128,
            mut Sx: i128,
            mut Sy: i128,
            mut Gx: i128,
            mut Gy: i128,
        }

        let a = div_floor(Sx, B) * B;
        let b = div_floor(Sy, B) * B;
        let c = div_floor(Gx, B) * B;
        let d = div_floor(Gy, B) * B;

        let mut ans = dist(Sx, Sy, Gx, Gy) * K;
        for &(x0, y0) in [(a, Sy), (a + B, Sy), (Sx, b), (Sx, b + B)].iter() {
            for &(x1, y1) in [(c, Gy), (c + B, Gy), (Gx, d), (Gx, d + B)].iter() {
                let dt = dist(Sx, Sy, x0, y0) * K + dist(x0, y0, x1, y1) + dist(x1, y1, Gx, Gy) * K;
                ans.chmin(dt);
            }
        }

        println!("{}", ans);

        // if Sx > Gx {
        //     Sx *= -1;
        //     Gx *= -1;
        // }

        // if Sy > Gy {
        //     Sy *= -1;
        //     Gy *= -1;
        // }

        // if Sx - Gx >= B && Sy - Gy >= B {
        //     let a = num_integer::div_ceil(Sx, B) * B;
        //     let b = num_integer::div_ceil(Sy, B) * B;
        //     let c = num_integer::div_floor(Gx, B) * B;
        //     let d = num_integer::div_floor(Gy, B) * B;

        //     let mut ans = (a - c).abs() + (b - d).abs();
        //     if (a - Sx).abs() < (b - Sy).abs() {
        //         ans += (a - Sx).abs() * K + (b - Sy).abs();
        //     } else {
        //         ans += (a - Sx).abs() + (b - Sy).abs() * K;
        //     }

        //     if (Gx - c).abs() < (Gy - d).abs() {
        //         ans += (Gx - c).abs() * K + (Gy - d).abs();
        //     } else {
        //         ans += (Gx - c).abs() + (Gy - d).abs() * K;
        //     }
        //     println!("{}", ans);
        // } else {

        // }
    }
}
pub trait Change<T: PartialOrd> {
    fn chmin(&mut self, rhs: Self) -> bool;
    fn chmax(&mut self, rhs: Self) -> bool;
}
impl<T: PartialOrd> Change<T> for T {
    fn chmax(&mut self, rhs: Self) -> bool {
        if *self < rhs {
            *self = rhs;
            true
        } else {
            false
        }
    }
    fn chmin(&mut self, rhs: Self) -> bool {
        if *self > rhs {
            *self = rhs;
            true
        } else {
            false
        }
    }
}
