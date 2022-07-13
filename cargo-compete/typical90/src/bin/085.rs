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
        mut K: i64,
    }

    let mut d = 2;
    let mut e = BTreeMap::<i64, i64>::new();
    while d * d <= K {
        while K % d == 0 {
            K /= d;
            *e.entry(d).or_default() += 1;
        }
        d += 1;
    }
    if K > 1 {
        *e.entry(K).or_default() += 1;
    }

    //println!("{:?}", e);

    let mut a = 1i64;
    let mut b = 1i64;
    let mut c = 1i64;
    for (_, &p) in e.iter() {
        a *= h3(p);
        b *= (p / 2) + 1;
        if p % 3 != 0 {
            c = 0;
        }
    }

    //println!("{} {} {}", a, b, c);

    let mut ans = (a + 3 * b + 6 * c) / 6;

    println!("{}", ans);
}

fn h3(n: i64) -> i64 {
    (n + 2) * (n + 1) / 2
}
