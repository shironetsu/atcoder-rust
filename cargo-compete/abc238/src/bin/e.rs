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
        Q: usize,
        lr: [(Usize1,usize);Q],
    }

    let mut ad = vec![vec![]; N + 1];
    for &(l, r) in lr.iter() {
        ad[l].push(r);
        ad[r].push(l);
    }

    let mut visited = vec![false; N + 1];
    let start = 0;
    let mut todo = VecDeque::<usize>::new();
    todo.push_back(start);
    loop {
        if let Some(i) = todo.pop_front() {
            if visited[i] {
                continue;
            }
            visited[i] = true;
            for &j in ad[i].iter() {
                todo.push_back(j);
            }
        } else {
            break;
        }
    }

    if visited[N] {
        println!("Yes");
    } else {
        println!("No");
    }
}
