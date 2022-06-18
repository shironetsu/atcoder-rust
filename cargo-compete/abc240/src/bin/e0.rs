use proconio::fastout;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use std::collections::BTreeSet;
#[allow(unused_imports)]
use std::fmt::Write;

fn dfs(
    v: usize,
    ad: &Vec<BTreeSet<usize>>,
    seen: &mut Vec<bool>,
    c: &mut usize,
    lr: &mut Vec<(usize, usize)>,
) -> (usize, usize) {
    let mut is_leaf = true;
    let mut lmin = std::usize::MAX;
    let mut rmax = 0;
    for &w in ad[v].iter() {
        if !seen[w] {
            seen[w] = true;
            is_leaf = false;
            let (l, r) = dfs(w, ad, seen, c, lr);
            if l < lmin {
                lmin = l;
            }
            if rmax < r {
                rmax = r;
            }
        }
    }
    if is_leaf {
        lr[v] = (*c, *c);
        *c += 1;
    } else {
        lr[v] = (lmin, rmax);
    }
    lr[v]
}

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        uv: [(Usize1, Usize1);N-1],
    }

    let mut ad = (0..N).map(|_| BTreeSet::new()).collect::<Vec<_>>();
    for &(u, v) in uv.iter() {
        ad[u].insert(v);
        ad[v].insert(u);
    }
    let mut seen = vec![false; N];
    let mut lr = vec![(0, 0); N];
    let mut c = 1;
    seen[0] = true;
    dfs(0, &mut ad, &mut seen, &mut c, &mut lr);
    for &(l, r) in lr.iter() {
        println!("{} {}", l, r);
    }
}
