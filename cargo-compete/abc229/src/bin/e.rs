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
        edges: [(Usize1, Usize1);M],
    }

    let mut arr = vec![vec![];N];
    for &(a, b) in edges.iter(){
        arr[a.min(b)].push(a.max(b));
    }

    let mut uf = petgraph::unionfind::UnionFind::new(N);
    let mut ans = vec![0;N];
    let mut c = 0;
    for i in (0..N).rev(){
        ans[i] = c;
        c += 1;
        for &j in arr[i].iter(){
            if uf.union(i, j){
                c -= 1;
            }
        }
    }

    for i in 0..N{
        println!("{}", ans[i]);
    }
    
}
