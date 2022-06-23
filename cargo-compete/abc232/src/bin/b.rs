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
        S: Chars,
        T: Chars,
    }
    if S.iter()
        .zip(T.iter())
        .map(|(&s, &t)| (s as i32 - t as i32).rem_euclid(26))
        .collect::<BTreeSet<_>>()
        .len()
        == 1
    {
        println!("Yes");
    } else {
        println!("No");
    }
}
