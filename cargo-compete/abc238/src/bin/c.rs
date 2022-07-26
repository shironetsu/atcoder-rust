#![allow(unused_imports)]
#![allow(non_snake_case)]
use itertools::*;
use maplit::*;
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use proconio::{fastout, input};
use std::collections::*;
use std::fmt::Write;
use superslice::{Ext, Ext2};

const MODULO: i64 = 998244353;

#[fastout]
fn main() {
    input! {
        n: i64,
    }

    let mut k = 0;
    let mut ans = 0;
    loop {
        if 10i64.pow(k) <= n && n < 10i64.pow(k + 1) {
            ans +=
                (n - 10i64.pow(k) + 1).rem_euclid(MODULO) * (n - 10i64.pow(k) + 2).rem_euclid(MODULO) / 2;
            break;
        } else {
            ans += (10i64.pow(k + 1) - 10i64.pow(k)).rem_euclid(MODULO)
                * (10i64.pow(k + 1) - 10i64.pow(k) + 1).rem_euclid(MODULO)
                / 2;
        }
        ans = ans.rem_euclid(MODULO);
        k += 1;
    }
    ans = ans.rem_euclid(MODULO);
    println!("{}", ans);
}
