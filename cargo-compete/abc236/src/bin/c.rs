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
        M: usize,
        S: [String;N],
        T: [String;M],
    }

    let T = T.into_iter().collect::<BTreeSet<String>>();
    for s in S.iter() {
        if T.contains(s) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
