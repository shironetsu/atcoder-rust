#![allow(unused_imports)]
#![allow(non_snake_case)]
use maplit::*;
use permutohedron::LexicalPermutation;
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use proconio::{fastout, input};
use std::collections::*;
use std::fmt::Write;

#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
        AB: [(Usize1, Usize1);M],
        CD: [(Usize1, Usize1);M],
    }
    let mut ad0 = vec![vec![false; N]; N];
    for &(a, b) in AB.iter() {
        ad0[a][b] = true;
        ad0[b][a] = true;
    }
    let mut ad1 = vec![vec![false; N]; N];
    for &(c, d) in CD.iter() {
        ad1[c][d] = true;
        ad1[d][c] = true;
    }

    let mut perm = (0..N).collect::<Vec<_>>();
    loop {
        let mut iso = true;
        for i in 0..N {
            for j in 0..N {
                iso &= ad0[i][j] == ad1[perm[i]][perm[j]];
            }
        }
        if iso {
            println!("Yes");
            return;
        }
        if !perm.next_permutation() {
            break;
        }
    }
    println!("No");
}
