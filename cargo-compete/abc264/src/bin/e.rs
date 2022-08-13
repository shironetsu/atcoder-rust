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
        E: usize,
        edges: [(Usize1, Usize1);E],
        Q: usize,
        X: [Usize1;Q],
    }

    let XX = X.iter().collect::<BTreeSet<_>>();
    let mut rem = (0..E).filter(|&i| !XX.contains(&i)).collect_vec();

    let mut uf = UnionFind::new(N + 1);

    let mut ad = vec![vec![]; N];

    for &i in rem.iter() {
        let (u, v) = edges[i];
        if N <= u {
            continue;
        }

        if v < N {
            uf.union(u, v);
            ad[u].push(v);
            ad[v].push(u);
        } else {
            uf.union(u, N);
        }
    }

    let mut el = vec![false; N];
    for i in 0..N {
        el[i] = uf.equiv(i, N);
    }

    let mut x = (0..N).filter(|&i| el[i]).count();

    let mut ans = vec![x];

    for (m, &i) in X.iter().rev().enumerate() {
        let (u, v) = edges[i];

        if u < N {
            if v < N {
                uf.union(u, v);
                ad[u].push(v);
                ad[v].push(u);
            } else {
                uf.union(u, N);
            }
            if uf.equiv(u, N) || uf.equiv(v, N) {
                let mut todo = VecDeque::new();
                todo.push_back(u);
                if v < N {
                    todo.push_back(v);
                }
                loop {
                    if let Some(s) = todo.pop_front() {
                        if !el[s] {
                            el[s] = true;
                            x += 1;
                            for &t in ad[s].iter() {
                                todo.push_back(t);
                            }
                        }
                    } else {
                        break;
                    }
                }
            }
        }
        if m < X.len() - 1 {
            ans.push(x);
        }
    }

    ans.reverse();
    for a in ans {
        println!("{}", a);
    }
}
