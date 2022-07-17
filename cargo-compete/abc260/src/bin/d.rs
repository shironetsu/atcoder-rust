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
        K: usize,
        P: [Usize1;N],
    }

    let mut under = vec![None; N];
    let mut pile = vec![0; N];
    let mut ans = vec![None; N];
    let mut top = BTreeSet::<usize>::new();
    for (i, &p) in P.iter().enumerate() {
        if let Some(&t) = top.range(p..).next() {
            under[p] = Some(t);
            pile[p] = (pile[t] + 1) as i32;
            top.remove(&t);
        } else {
            pile[p] = 1;
        }
        
        top.insert(p);

        if pile[p] == K as i32 {
            let mut j = p;
            loop {
                ans[j] = Some(i);
                if let Some(k) = under[j] {
                    j = k;
                } else {
                    break;
                }
            }
            top.remove(&p);
        }
    }
    let ans = ans
        .iter()
        .map(|op| op.map_or(-1, |i| i as i32 + 1))
        .collect_vec();

    for a in ans {
        println!("{}", a);
    }
}
