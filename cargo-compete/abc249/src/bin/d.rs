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
        A: [i64;N],
    }

    let mut count = BTreeMap::<i64, i64>::new();
    for &a in A.iter() {
        *count.entry(a).or_insert(0) += 1;
    }

    let mut ans = 0i64;
    for (&a, &c) in count.iter() {
        let r = num_integer::sqrt(a);
        for d in 1..=r {
            if a % d == 0 {
                let e = a / d;
                if d * e != a {
                    continue;
                }
                let t = count.get(&d).unwrap_or(&0) * count.get(&e).unwrap_or(&0);
                if d < e {
                    ans += 2 * c * t;
                } else if d == e && e == r {
                    ans += c * t;
                }
            }
        }
    }

    println!("{}", ans);
}
