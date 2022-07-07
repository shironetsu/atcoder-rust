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
        P: i64,
        Q: i64,
        A: [i64;N],
    }

    // let ans = (0..N).combinations(5).filter(|c|
    //     c.into_iter().map(|&i|A[i]).fold(1, |p, x| (p * x).rem_euclid(P)) == Q
    // ).count();

    let mut ans = 0i64;
    for a in 0..N {
        for b in a + 1..N {
            for c in b + 1..N {
                for d in c + 1..N {
                    for e in d + 1..N {
                        let r = [a, b, c, d, e]
                            .into_iter()
                            .map(|&i| A[i])
                            .fold(1, |p, x| p * x % P);
                        if r == Q {
                            ans += 1;
                        }
                    }
                }
            }
        }
    }

    println!("{}", ans);
}
