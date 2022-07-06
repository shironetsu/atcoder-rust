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
        K: usize,
        P: i64,
        A: [i64;N],
    }

    if N == 1 {
        let A = sorted(A).collect_vec();
        let ans = A.upper_bound(&P);
        println!("{}", ans);
        return;
    }

    let p = {
        let a = A[..N/2].iter().collect_vec();
        let mut v = vec![vec![]; a.len() + 1];
        for i in 0..1 << a.len() {
            let mut sum = 0i64;
            let mut count = 0;
            for j in 0..a.len() {
                if (i >> j) & 1 == 1 {
                    sum += a[j];
                    count += 1;
                }
            }
            v[count].push(sum);
        }
        for w in v.iter_mut(){
            w.sort();
        }
        v
    };

    let q = {
        let a = A[N/2..].iter().collect_vec();
        let mut v = vec![vec![]; a.len() + 1];
        for i in 0..1 << a.len() {
            let mut sum = 0i64;
            let mut count = 0;
            for j in 0..a.len() {
                if (i >> j) & 1 == 1 {
                    sum += a[j];
                    count += 1;
                }
            }
            v[count].push(sum);
        }
        for w in v.iter_mut(){
            w.sort();
        }
        v
    };

    let mut ans = 0i64;
    for i in 0..p.len() {
        for &x in p[i].iter() {
            if i <= K && K - i < q.len() {
                let j = K - i;
                let y = P - x;
                ans += q[j].upper_bound(&y) as i64;
            }
        }
    }
    println!("{}", ans);
}
