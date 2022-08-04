#![allow(unused_imports)]
#![allow(non_snake_case)]
use itertools::*;
use maplit::*;
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use proconio::{fastout, input};
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::collections::*;
use std::fmt::Write;
use superslice::{Ext, Ext2};

#[derive(PartialEq, Eq, Clone, Copy, Debug, Ord)]
struct Proj {
    x: i64,
    y: i64,
}

impl Proj {
    fn new(x: i64, y: i64) -> Proj {
        let g = num_integer::gcd(x, y);
        let x = x / g;
        let y = y / g;
        Proj { x, y }
    }
}

impl PartialOrd for Proj {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let (x0, y0) = (self.x, self.y);
        let (x1, y1) = (other.x, other.y);
        (x0 * y1).partial_cmp(&(x1 * y0)).map(|ord| ord.reverse())
    }

    fn lt(&self, other: &Self) -> bool {
        self.partial_cmp(&other) == Some(Ordering::Less)
    }
    fn le(&self, other: &Self) -> bool {
        self.partial_cmp(&other) != Some(Ordering::Greater)
    }
    fn gt(&self, other: &Self) -> bool {
        self.partial_cmp(&other) == Some(Ordering::Greater)
    }
    fn ge(&self, other: &Self) -> bool {
        self.partial_cmp(&other) != Some(Ordering::Less)
    }
}

#[fastout]
fn main() {
    input! {
        N: usize,
    }

    let mut a = vec![];
    let mut b = vec![];
    for i in 0..N {
        input! {
            x: i64,
            y: i64,
        }
        a.push(Proj::new(x, y - 1));
        b.push(Proj::new(x - 1, y));
    }
    let mut ib = b.into_iter().enumerate().collect_vec();
    ib.sort_by_key(|pair| pair.1);
    //let ii = bb.iter().map(|(i, _)|i).collect_vec();
    let mut t = ib[0].1;
    let mut ans = 1;
    for &(i, b) in ib.iter().skip(1) {
        if a[i] < t {
            continue;
        }
        ans += 1;
        t = b;
    }
    println!("{:?}", ans);
}
