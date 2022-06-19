use maplit::btreeset;
use proconio::marker::Usize1;
use proconio::{fastout, input};
use std::collections::BTreeSet;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        uv: [(Usize1, Usize1);N-1]
    }

    let mut ad = vec![btreeset!(); N];
    for &(u, v) in uv.iter() {
        ad[u].insert(v);
        ad[v].insert(u);
    }
    let mut seen = vec![false; N];
    let mut fl = vec![(0, 0); N];
    let mut lr = vec![(0, 0); N];
    let mut t = 0;
    let mut l = 1;
    let mut r = 1;
    dfs(
        0, &mut ad, &mut seen, &mut fl, &mut lr, &mut t, &mut l, &mut r,
    );
    for &(l, r) in lr.iter() {
        println!("{} {}", l, r);
    }
}

fn dfs(
    v: usize,
    ad: &Vec<BTreeSet<usize>>,
    seen: &mut Vec<bool>,
    fl: &mut Vec<(usize, usize)>,
    lr: &mut Vec<(usize, usize)>,
    t: &mut usize,
    l: &mut usize,
    r: &mut usize,
) {
    seen[v] = true;
    fl[v].0 = *t;
    lr[v].0 = *l;
    *t += 1;
    for &w in ad[v].iter() {
        if !seen[w] {
            dfs(w, ad, seen, fl, lr, t, l, r);
        }
    }
    fl[v].1 = *t;
    *t += 1;
    if fl[v].0 + 1 == fl[v].1 {
        *r = *l;
        *l += 1;
    }
    lr[v].1 = *r;
}