#![allow(unused_imports)]
#![allow(non_snake_case)]
use itertools::*;
use maplit::*;
use ordered_float::OrderedFloat;
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use proconio::{fastout, input};
use std::collections::*;
use std::fmt::Write;
use superslice::{Ext, Ext2};

#[fastout]
fn main() {
    input! {
        N: usize,
    }

    let mut X = Vec::with_capacity(N);
    let mut Y = Vec::with_capacity(N);
    for _ in 0..N {
        input! {x: i64, y:i64};
        X.push(x);
        Y.push(y);
    }

    //let pi = OrderedFloat(std::f64::consts::PI);
    let pi2 = OrderedFloat(std::f64::consts::PI * 2.0);

    let mut ans = OrderedFloat(0.0);
    for i in 0..N {
        let x0 = X[i];
        let y0 = Y[i];
        let mut t = Vec::<OrderedFloat<f64>>::with_capacity(N - 1);
        for j in 0..N {
            if i == j {
                continue;
            }
            let x = (X[j] - x0) as f64;
            let y = (Y[j] - y0) as f64;
            let theta = OrderedFloat(y.atan2(x)); //-pi ~ +pi
            t.push(theta);
        }
        let mut t = t
            .clone()
            .into_iter()
            .chain(
                t.into_iter()
                    .map(|OrderedFloat(x)| OrderedFloat(x + 2.0 * std::f64::consts::PI)),
            )
            .collect_vec();
        t.sort();
        for &OrderedFloat(theta) in t.iter().take_while(|&x| x <= &pi2) {
            let j = t.upper_bound(&OrderedFloat(theta + std::f64::consts::PI));
            if 1 <= j {
                ans.chmax(OrderedFloat(t[j - 1].into_inner() - theta));
            }
        }
    }
    println!("{}", ans.into_inner().to_degrees());
}
//________________________________________________________________________________
//
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
