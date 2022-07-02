#![allow(unused_imports)]
#![allow(non_snake_case)]
use maplit::*;
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use proconio::{fastout, input};
use std::collections::*;
use std::fmt::Write;

#[fastout]
fn main() {
    input! {
        N: usize,
        Q: usize,
        a: [i64;N],
    }

    let mut m = BTreeMap::<i64, Vec<usize>>::new();
    for (i, &x) in a.iter().enumerate() {
        m.entry(x).or_insert(vec![]).push(i);
    }

    for _ in 0..Q {
        input! {
            x: i64,
            k: Usize1,
        }
        if let Some(b) = m.get(&x) {
            if b.len() <= k {
                println!("{}", -1);
            } else {
                let i = b[k];
                println!("{}", i + 1);
            }
        } else {
            println!("{}", -1);
        }
    }
}
