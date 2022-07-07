#![allow(unused_imports)]
#![allow(non_snake_case)]
use itertools::*;
use maplit::*;
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use proconio::{fastout, input};
use std::collections::*;
use std::fmt::Write;
use superslice::{Ext, Ext2};

const MODULO: i64 = 100_000;
fn f(x: i64)->i64{
    let y = x.to_string().chars().map(|c|{(c as u8 - '0' as u8) as i64}).sum::<i64>();
    (x + y).rem_euclid(MODULO)
}

#[fastout]
fn main() {
    input! {
        N: i64,
        K: i64,
    }
    let mut e = BTreeMap::<i64, i64>::new();
    let mut g = vec![];
    let mut x = N;
    let mut i = 0i64;
    loop{
        if let Some(&j) = e.get(&x){
            let period = i - j;
            let ans = if K < j {
                g[K as usize]
            } else {
                let r = (K-j).rem_euclid(period);
                g[(j + r) as usize]
            };
            println!("{}", ans);
            return;
        } else {
            e.insert(x, i);
            g.push(x);
            i += 1;
            x = f(x);
        }
    }

    


}
