#![allow(unused_imports)]
#![allow(non_snake_case)]
use proconio::{input, fastout};
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use std::fmt::Write;
use std::collections::*;
use maplit::*;
use itertools::*;
use superslice::{Ext, Ext2};

#[fastout]
fn main() {
    input!{
        N: usize,
        M: usize,
    }

    let mut a = vec![vec![];M];
    for i in 0..M{
        input!{
            k: usize,
            aa: [Usize1;k],
        }

        a[i] = aa;
    }

    let mut e = vec![BTreeSet::new();N];
    for i in 0..M{
        for j in 0..a[i].len() - 1{
            e[a[i][j+1]].insert(a[i][j]);
        }
    }

    let mut ord = vec![];
    let mut s = BTreeSet::new();
    for i in 0..N{
        if e[i].len() == 0 {
            s.insert(i);
        }
    }
    
}
