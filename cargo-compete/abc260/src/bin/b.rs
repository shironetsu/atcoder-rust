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
        X: usize,
        Y: usize,
        Z: usize,
        A: [i32;N],
        B: [i32;N],
    }

    let mut C = vec![0; N];
    for i in 0..N {
        C[i] = A[i] + B[i];
    }

    let mut a = A
        .into_iter()
        .enumerate()
        .map(|(i, score)| (-score, i))
        .collect_vec();
    let mut b = B
        .into_iter()
        .enumerate()
        .map(|(i, score)| (-score, i))
        .collect_vec();
    let mut c = C
        .into_iter()
        .enumerate()
        .map(|(i, score)| (-score, i))
        .collect_vec();
    a.sort();
    b.sort();
    c.sort();
    // println!("{:?}", a);
    // println!("{:?}", b);
    // println!("{:?}", c);
    let mut ans = vec![];
    let mut used = vec![false; N];
    let mut count = 0;
    for &(_, i) in a.iter() {
        if count == X {
            break;
        }
        if used[i] {
            continue;
        }
        used[i] = true;
        ans.push(i);
        count += 1;
    }
    let mut count = 0;
    for &(_, i) in b.iter() {
        if count == Y {
            break;
        }
        if used[i] {
            continue;
        }
        used[i] = true;
        ans.push(i);
        count += 1;
    }
    let mut count = 0;
    for &(_, i) in c.iter() {
        if count == Z {
            break;
        }
        if used[i] {
            continue;
        }
        used[i] = true;
        ans.push(i);
        count += 1;
    }

    ans.sort();
    for a in ans {
        println!("{}", a + 1);
    }
}
