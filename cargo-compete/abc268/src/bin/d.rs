#![allow(unused_imports)]
#![allow(non_snake_case)]
use itertools::*;
use maplit::*;
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use proconio::{fastout, input};
use std::collections::*;
use std::fmt::{Display, Write};
use superslice::{Ext, Ext2};

#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
        mut S: [String;N],
        mut T: [String;M],
    }

    if N == 1 {
        if T.iter().find(|x| **x == S[0]).is_some() {
            println!("{}", -1);
        } else {
            if S[0].len() < 3 {
                println!("{}", -1);
            } else {
                println!("{}", S[0]);
            }
        }
        return;
    }

    S.sort();
    let mut inv = BTreeMap::<String, usize>::new();
    for (i, s) in S.iter().enumerate() {
        inv.insert(s.clone(), i);
    }

    let r_max = 16 - S.iter().map(|x| x.len()).sum::<usize>() - (N - 1);

    let mut p = vec![];
    let mut todo = VecDeque::new();
    todo.push_back((0, vec![0; N - 1]));
    loop {
        if let Some((r, v)) = todo.pop_front() {
            if r == N - 1 {
                p.push(v);
                continue;
            }
            let sum = v.iter().sum::<usize>();
            for i in 0..=r_max - sum {
                let mut w = v.clone();
                w[r] = i;
                todo.push_back((r + 1, w));
            }
        } else {
            break;
        }
    }

    let mut m = BTreeMap::<Vec<usize>, BTreeSet<Vec<usize>>>::new();

    for i in 0..M {
        {
            let t = T[i].chars().collect_vec();
            if t[0] == '_' || t[t.len() - 1] == '_' {
                continue;
            }
        }
        let t = T[i].split("_").filter(|&s| s != "").collect::<Vec<_>>();
        {
            let mut t = t.clone();
            t.sort();
            if S != t {
                continue;
            }
        }
        let p = (0..N).map(|i| *inv.get(t[i]).unwrap()).collect_vec();

        let t = T[i].chars().collect_vec();
        let mut l = 0;
        let mut r = 0;
        let mut a = vec![];
        loop {
            if t[l] != '_' {
                l += 1;
            } else {
                r = l;
                while r < t.len() && t[r] == '_' {
                    r += 1;
                }
                a.push(r - l - 1);
                l = r;
            }

            if l >= t.len() || r >= t.len() {
                break;
            }
        }
        if let Some(r) = m.get_mut(&p) {
            r.insert(a);
        } else {
            m.insert(p, btreeset![a]);
        }
    }

    let mut q = (0..N).collect_vec();

    loop {
        if let Some(v) = m.get(&q) {
            for pp in p.iter() {
                if !v.contains(pp) {
                    let ans = q
                        .iter()
                        .map(|&i| S[i].clone())
                        .interleave(pp.iter().map(|x| '_'.to_string().repeat(x + 1)))
                        .join("");
                    println!("{}", ans);
                    return;
                }
            }
        } else {
            let ans = (0..N).map(|i| S[q[i]].clone()).join("_");
            println!("{}", ans);
            return;
        }

        if !q.next_permutation() {
            break;
        }
    }

    println!("{}", -1);
}
//______________________________________________________________________________
//
pub trait Answer {
    fn ans(&self);
    fn ansl(&self);
}

impl<T: Display> Answer for Vec<T> {
    fn ans(&self) {
        let ans = self
            .iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<_>>()
            .join(" ");
        println!("{}", ans);
    }

    fn ansl(&self) {
        let ans = self
            .iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<_>>()
            .join("\n");
        println!("{}", ans);
    }
}
//______________________________________________________________________________
//
#[macro_export]
macro_rules! input_edges {
    ($n: expr, $m: expr, $edges: tt, $ad: tt) => {
        input! {
            $edges: [(Usize1, Usize1); $m],
        }

        let mut $ad = vec![vec![]; $n];
        for &(a, b) in $edges.iter() {
            $ad[a].push(b);
            $ad[b].push(a);
        }
        let $ad = $ad;
    };
}
