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
    }
    let mut x = vec![0; N];
    let mut y = vec![0; N];

    for i in 0..N {
        input! {
            xi: i64,
            yi: i64,
        }
        x[i] = xi;
        y[i] = yi;
    }

    let u = (0..N).map(|i| x[i] - y[i]).collect_vec();
    let v = (0..N).map(|i| x[i] + y[i]).collect_vec();

    let umin = u.iter().min().unwrap();
    let umax = u.iter().max().unwrap();
    let vmin = v.iter().min().unwrap();
    let vmax = v.iter().max().unwrap();

    for _ in 0..Q {
        input! {
            q: Usize1,
        }

        let &ans = [u[q] - umin, umax - u[q], v[q] - vmin, vmax - v[q]]
            .iter()
            .max()
            .unwrap();
        println!("{}", ans);
    }
}
