#![allow(unused_imports)]
#![allow(non_snake_case)]
use itertools::*;
use maplit::*;
use petgraph::unionfind::UnionFind;
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use proconio::{fastout, input};
use std::collections::*;
use std::fmt::Write;
use superslice::{Ext, Ext2};

#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
    }
    let mut inn = vec![btreeset![]; N];
    let mut out = vec![btreeset![]; N];
    for _ in 0..M {
        input! {
            a: Usize1,
            b: Usize1,
        }
        out[a].insert(b);
        inn[b].insert(a);
    }

    let mut c = vec![false; N];
    let mut q = VecDeque::new();
    for i in 0..N {
        if out[i].len() == 0 || inn[i].len() == 0 {
            q.push_back(i);
        }
    }
    loop {
        if let Some(i) = q.pop_front() {
            if c[i] {
                continue;
            }
            c[i] = true;
            for &j in out[i].iter() {
                inn[j].remove(&i);
                if inn[j].len() == 0 {
                    q.push_back(j);
                }
            }
            for &j in inn[i].iter() {
                out[j].remove(&i);
                if out[j].len() == 0 {
                    q.push_back(j);
                }
            }
        } else {
            break;
        }
    }

    let mut uf = UnionFind::new(N);
    for a in 0..N {
        if c[a] {
            continue;
        }

        for &b in out[a].iter() {
            if c[b] {
                continue;
            }
            uf.union(a, b);
        }

        for &b in inn[a].iter() {
            if c[b] {
                continue;
            }
            uf.union(a, b);
        }
    }

    let reps = uf.into_labeling();
    let mut conjs = BTreeMap::<usize, usize>::new();
    for &r in reps.iter() {
        *conjs.entry(r).or_default() += 1;
    }

    let mut ans = 0i64;
    for (_, &size) in conjs.iter() {
        let size = size as i64;
        ans += size * (size - 1) / 2;
    }

    println!("{}", ans);
}
