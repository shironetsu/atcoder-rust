#![allow(unused_imports)]
#![allow(non_snake_case)]
use proconio::{input, fastout};
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use std::fmt::Write;
use std::collections::*;
use maplit::*;
use itertools::*;
use superslice::{Ext, Ext2};

const MODULO: i64 = 998244353;

#[fastout]
fn main() {
    input!{
        N: usize,
        P: [Usize1;N],
        Q: [Usize1;N],
    }

    let mut num = vec![(0, 0);N]; //num -> [card, card...]
    for i in 0..N{
        num[P[i]].0 = i;
        num[Q[i]].1 = i;
    }

    let mut uf = petgraph::unionfind::UnionFind::new(N);
    for &(p, q) in num.iter(){
        uf.union(p, q);
    }

    let mut l = uf.into_labeling();
    let mut c = BTreeMap::<usize, usize>::new();
    for &rep in l.iter(){
        *c.entry(rep).or_default() += 1;
    }   

    let mut lucas = vec![0;N+1];
    lucas[0] = 2;
    lucas[1] = 1;
    for i in 2..=N{
        lucas[i] = lucas[i-1]+lucas[i-2];
        lucas[i] %= MODULO;
    }

    let mut ans = 1i64;
    for (_, &m) in c.iter(){
        ans *= lucas[m];
        ans %= MODULO;
    }

    println!("{}", ans);
}