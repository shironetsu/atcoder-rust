#![allow(unused_imports)]
#![allow(non_snake_case)]
use proconio::{input, fastout};
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use std::fmt::{Write, Display};
use std::collections::*;
use maplit::*;
use itertools::*;
use superslice::{Ext, Ext2};

#[fastout]
fn main() {
    input!{
        N: usize,
        ab: [(i64, i64);N],
        cd: [(i64, i64);N],
    }

    if N == 1 {
        println!("Yes");
        return;
    }

    let mut a = vec![0;N];
    let mut b = vec![0;N];
    let mut c = vec![0;N];
    let mut d = vec![0;N];
    for i in 0..N{
        a[i] = ab[i].0;
        b[i] = ab[i].1;
        c[i] = cd[i].0;
        d[i] = cd[i].1;
    }

    let a0 = a[0];
    let b0 = b[0];
    for i in 0..N{
        a[i] -= a0;
        b[i] -= b0;
    }

    let x0 = a[1];
    let y0 = b[1];
    for i in 0..N{
        let mut c = c.clone();
        let mut d = d.clone();
        let c0 = c[i];
        let d0 = d[i];
        for j in 0..N{
            c[j] -= c0;
            d[j] -= d0;
        }
        let mut p = BTreeSet::new();
        for j in 0..N{
            p.insert((c[j], d[j]));
        }
        for j in 0..N{
            let x1 = c[j];
            let y1 = d[j];
            let rr: i64 = x0.pow(2) + y0.pow(2);
            if rr != x1.pow(2) + y1.pow(2) {
                continue;
            }
            let cc = x0*x1+y0*y1;
            let ss = x0*y1-x1*y0;
            for k in 0..N{
                let xx = cc * a[k] - ss * b[k];
                let yy = ss * a[k] + cc * b[k];
                if xx % rr != 0 || yy % rr != 0 {
                    break;
                }
                if !p.contains(&(xx/rr, yy/rr)) {
                    break;
                }
                if k == N - 1 {
                    println!("Yes");
                    return;
                }
            }
        }
    }

    println!("No");
    
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

