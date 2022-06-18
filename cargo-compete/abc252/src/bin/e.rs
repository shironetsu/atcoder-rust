#![allow(unused_imports)]
#![allow(non_snake_case)]
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use proconio::{fastout, input};
use std::cmp::Reverse;
use std::collections::*;
use std::fmt::Write;

#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
        ABC: [(Usize1,Usize1,i64);M],
    }

    let mut ad = Vec::<BTreeSet<(usize, Reverse<i64>, usize)>>::new();
    for _ in 0..N {
        ad.push(BTreeSet::new());
    }
    for (i, &(a, b, c)) in ABC.iter().enumerate() {
        ad[a].insert((b, Reverse(c), i));
        ad[b].insert((a, Reverse(c), i));
    }
    //distance, to, road
    let mut pq = BinaryHeap::<(Reverse<i64>, usize, usize)>::new();
    for &(v, c, i) in ad[0].iter() {
        pq.push((c, v, i));
    }

    let mut visited = vec![false; N];
    let mut ans = vec![0; N];
    visited[0] = true;
    loop {
        if let Some((c, v, i)) = pq.pop() {
            if visited[v] {
                continue;
            } else {
                visited[v] = true;
            }
            ans[v] = i;
            for &(w, d, i) in ad[v].iter() {
                pq.push((Reverse(c.0 + d.0), w, i));
            }
        } else {
            break;
        }
    }

    for i in 1..N {
        print!("{} ", ans[i] + 1);
    }
}
