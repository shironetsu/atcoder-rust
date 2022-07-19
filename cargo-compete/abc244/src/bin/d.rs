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
        S: [char;3],
        T: [char;3],
    }

    let toi = btreemap!(
        'R' => 0,
        'G' => 1,
        'B' => 2,
    );
    let s = S.into_iter().map(|c| toi.get(&c).unwrap()).collect_vec();
    let t = T.into_iter().map(|c| toi.get(&c).unwrap()).collect_vec();
    let mut a = 0;
    let mut b = 0;
    //println!("{:?}", s);
    //println!("{:?}", t);
    for i in 0..3 {
        for j in i + 1..3 {
            if s[i] > s[j] {
                a += 1;
            }
            if t[i] > t[j] {
                b += 1;
            }
        }
    }
    if (a ^ b) & 1 == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
