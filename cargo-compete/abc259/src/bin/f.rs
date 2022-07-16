#![allow(unused_imports)]
#![allow(non_snake_case)]
use itertools::*;
use maplit::*;
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use proconio::{fastout, input};
use std::collections::*;
use std::fmt::Write;
use superslice::{Ext, Ext2};

const INF: i64 = 2_000_000_000;

#[fastout]
fn main() {
    input! {
        N: usize,
        d: [i64;N],
        uvw: [(Usize1, Usize1, i64);N-1],
    }

    let mut w = BTreeMap::<(usize, usize), i64>::new();
    let mut ad = vec![vec![]; N];
    for &(u, v, ww) in uvw.iter() {
        ad[u].push(v);
        ad[v].push(u);
        w.insert((u, v), ww);
        w.insert((v, u), ww);
    }

    let mut seen = vec![false; N];
    seen[0] = true;
    let (lte, lt) = dfs(0, &mut seen, &ad, &w, &d);
    println!("{}", lte);
}

fn dfs(
    u: usize,
    seen: &mut Vec<bool>,
    ad: &Vec<Vec<usize>>,
    w: &BTreeMap<(usize, usize), i64>,
    d: &Vec<i64>,
) -> (i64, i64) {
    let mut bh = BinaryHeap::<(i64, i64, i64)>::new();
    for &v in ad[u].iter() {
        if seen[v] {
            continue;
        }
        seen[v] = true;
        let (lte, lt) = dfs(v, seen, ad, w, d);
        let delta = if d[v] > 0 { w.get(&(u, v)).unwrap() - (lte - lt) } else { -INF };
        bh.push((delta, lt, lte));
    }

    let mut count = 0;
    let mut lte = 0;
    let mut lt = 0;
    loop {
        if let Some((delta, lt0, lte0)) = bh.pop() {
            count += 1;
            if count < d[u] && delta > 0 {
                lte += delta + lte0;
                lt += delta + lte0;
            } else if count == d[u] && delta > 0 {
                lte += delta + lte0;
                lt += lte0;
            } else {
                lte += lte0;
                lt += lte0;
            }
        } else {
            break;
        }
    }
    (lte, lt)
}
