#![allow(unused_imports)]
#![allow(non_snake_case)]
use maplit::*;
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use proconio::{fastout, input};
use std::collections::*;
use std::fmt::Write;

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

#[fastout]
fn main() {
    input! {
        N: usize,
        K: usize,
        A: [u32;N],
        B: [Usize1;K],
    }

    let mut hates = btreeset!();
    let mut amax = 0;
    for (i, &a) in A.iter().enumerate() {
        if amax.chmax(a) {
            hates = btreeset!(i);
        } else if amax == a {
            hates.insert(i);
        }
    }

    let b = B.into_iter().collect::<BTreeSet<_>>();
    if b.intersection(&hates).count() > 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
